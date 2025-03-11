use crate::gfx_base::{PassBarrierPair, Rect, Viewport};

use super::{
    DynRenderFn, FrameGraph, PassInsertPoint, StringHandle,
    render_target_attachment::{RenderTargetAttachment, RenderTargetAttachmentInfo},
    virtual_resources::VirtualResource,
};

pub struct PassNode {
    pass: Option<Box<DynRenderFn>>,
    //指向读取的资源节点索引
    pub reads: Vec<usize>,
    pub writes: Vec<usize>,
    pub attachments: Vec<RenderTargetAttachment>,
    pub resource_request_array: Vec<usize>,
    pub resource_release_array: Vec<usize>,
    pub name: StringHandle,
    pub ref_count: u32,
    pub next_pass_node_handle: Option<usize>,
    pub head_pass_node_handle: Option<usize>,
    pub distance_to_headad: u16,
    used_render_target_slot_mask: u16,
    pub id: usize,
    pub device_pass_id: usize,
    pub insert_point: PassInsertPoint,
    pub side_effect: bool,
    pub subpass: bool,
    pub subpass_end: bool,
    pub has_cleared_attachment: bool,
    pub clear_action_ignorable: bool,
    custom_viewport: bool,
    viewport: Option<Viewport>,
    scissor: Option<Rect>,
    barriers: Option<PassBarrierPair>,
}

pub struct PassNodeInfo {
    pub ref_count: u32,
    pub subpass: bool,
    pub device_pass_id: usize,
    pub attachments_infos: Vec<RenderTargetAttachmentInfo>,
}

impl PassNode {
    pub fn write(&mut self, graph: &mut FrameGraph, out_handle: usize) -> usize {
        let old_resour_node_info = graph.get_resource_node(out_handle).get_info();
        graph.virtual_resources[old_resour_node_info.virtual_resource_id]
            .get_mut_info()
            .new_version();

        let new_resour_node_handle =
            graph.create_resource_node_with_id(old_resour_node_info.virtual_resource_id);

        graph.resource_nodes[new_resour_node_handle].pass_node_writer_index =
            Some(new_resour_node_handle);

        self.writes.push(new_resour_node_handle);

        new_resour_node_handle
    }

    pub fn read(&mut self, input_handle: usize) {
        if !self.reads.contains(&input_handle) {
            self.reads.push(input_handle);
        }
    }

    pub fn request_transient_resources(&mut self, resources: &mut [Box<dyn VirtualResource>]) {
        for resource_id in self.resource_request_array.iter() {
            let resource = &mut resources[*resource_id];

            if !resource.get_info().imported {
                resource.request();
            }
        }
    }

    pub fn release_transient_resources(&mut self, resources: &mut [Box<dyn VirtualResource>]) {
        for resource_id in self.resource_request_array.iter() {
            let resource = &mut resources[*resource_id];

            if !resource.get_info().imported {
                resource.release();
            }
        }
    }

    pub fn get_info(&self) -> PassNodeInfo {
        PassNodeInfo {
            ref_count: self.ref_count,
            subpass: self.subpass,
            device_pass_id: self.device_pass_id,
            attachments_infos: self
                .attachments
                .iter()
                .map(|attachment| attachment.get_info())
                .collect(),
        }
    }

    pub fn can_merge(&self, graph: &FrameGraph, pass_node: &PassNode) -> bool {
        let attachment_count = self.attachments.len();

        if self.has_cleared_attachment || attachment_count != pass_node.attachments.len() {
            return false;
        }

        for i in 0..attachment_count {
            let attachment_a = &self.attachments[i];
            let attachment_b = &pass_node.attachments[i];

            if attachment_a.desc.usage != attachment_b.desc.usage
                || attachment_a.desc.slot != attachment_b.desc.slot
                || attachment_a.desc.write_mask != attachment_b.desc.write_mask
                || attachment_a.level != attachment_b.level
                || attachment_a.layer != attachment_b.layer
                || attachment_a.index != attachment_b.index
                || graph
                    .get_resource_node(attachment_a.texture_handle.index)
                    .virtual_resource_id
                    != graph
                        .get_resource_node(attachment_b.texture_handle.index)
                        .virtual_resource_id
            {
                return false;
            }
        }

        true
    }

    pub fn get_render_target_attachment(
        &self,
        graph: &FrameGraph,
        virtual_resource_id: usize,
    ) -> Option<&RenderTargetAttachment> {
        self.attachments.iter().find(|attachment| {
            graph
                .get_resource_node(attachment.texture_handle.index)
                .virtual_resource_id
                == virtual_resource_id
        })
    }

    pub fn get_render_target_attachment_index(
        &self,
        graph: &FrameGraph,
        virtual_resource_id: usize,
    ) -> Option<usize> {
        self.attachments
            .iter()
            .enumerate()
            .find(|(_index, attachment)| {
                graph
                    .get_resource_node(attachment.texture_handle.index)
                    .virtual_resource_id
                    == virtual_resource_id
            })
            .map(|(index, _)| index)
    }

    pub fn new(insert_point: PassInsertPoint, name: StringHandle, id: usize) -> Self {
        Self {
            pass: None,
            reads: vec![],
            writes: vec![],
            attachments: vec![],
            resource_request_array: vec![],
            resource_release_array: vec![],
            name,
            ref_count: 0,
            head_pass_node_handle: None,
            next_pass_node_handle: None,
            distance_to_headad: 0,
            used_render_target_slot_mask: 0,
            id,
            device_pass_id: 0,
            insert_point,
            side_effect: false,
            subpass: false,
            subpass_end: false,
            has_cleared_attachment: false,
            clear_action_ignorable: false,
            custom_viewport: false,
            viewport: None,
            scissor: None,
            barriers: None,
        }
    }
}

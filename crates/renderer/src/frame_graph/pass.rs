use crate::gfx_base::{Handle, PassBarrierPair, Rect, Viewport};

use super::{
    DynRenderFn, FrameGraph, PassInsertPoint, StringHandle,
    device_pass::LogicPass,
    render_target_attachment::{RenderTargetAttachment, RenderTargetAttachmentInfo},
    virtual_resources::VirtualResource,
};

use crate::gfx_base::Allocator;

pub struct PassNode {
    ///渲染函数
    pub render_fn: Option<Box<DynRenderFn>>,
    ///读取的资源节点索引
    pub reads: Vec<Handle>,
    ///写入的资源节点索引
    pub writes: Vec<Handle>,
    pub attachments: Vec<RenderTargetAttachment>,
    pub resource_request_array: Vec<Handle>,
    pub resource_release_array: Vec<Handle>,
    pub name: StringHandle,
    pub ref_count: u32,
    //指明device pass中pass node的连接关系
    pub next_pass_node_handle: Option<Handle>,
    pub head_pass_node_handle: Option<Handle>,
    pub distance_to_headad: u16,
    used_render_target_slot_mask: u16,
    pub handle: Handle,
    pub device_pass_handle: Handle,
    pub insert_point: PassInsertPoint,
    pub side_effect: bool,
    pub subpass: bool,
    pub subpass_end: bool,
    pub has_cleared_attachment: bool,
    pub clear_action_ignorable: bool,
    pub custom_viewport: bool,
    pub viewport: Option<Viewport>,
    pub scissor: Option<Rect>,
    pub barriers: PassBarrierPair,
}

pub struct PassNodeInfo {
    pub ref_count: u32,
    pub subpass: bool,
    pub device_pass_handle: Handle,
    pub attachments_infos: Vec<RenderTargetAttachmentInfo>,
    pub handle: Handle,
}

impl PassNode {
    pub fn take(&mut self) -> LogicPass {
        LogicPass {
            custom_viewport: self.custom_viewport,
            viewport: self.viewport.take(),
            scissor: self.scissor.take(),
            render_fn: self.render_fn.take(),
        }
    }

    ///根据旧的资源节点创建新的资源节点，并记录新节点的handle
    pub fn write(&mut self, graph: &mut FrameGraph, out_handle: Handle) -> Handle {
        let old_resour_node_info = graph.get_resource_node(out_handle).to_info();
        graph
            .get_resource_mut(old_resour_node_info.virtual_resource_handle)
            .info_mut()
            .new_version();

        let new_resour_node_handle = graph.create_resource_node_with_virtual_resource_handle(
            old_resour_node_info.virtual_resource_handle,
        );

        graph.resource_nodes[new_resour_node_handle].pass_node_writer_handle =
            Some(new_resour_node_handle);

        self.writes.push(new_resour_node_handle);

        new_resour_node_handle
    }

    pub fn read(&mut self, input_handle: Handle) {
        if !self.reads.contains(&input_handle) {
            self.reads.push(input_handle);
        }
    }

    pub fn request_transient_resources(
        &mut self,
        allocator: &Allocator,
        resources: &mut [Box<dyn VirtualResource>],
    ) {
        for resource_id in self.resource_request_array.iter() {
            let resource = &mut resources[resource_id.index()];

            if !resource.info().imported {
                resource.request(allocator);
            }
        }
    }

    pub fn release_transient_resources(
        &mut self,
        allocator: &Allocator,
        resources: &mut [Box<dyn VirtualResource>],
    ) {
        for resource_id in self.resource_request_array.iter() {
            let resource = &mut resources[resource_id.index()];

            if !resource.info().imported {
                resource.release(allocator);
            }
        }
    }

    pub fn to_info(&self) -> PassNodeInfo {
        PassNodeInfo {
            ref_count: self.ref_count,
            subpass: self.subpass,
            device_pass_handle: self.device_pass_handle,
            attachments_infos: self
                .attachments
                .iter()
                .map(|attachment| attachment.to_info())
                .collect(),
            handle: self.handle,
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
                    .get_resource_node(attachment_a.texture_handle.handle())
                    .virtual_resource_handle
                    != graph
                        .get_resource_node(attachment_b.texture_handle.handle())
                        .virtual_resource_handle
            {
                return false;
            }
        }

        true
    }

    pub fn get_render_target_attachment(
        &self,
        graph: &FrameGraph,
        virtual_resource_handle: Handle,
    ) -> Option<&RenderTargetAttachment> {
        self.attachments.iter().find(|attachment| {
            graph
                .get_resource_node(attachment.texture_handle.handle())
                .virtual_resource_handle
                == virtual_resource_handle
        })
    }

    pub fn get_render_target_attachment_with_virtual_resource_handle(
        &self,
        graph: &FrameGraph,
        virtual_resource_handle: Handle,
    ) -> Option<usize> {
        self.attachments
            .iter()
            .enumerate()
            .find(|(_index, attachment)| {
                graph
                    .get_resource_node(attachment.texture_handle.handle())
                    .virtual_resource_handle
                    == virtual_resource_handle
            })
            .map(|(index, _)| index)
    }

    pub fn new(insert_point: PassInsertPoint, name: StringHandle, handle: Handle) -> Self {
        Self {
            render_fn: None,
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
            handle,
            device_pass_handle: Handle::new(0),
            insert_point,
            side_effect: false,
            subpass: false,
            subpass_end: false,
            has_cleared_attachment: false,
            clear_action_ignorable: false,
            custom_viewport: false,
            viewport: None,
            scissor: None,
            barriers: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        frame_graph::{FrameGraph, render_target_attachment::RenderTargetAttachment},
        gfx_base::{Allocator, Handle, TextureDescriptor, test::TestResourceCreator},
        utils::IndexHandle,
    };

    use super::PassNode;

    #[test]
    fn test_write() {
        let mut graph = FrameGraph::new(Allocator::new(TestResourceCreator {}));
        let handle = graph.create(
            IndexHandle::new("2".to_string(), 0),
            TextureDescriptor::default(),
        );

        let mut pass_node = PassNode::new(0, IndexHandle::new("a".to_string(), 1), Handle::new(0));

        let new_resource_handle = pass_node.write(&mut graph, handle.handle());

        let new_resource_node_info = graph.get_resource_node(new_resource_handle).to_info();
        let version = graph
            .get_resource(new_resource_node_info.virtual_resource_handle)
            .info()
            .version;

        assert_eq!(version, 1);
        assert_eq!(new_resource_node_info.handle, Handle::new(1));
    }

    #[test]
    fn test_can_merge() {
        let mut graph = FrameGraph::new(Allocator::new(TestResourceCreator {}));

        let mut pass_node_a =
            PassNode::new(0, IndexHandle::new("a".to_string(), 0), Handle::new(0));
        let mut pass_node_b =
            PassNode::new(0, IndexHandle::new("b".to_string(), 1), Handle::new(1));

        pass_node_a.has_cleared_attachment = true;
        assert!(!pass_node_a.can_merge(&graph, &pass_node_b));

        let handle = graph.create(
            IndexHandle::new("c".to_string(), 2),
            TextureDescriptor::default(),
        );

        pass_node_a.has_cleared_attachment = false;
        pass_node_a.attachments.push(RenderTargetAttachment {
            texture_handle: handle.clone(),
            ..Default::default()
        });

        pass_node_b.attachments.push(RenderTargetAttachment {
            texture_handle: handle,
            ..Default::default()
        });

        assert!(pass_node_a.can_merge(&graph, &pass_node_b));
    }
}

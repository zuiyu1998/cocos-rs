use std::{cmp::Ordering, rc::Rc};

use crate::gfx_base::{
    AnyFGResource, Handle, INVALID_BINDING, PassBarrierPair, Rect, StoreOp, SubpassInfo, Viewport,
};

use super::{
    DynRenderFn, FrameGraph,
    render_target_attachment::{
        RenderTargetAttachment, RenderTargetAttachmentInfo, RenderTargetAttachmentUsage,
    },
    resource_table::ResourceTable,
};

#[derive(Default)]
pub struct DevicePass {
    barriers: Vec<PassBarrierPair>,
    used_render_target_slot_mask: u16,
    subpasses: Vec<Subpass>,
    attachments: Vec<Attachment>,
    resource_table: ResourceTable,
}

#[derive(Default)]
pub struct LogicPass {
    pub custom_viewport: bool,
    pub viewport: Option<Viewport>,
    pub scissor: Option<Rect>,
    pub render_fn: Option<Box<DynRenderFn>>,
}

pub struct Attachment {
    attachment: RenderTargetAttachment,
    render_target: Rc<AnyFGResource>,
}

#[derive(Default)]
pub struct Subpass {
    desc: SubpassInfo,
    barrier_id: u32,
    logic_passes: Vec<LogicPass>,
}

impl DevicePass {
    pub fn new(graph: &mut FrameGraph, pass_node_handles: Vec<Handle>) -> Self {
        let mut device_pass = DevicePass::default();
        let mut attachments = vec![];

        let mut index = 0;

        for pass_node_handle in pass_node_handles.iter() {
            device_pass.append(graph, *pass_node_handle, &mut attachments);

            let barriers = graph.get_pass_node(*pass_node_handle).barriers.clone();
            device_pass.barriers.push(barriers);

            index += 1;

            device_pass.subpasses.last_mut().unwrap().barrier_id = index;
        }

        // todo
        // auto *device = gfx::Device::getInstance();
        // _enableAutoBarrier: auto barrier in framegraph
        // barrierDeduce: deduce barrier gfx internally
        // to avoid redundant instructions, either inside or outside
        // device->enableAutoBarrier(!gfx::ENABLE_GRAPH_AUTO_BARRIER);

        let mut depth_index = INVALID_BINDING;
        let mut depth_new_index = INVALID_BINDING;

        for i in 0..attachments.len() {
            if attachments[i].desc.usage != RenderTargetAttachmentUsage::Color {
                assert!(depth_index == INVALID_BINDING);
                depth_index = i as u32;
                depth_new_index = (attachments.len() - 1) as u32;
            }
        }

        attachments.sort();

        for subpass in device_pass.subpasses.iter_mut() {
            let info = &mut subpass.desc;

            for id in info.inputs.iter_mut() {
                match (*id).cmp(&depth_index) {
                    Ordering::Equal => {
                        *id = depth_new_index;
                    }
                    Ordering::Greater => {
                        *id -= 1;
                    }
                    _ => {}
                }
            }

            for id in info.resolves.iter_mut() {
                match (*id).cmp(&depth_index) {
                    Ordering::Equal => {
                        *id = depth_new_index;
                    }
                    Ordering::Greater => {
                        *id -= 1;
                    }
                    _ => {}
                }
            }

            for id in info.preserves.iter_mut() {
                match (*id).cmp(&depth_index) {
                    Ordering::Equal => {
                        *id = depth_new_index;
                    }
                    Ordering::Greater => {
                        *id -= 1;
                    }
                    _ => {}
                }
            }
        }

        //todo renderTargets

        for attachment in attachments.into_iter() {
            let resource_node = graph.get_resource_node(attachment.to_info().texture_handle);
            let resource = graph
                .get_resource(resource_node.virtual_resource_handle)
                .get_any_resource()
                .unwrap();

            assert!(resource.is_texture());

            let attachment = Attachment {
                attachment,
                render_target: resource,
            };
            device_pass.attachments.push(attachment);
        }

        for pass_node_index in pass_node_handles.iter() {
            device_pass.resource_table.extra(graph, *pass_node_index);
        }

        device_pass
    }

    //填充Subpass
    fn append(
        &mut self,
        graph: &mut FrameGraph,
        pass_node_handle: Handle,
        attachments: &mut Vec<RenderTargetAttachment>,
    ) {
        let mut sub_pass = Subpass::default();

        let mut pass_node_handle = pass_node_handle;

        loop {
            let logic_pass = graph.take_pass_node(pass_node_handle);
            sub_pass.logic_passes.push(logic_pass);

            let reads = graph.get_pass_node(pass_node_handle).reads.clone();

            let attachment_infos: Vec<RenderTargetAttachmentInfo> = graph
                .get_pass_node(pass_node_handle)
                .attachments
                .iter()
                .map(|attachment| attachment.to_info())
                .collect();

            for attachment_info in attachment_infos.iter() {
                self.append_with_attachment(
                    graph,
                    attachment_info,
                    attachments,
                    &mut sub_pass.desc,
                );
            }

            for handle in reads.iter() {
                let (end, index) = {
                    if let Some(index) = attachments
                        .iter()
                        .enumerate()
                        .find(|(_index, attachment)| attachment.to_info().texture_handle == *handle)
                        .map(|(index, _)| index)
                    {
                        if index + 1 == attachments.len() {
                            (true, Some(index))
                        } else {
                            (false, Some(index))
                        }
                    } else {
                        (false, None)
                    }
                };

                if !end {
                    let input = index.unwrap();

                    let end = {
                        if let Some(index) = sub_pass
                            .desc
                            .inputs
                            .iter()
                            .enumerate()
                            .find(|(_index, desc_input)| **desc_input == input as u32)
                            .map(|(index, _)| index)
                        {
                            index + 1 == attachments.len()
                        } else {
                            false
                        }
                    };

                    if end {
                        sub_pass.desc.inputs.push(input as u32);
                    }
                }
            }

            let next = graph
                .get_pass_node(pass_node_handle)
                .next_pass_node_handle
                .is_some();

            if !next {
                break;
            } else {
                pass_node_handle = graph
                    .get_pass_node(pass_node_handle)
                    .next_pass_node_handle
                    .unwrap();
            }
        }

        self.subpasses.push(sub_pass);
    }

    fn append_with_attachment(
        &mut self,
        graph: &mut FrameGraph,
        attachment_info: &RenderTargetAttachmentInfo,
        attachments: &mut Vec<RenderTargetAttachment>,
        subpass: &mut SubpassInfo,
    ) {
        let usage = attachment_info.usage;
        let mut slot = attachment_info.slot as u32;

        if usage == RenderTargetAttachmentUsage::Color {
            slot = if subpass.colors.len() > attachment_info.slot as usize {
                subpass.colors[attachment_info.slot as usize]
            } else {
                INVALID_BINDING
            }
        }

        let (last_attachment, last_attachment_info, last_attachment_index) = {
            if let Some((attachment_index, attachment)) =
                attachments.iter().enumerate().find(|(_index, attachment)| {
                    let info = attachment.to_info();
                    info.usage == usage && info.slot as u32 == slot
                })
            {
                (
                    attachment_index + 1 == attachments.len(),
                    Some(attachment.to_info()),
                    Some(attachment_index),
                )
            } else {
                (false, None, None)
            }
        };

        let out_attachment_info: RenderTargetAttachmentInfo;

        if last_attachment {
            let mut attachment = RenderTargetAttachment::default();

            //设置attachment的slot
            if usage == RenderTargetAttachmentUsage::Color {
                for i in 0..RenderTargetAttachment::DEPTH_STENCIL_SLOT_START {
                    if (self.used_render_target_slot_mask & (1 << i)) == 0 {
                        attachment.desc.slot = i;
                        self.used_render_target_slot_mask |= 1 << i;
                    }
                }
            } else {
                assert!((self.used_render_target_slot_mask & (1 << attachment.desc.slot)) == 0);
                self.used_render_target_slot_mask |= 1 << attachment.desc.slot;
            }

            out_attachment_info = attachment.to_info();

            attachments.push(attachment);
        } else {
            let last_attachment_info = last_attachment_info.unwrap();
            let last_attachment_index = last_attachment_index.unwrap();

            let resource_node_a = graph.get_resource_node(last_attachment_info.texture_handle);
            let resource_node_b = graph.get_resource_node(attachment_info.texture_handle);

            if resource_node_a.virtual_resource_handle == resource_node_b.virtual_resource_handle {
                if attachment_info.store_op != StoreOp::Discard {
                    attachments[last_attachment_index].store_op = attachment_info.store_op;
                    attachments[last_attachment_index].desc.end_accesses =
                        attachment_info.end_accesses;
                }

                out_attachment_info = attachments[last_attachment_index].to_info();
            } else {
                let mut attachment = RenderTargetAttachment::default();
                for i in 0..RenderTargetAttachment::DEPTH_STENCIL_SLOT_START {
                    if (self.used_render_target_slot_mask & (1 << i)) == 0 {
                        attachment.desc.slot = i;
                        self.used_render_target_slot_mask |= 1 << i;
                    }
                }
                out_attachment_info = attachment.to_info();
                attachments.push(attachment);
            }
        }

        if usage == RenderTargetAttachmentUsage::Color {
            let end = {
                if let Some(index) = subpass
                    .colors
                    .iter()
                    .enumerate()
                    .find(|(_index, color)| **color == out_attachment_info.slot as u32)
                    .map(|(index, _)| index)
                {
                    index + 1 == attachments.len()
                } else {
                    false
                }
            };

            if end {
                subpass.colors.push(out_attachment_info.slot as u32);
            }
        } else {
            subpass.depth_stencil = out_attachment_info.slot as u32;
        }
    }

    pub fn execute(&mut self) {}
}

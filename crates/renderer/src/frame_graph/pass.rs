use crate::gfx_base::{PassBarrierPair, Rect, Viewport};

use super::{
    DynRenderFn, Handle, Id, PassInsertPoint, StringHandle,
    render_target_attachment::RenderTargetAttachment,
};

pub struct PassNode {
    pass: Option<Box<DynRenderFn>>,
    reads: Vec<Handle>,
    writes: Vec<Handle>,
    attachments: Vec<RenderTargetAttachment>,
    resource_request_array: Vec<Handle>,
    resource_release_array: Vec<Handle>,
    name: StringHandle,
    ref_countt: u32,
    next_pass_node_handle: Option<Handle>,
    head_pass_node_handle: Option<Handle>,
    distance_to_headad: u16,
    used_render_target_slot_mask: u16,
    id: Id,
    device_pass_id: Id,
    insert_point: PassInsertPoint,
    side_effect: bool,
    subpass: bool,
    subpass_end: bool,
    has_cleared_attachment: bool,
    clear_action_ignorable: bool,
    custom_viewport: bool,
    viewport: Option<Viewport>,
    scissor: Option<Rect>,
    barriers: Option<PassBarrierPair>,
}

impl PassNode {
    pub fn new(
        insert_point: PassInsertPoint,
        name: StringHandle,
        id: Id,
        pass: Box<DynRenderFn>,
    ) -> Self {
        Self {
            pass: Some(pass),
            reads: vec![],
            writes: vec![],
            attachments: vec![],
            resource_request_array: vec![],
            resource_release_array: vec![],
            name,
            ref_countt: 0,
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

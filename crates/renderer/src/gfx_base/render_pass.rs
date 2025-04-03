use crate::{define_atomic_id, define_gfx_type};
use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::frame_graph::RenderContext;

define_atomic_id!(RenderPassId);

#[derive(Default, Clone, Debug)]
pub struct RenderPassInfo {}

impl RenderPassInfo {
    pub fn new() -> Self {
        RenderPassInfo::default()
    }
}

pub trait RenderPassTrait: 'static + Debug {
    fn do_init(&mut self, render_context: &RenderContext);
}

pub trait ErasedRenderPassTrait: 'static + Debug + Downcast {
    fn do_init(&mut self, render_context: &RenderContext);
}

impl<T: RenderPassTrait> ErasedRenderPassTrait for T {
    fn do_init(&mut self, render_context: &RenderContext) {
        <T as RenderPassTrait>::do_init(self, render_context);
    }
}

define_gfx_type!(
    RenderPass,
    RenderPassId,
    RenderPassTrait,
    ErasedRenderPassTrait
);

impl RenderPass {
    pub fn do_init(&mut self, render_context: &RenderContext) {
        self.value.do_init(render_context);
    }
}

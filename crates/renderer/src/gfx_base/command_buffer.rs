use std::fmt::Debug;

use downcast_rs::Downcast;

use crate::{define_atomic_id, define_gfx_type};

use super::RenderPassInfo;

define_atomic_id!(CommandBufferId);

pub trait CommandBufferTrait: 'static + Sync + Send + Debug {
    fn begin_render_pass(&mut self, render_pass_info: RenderPassInfo);

    fn end_render_pass(&mut self);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Debug + Downcast {
    fn begin_render_pass(&mut self, render_pass_info: RenderPassInfo);

    fn end_render_pass(&mut self);
}

impl<T> ErasedCommandBufferTrait for T
where
    T: CommandBufferTrait,
{
    fn begin_render_pass(&mut self, render_pass_info: RenderPassInfo) {
        <T as CommandBufferTrait>::begin_render_pass(self, render_pass_info);
    }

    fn end_render_pass(&mut self) {
        <T as CommandBufferTrait>::end_render_pass(self);
    }
}

define_gfx_type!(
    CommandBuffer,
    CommandBufferId,
    CommandBufferTrait,
    ErasedCommandBufferTrait
);

impl CommandBuffer {
    pub fn begin_render_pass(&mut self, render_pass_info: RenderPassInfo) {
        self.value.begin_render_pass(render_pass_info);
    }

    pub fn end_render_pass(&mut self) {
        self.value.end_render_pass();
    }
}

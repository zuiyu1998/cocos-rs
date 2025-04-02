use std::fmt::Debug;

use downcast_rs::{Downcast, impl_downcast};

pub trait RenderPassTrait: 'static + Sync + Send + Debug + Clone {
    fn new(info: RenderPassInfo) -> Self;
}

pub trait ErasedRenderPassTrait: 'static + Sync + Send + Debug + Downcast {
    fn clone_value(&self) -> Box<dyn ErasedRenderPassTrait>;
}

impl<T> ErasedRenderPassTrait for T
where
    T: RenderPassTrait,
{
    fn clone_value(&self) -> Box<dyn ErasedRenderPassTrait> {
        Box::new(self.clone())
    }
}

impl_downcast!(ErasedRenderPassTrait);

pub struct RenderPassInfo {}

impl RenderPassInfo {
    pub fn new() -> Self {
        RenderPassInfo {}
    }
}

#[derive(Debug)]
pub struct RenderPass {
    value: Box<dyn ErasedRenderPassTrait>,
}

impl Clone for RenderPass {
    fn clone(&self) -> Self {
        RenderPass {
            value: self.value.clone_value(),
        }
    }
}

impl RenderPass {
    pub fn from_info<T: RenderPassTrait>(info: RenderPassInfo) -> Self {
        RenderPass::new(T::new(info))
    }

    pub fn new<T: RenderPassTrait>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }
}

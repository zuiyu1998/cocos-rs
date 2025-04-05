use crate::frame_graph::{GpuRead, ResourceNodeRef};

use super::SwapChain;

#[derive(Clone, Debug)]
pub enum ColorAttachment {
    SwapChain(ResourceNodeRef<SwapChain, GpuRead>),
}

impl ColorAttachment {
    pub fn swap_chain(handle: ResourceNodeRef<SwapChain, GpuRead>) -> Self {
        ColorAttachment::SwapChain(handle)
    }
}

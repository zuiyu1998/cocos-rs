use crate::frame_graph::{GpuRead, ResourceRef};

use super::SwapChain;

#[derive(Clone, Debug)]
pub enum ColorAttachment {
    SwapChain(ResourceRef<SwapChain, GpuRead>),
}

impl ColorAttachment {
    pub fn swap_chain(handle: ResourceRef<SwapChain, GpuRead>) -> Self {
        ColorAttachment::SwapChain(handle)
    }
}

use super::{ColorAttachment, DepthStencilAttachment, SubpassDependency, SubpassInfo};
use std::fmt::Debug;

pub trait RenderPassTrait: 'static + Debug {}

#[derive(Debug)]
pub struct RenderPass(Box<dyn RenderPassTrait>);

impl PartialEq for RenderPass {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl Eq for RenderPass {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct RenderPassDescriptor {
    pub color_attachments: Vec<ColorAttachment>,
    pub depth_stencil_attachment: DepthStencilAttachment,
    pub depth_stencil_resolve_attachment: DepthStencilAttachment,
    pub subpasses: Vec<SubpassInfo>,
    pub dependencies: Vec<SubpassDependency>,
}

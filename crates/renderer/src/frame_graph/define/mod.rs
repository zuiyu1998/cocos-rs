mod allocator;
mod texture;

use crate::RendererError;
use std::{hash::Hash, sync::Arc};

pub use allocator::*;
pub use texture::*;

pub enum AnyFGResource {
    Texture(Texture),
}

impl AnyFGResource {
    pub fn is_texture(&self) -> bool {
        matches!(self, AnyFGResource::Texture(_))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureDescriptor),
}

pub struct AnyResource {
    pub desc: AnyFGResourceDescriptor,
    pub resource: Arc<AnyFGResource>,
}

pub type DynRenderFn = dyn FnOnce() -> Result<(), RendererError>;

impl AnyResource {
    pub fn new(desc: AnyFGResourceDescriptor, resource: Arc<AnyFGResource>) -> Self {
        AnyResource { desc, resource }
    }
}
///资源
pub trait FGResource: 'static + Sized {
    type Descriptor: FGResourceDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self;

    fn create_transient(allocator: &Allocator, desc: &Self::Descriptor) -> AnyResource {
        let desc: AnyFGResourceDescriptor = desc.clone().into();
        allocator.alloc(&desc)
    }

    fn destroy_transient(allocator: &Allocator, resource: AnyResource) {
        allocator.free(resource);
    }
}

///资源描述符
pub trait FGResourceDescriptor:
    'static + Sized + Clone + Hash + Eq + Into<AnyFGResourceDescriptor>
{
    type Resource: FGResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}

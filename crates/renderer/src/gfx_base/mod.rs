mod allocator;
mod buffer;
mod common;
mod handle;
mod texture;

pub use allocator::*;
pub use buffer::*;
pub use common::*;
pub use handle::*;
pub use texture::*;

use std::{hash::Hash, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
pub enum AnyFGResource {
    Texture(Texture),
    Buffer(Buffer),
}

impl AnyFGResource {
    pub fn is_texture(&self) -> bool {
        matches!(self, AnyFGResource::Texture(_))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureDescriptor),
    Buffer(BufferDescriptor),
}

pub struct AnyResource {
    pub desc: AnyFGResourceDescriptor,
    pub resource: Rc<AnyFGResource>,
}

impl AnyResource {
    pub fn new(desc: AnyFGResourceDescriptor, resource: Rc<AnyFGResource>) -> Self {
        AnyResource { desc, resource }
    }
}
///资源
pub trait FGResource: 'static + Sized {
    type Descriptor: FGResourceDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self;
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

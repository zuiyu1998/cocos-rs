mod allocator;
mod texture;

use std::{hash::Hash, sync::Arc};

pub use texture::*;

pub trait FrameResourceAllocator {
    type Descriptor: FrameResourceDescriptor;
    type Resource: FrameResource;

    fn alloc(&self, desc: &Self::Descriptor) -> ResourceRef<Self::Resource, Self::Descriptor>;
    fn free(&self, resource: ResourceRef<Self::Resource, Self::Descriptor>);

    fn get_instance() -> Self;
}

pub struct ResourceRef<Resource: FrameResource, Descriptor: FrameResourceDescriptor> {
    pub desc: Descriptor,
    pub resource: Arc<Resource>,
}

///资源
pub trait FrameResource: 'static + Sized {
    type Descriptor: FrameResourceDescriptor;
    type Allocator: FrameResourceAllocator<Resource = Self, Descriptor = Self::Descriptor>;

    fn create_transient(desc: &Self::Descriptor) -> ResourceRef<Self, Self::Descriptor> {
        Self::Allocator::get_instance().alloc(desc)
    }
    fn destroy_transient(resource: ResourceRef<Self, Self::Descriptor>) {
        Self::Allocator::get_instance().free(resource)
    }
}

///资源描述符
pub trait FrameResourceDescriptor: 'static + Sized + Clone + Hash + Eq {
    type Resource: FrameResource;
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

use std::sync::OnceLock;

use super::{
    FrameResource, FrameResourceAllocator, FrameResourceDescriptor, ResourceRef,
    allocator::{Allocator, ResourceCreator},
};

static TEXTURE_ALLOCATOR: OnceLock<TextureAllocator> = OnceLock::new();

pub struct TextureCreator {}

impl ResourceCreator for TextureCreator {
    type Descriptor = TextureDescriptor;
    type Resource = Texture;

    fn create(&self, desc: &Self::Descriptor) -> Self::Resource {
        Texture { desc: desc.clone() }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Texture {
    desc: TextureDescriptor,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureDescriptor {
    pub width: u32,
}

#[derive(Clone)]
pub struct TextureAllocator(Allocator<Texture, TextureDescriptor, TextureCreator>);

impl TextureAllocator {
    pub fn initialization(creator: TextureCreator) {
        TEXTURE_ALLOCATOR.get_or_init(|| TextureAllocator(Allocator::new(creator)));
    }
}

impl FrameResourceAllocator for TextureAllocator {
    type Descriptor = TextureDescriptor;
    type Resource = Texture;

    fn alloc(&self, desc: &Self::Descriptor) -> ResourceRef<Self::Resource, Self::Descriptor> {
        self.0.alloc(desc)
    }

    fn get_instance() -> Self {
        TEXTURE_ALLOCATOR.get().cloned().unwrap()
    }

    fn free(&self, resource: ResourceRef<Self::Resource, Self::Descriptor>) {
        self.0.free(resource)
    }
}

impl FrameResource for Texture {
    type Descriptor = TextureDescriptor;
    type Allocator = TextureAllocator;

    fn create_transient(desc: &Self::Descriptor) -> ResourceRef<Texture, TextureDescriptor> {
        TextureAllocator::get_instance().alloc(desc)
    }

    fn destroy_transient(resource: ResourceRef<Texture, TextureDescriptor>) {
        TextureAllocator::get_instance().free(resource)
    }
}

impl FrameResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

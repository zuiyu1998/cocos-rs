use super::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Buffer {
    desc: BufferDescriptor,
}

impl Buffer {
    pub fn new(desc: BufferDescriptor) -> Self {
        Buffer { desc }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BufferDescriptor {
    pub width: u32,
}

impl FGResource for Buffer {
    type Descriptor = BufferDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::Buffer(texture) => texture,
            _ => {
                unimplemented!()
            }
        }
    }
}

impl From<BufferDescriptor> for AnyFGResourceDescriptor {
    fn from(value: BufferDescriptor) -> Self {
        AnyFGResourceDescriptor::Buffer(value)
    }
}

impl FGResourceDescriptor for BufferDescriptor {
    type Resource = Buffer;
}

use super::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Texture {
    desc: TextureDescriptor,
}

impl Texture {
    pub fn new(desc: TextureDescriptor) -> Self {
        Texture { desc }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TextureDescriptor {
    pub width: u32,
}

impl FGResource for Texture {
    type Descriptor = TextureDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::Texture(texture) => texture,
        }
    }
}

impl From<TextureDescriptor> for AnyFGResourceDescriptor {
    fn from(value: TextureDescriptor) -> Self {
        AnyFGResourceDescriptor::Texture(value)
    }
}

impl FGResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

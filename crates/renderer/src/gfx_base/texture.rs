use std::fmt::Debug;

use super::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

pub trait BaseTexture: 'static + Debug {
    fn test(&self);
}

#[derive(Debug)]
pub struct Texture(Box<dyn BaseTexture>);

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl Eq for Texture {}

impl Texture {
    pub fn new<T: BaseTexture>(base: T) -> Self {
        Texture(Box::new(base))
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
            _ => {
                unimplemented!()
            }
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

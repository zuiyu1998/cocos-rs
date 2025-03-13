use std::fmt::Debug;

use super::{
    AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor, Format, SampleCount,
    TextureFlags, TextureType, TextureUsage,
};

pub trait TextureTrait: 'static + Debug {
    fn test(&self);
}

#[derive(Debug)]
pub struct Texture(Box<dyn TextureTrait>);

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.0, &*other.0)
    }
}

impl Eq for Texture {}

impl Texture {
    pub fn new<T: TextureTrait>(base: T) -> Self {
        Texture(Box::new(base))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureDescriptor {
    pub texture_type: TextureType,
    pub texture_usage: TextureUsage,
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub texture_flags: TextureFlags,
    pub layer_count: u32,
    pub level_count: u32,
    pub sample_count: SampleCount,
    pub depth: u32,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        TextureDescriptor {
            texture_type: Default::default(),
            texture_usage: TextureUsage::NONE,
            format: Format::Unknown,
            width: 0,
            height: 0,
            texture_flags: TextureFlags::NONE,
            layer_count: 1,
            level_count: 1,
            sample_count: SampleCount::X1,
            depth: 1,
        }
    }
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

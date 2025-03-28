use crate::{Texture, TextureDescriptor};

use super::{AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

impl FGResource for Texture {
    type Descriptor = TextureDescriptor;
}

impl FGResourceDescriptor for TextureDescriptor {
    type Resource = Texture;
}

impl From<TextureDescriptor> for AnyFGResourceDescriptor {
    fn from(value: TextureDescriptor) -> Self {
        AnyFGResourceDescriptor::Texture(value)
    }
}

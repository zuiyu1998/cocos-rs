use crate::{Texture, TextureInfo};

use super::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

impl FGResource for Texture {
    type Descriptor = TextureInfo;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::ImportedTexture(res) => res,
            AnyFGResource::OwnedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}

impl FGResourceDescriptor for TextureInfo {
    type Resource = Texture;
}

impl From<TextureInfo> for AnyFGResourceDescriptor {
    fn from(value: TextureInfo) -> Self {
        AnyFGResourceDescriptor::Texture(value)
    }
}

use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::{define_atomic_id, define_gfx_frame_graph_type};

define_atomic_id!(TextureId);

pub trait TextureTrait: 'static + Debug {}
pub trait ErasedTextureTrait: 'static + Downcast + Debug {}

impl<T: TextureTrait> ErasedTextureTrait for T {}

define_gfx_frame_graph_type!(
    Texture,
    TextureId,
    TextureTrait,
    ErasedTextureTrait,
    TextureInfo
);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureInfo;

use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::{define_atomic_id, define_gfx_type};

define_atomic_id!(TextureViewId);

pub trait TextureViewTrait: 'static + Debug {}
pub trait ErasedTextureViewTrait: 'static + Downcast + Debug {}

impl<T: TextureViewTrait> ErasedTextureViewTrait for T {}

define_gfx_type!(
    TextureView,
    TextureViewId,
    TextureViewTrait,
    ErasedTextureViewTrait
);

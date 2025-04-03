use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::{define_atomic_id, define_gfx_type};

use super::TextureView;

define_atomic_id!(SwapChainId);

pub trait SwapChainTrait: 'static + Debug {
    fn get_texture_view(&self) -> TextureView;

    fn present(&mut self);
}

pub trait ErasedSwapChainTrait: 'static + Downcast + Debug {
    fn get_texture_view(&self) -> TextureView;

    fn present(&mut self);
}

impl<T: SwapChainTrait> ErasedSwapChainTrait for T {
    fn get_texture_view(&self) -> TextureView {
        <T as SwapChainTrait>::get_texture_view(&self)
    }
    fn present(&mut self) {
        <T as SwapChainTrait>::present(self)
    }
}

define_gfx_type!(SwapChain, SwapChainId, SwapChainTrait, ErasedSwapChainTrait);

impl SwapChain {
    pub fn get_texture_view(&self) -> TextureView {
        self.value.get_texture_view()
    }

    pub fn present(&mut self) {
        self.value.present();
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SwapChainInfo {}

use crate::TextureViewTrait;

#[derive(Debug)]
pub struct WgpuTextureView {
    pub texture_view: wgpu::TextureView,
}

impl TextureViewTrait for WgpuTextureView {}

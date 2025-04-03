use wgpu::SurfaceTexture;

use crate::{SwapChainTrait, TextureView, WgpuTextureView};

#[derive(Debug)]
pub struct WgpuSwapChain {
    surface_texture: Option<SurfaceTexture>,
    surface_format: wgpu::TextureFormat,
}

impl SwapChainTrait for WgpuSwapChain {
    fn present(&mut self) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
    }

    fn get_texture_view(&self) -> TextureView {
        let texture_view = self.surface_texture.as_ref().unwrap().texture.create_view(
            &wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            },
        );

        TextureView::new(WgpuTextureView { texture_view })
    }
}

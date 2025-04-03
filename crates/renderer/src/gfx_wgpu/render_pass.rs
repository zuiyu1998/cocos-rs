use crate::{
    frame_graph::RenderContext,
    gfx_base::{RenderPassInfo, RenderPassTrait, TextureView, ColorAttachment},
};

#[derive(Debug)]
pub struct WgpuRenderPass {
    desc: RenderPassInfo,
    pub texture_views: Option<Vec<TextureView>>,
}

impl WgpuRenderPass {
    pub fn new(desc: RenderPassInfo) -> Self {
        WgpuRenderPass {
            desc,
            texture_views: None,
        }
    }
}

impl RenderPassTrait for WgpuRenderPass {
    fn do_init(&mut self, render_context: &RenderContext) {
        let mut texture_views = vec![];

        for color_attachment in self.desc.color_attachments.iter() {
            match color_attachment {
                ColorAttachment::SwapChain(handle) => {
                    if let Some(resource) = render_context.get_resource(&handle) {
                        texture_views.push(resource.get_texture_view());
                    }
                }
            }
        }

        self.texture_views = Some(texture_views);
    }
}

use crate::{
    gfx_base::{CommandBufferTrait, Device, RenderPass},
    gfx_wgpu::WgpuDevice,
};

#[derive(Debug, Default)]
pub struct WgpuCommandBuffer {
    encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'static>>,
    pub command_buffer: Option<wgpu::CommandBuffer>,
}

impl CommandBufferTrait for WgpuCommandBuffer {
    fn begin_render_pass(&mut self, device: &Device, _render_pass: RenderPass) {
        let device = device.downcast_ref::<WgpuDevice>().unwrap();

        let mut encoder = device.device.create_command_encoder(&Default::default());
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &vec![],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let render_pass = render_pass.forget_lifetime();

        self.encoder = Some(encoder);
        self.render_pass = Some(render_pass);
    }

    fn end_render_pass(&mut self) {
        let render_pass = self.render_pass.take().unwrap();
        let encoder = self.encoder.take().unwrap();

        drop(render_pass);

        let command_buffer = encoder.finish();

        self.command_buffer = Some(command_buffer);
    }
}

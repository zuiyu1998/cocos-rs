use crate::DeviceTrait;

#[derive(Debug)]
pub struct WgpuDevice {
    pub device: wgpu::Device,
}

impl DeviceTrait for WgpuDevice {
    fn create_command_buffer(&self) -> crate::CommandBuffer {
        todo!()
    }

    fn submit(&self, _command_buffers: Vec<crate::CommandBuffer>) {
        todo!()
    }
}

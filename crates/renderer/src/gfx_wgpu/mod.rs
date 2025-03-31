use crate::DeviceTrait;

#[derive(Debug)]
pub struct WgpuDevice {
    pub device: wgpu::Device,
}

impl DeviceTrait for WgpuDevice {}

use crate::{define_atomic_id, define_gfx_type};
use std::fmt::Debug;

use downcast_rs::Downcast;

use super::command_buffer::CommandBuffer;

define_atomic_id!(DeviceId);

pub trait DeviceTrait: 'static + Sync + Send + Debug {
    fn create_command_buffer(&self) -> CommandBuffer;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);
}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Debug + Downcast {
    fn create_command_buffer(&self) -> CommandBuffer;
}

impl<T: DeviceTrait> ErasedDeviceTrait for T {
    fn create_command_buffer(&self) -> CommandBuffer {
        <T as DeviceTrait>::create_command_buffer(&self)
    }
}

define_gfx_type!(Device, DeviceId, DeviceTrait, ErasedDeviceTrait);

impl Device {
    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }
}

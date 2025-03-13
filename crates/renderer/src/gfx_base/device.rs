use super::{Rect, Viewport};

pub struct Device {}

impl Device {
    pub fn get_command_buffer_mut(&self) -> &mut CommandBuffer {
        todo!()
    }
}

pub struct CommandBuffer {}

impl CommandBuffer {
    pub fn set_viewport(&mut self, _viewport: &Viewport) {
        //todo
    }
    pub fn set_scissor(&mut self, _scissor: &Rect) {
        //todo
    }
}

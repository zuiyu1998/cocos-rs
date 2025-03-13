use super::{
    AnyFGResource, AnyFGResourceDescriptor, Buffer, BufferDescriptor, Rect, ResourceCreator,
    Texture, TextureDescriptor, Viewport,
    render_pass::{RenderPass, RenderPassDescriptor},
};

pub trait DeviceTrait: 'static {
    fn get_command_buffer_mut(&mut self) -> &mut CommandBuffer;

    fn create_texture(&self, desc: TextureDescriptor) -> Texture;

    fn create_buffer(&self, desc: BufferDescriptor) -> Buffer;

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass;
}

impl ResourceCreator for Device {
    fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
        match desc {
            AnyFGResourceDescriptor::Texture(desc) => self.create_texture(desc).into(),
            AnyFGResourceDescriptor::Buffer(desc) => self.create_buffer(desc).into(),
        }
    }
}

pub struct Device(Box<dyn DeviceTrait>);

impl DeviceTrait for Device {
    fn get_command_buffer_mut(&mut self) -> &mut CommandBuffer {
        self.0.get_command_buffer_mut()
    }

    fn create_texture(&self, desc: TextureDescriptor) -> Texture {
        self.0.create_texture(desc)
    }

    fn create_buffer(&self, desc: BufferDescriptor) -> Buffer {
        self.0.create_buffer(desc)
    }

    fn create_render_pass(&self, desc: RenderPassDescriptor) -> RenderPass {
        self.0.create_render_pass(desc)
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

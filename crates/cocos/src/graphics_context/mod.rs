pub mod pipeline;

use std::sync::Arc;

use cocos_renderer::Device;

use winit::window::Window;

pub enum GraphicsContext {
    Uninitialized,
    Initialized(InitializedGraphicsContext),
}

impl GraphicsContext {
    pub fn initialize_graphics_context(&mut self, deive: Device, window: Arc<Window>) {
        *self = GraphicsContext::Initialized(InitializedGraphicsContext::new(deive, window));
    }
}

pub struct InitializedGraphicsContext {
    device: Arc<Device>,
    window: Arc<Window>,
}

impl InitializedGraphicsContext {
    pub fn new(deive: Device, window: Arc<Window>) -> Self {
        Self {
            device: Arc::new(deive),
            window,
        }
    }

    pub fn render(&mut self) {
        println!("{:?} {:?}", self.device, self.window)
    }
}

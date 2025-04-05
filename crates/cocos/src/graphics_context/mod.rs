pub mod pipeline;

use std::sync::Arc;

use cocos_renderer::Device;

use pipeline::{RenderPipeline, deferred::DeferredRenderPipeline};
use winit::window::Window;

pub enum GraphicsContext {
    Uninitialized,
    Initialized(InitializedGraphicsContext),
}

impl GraphicsContext {
    pub fn initialize_graphics_context(&mut self, deive: Arc<Device>, window: Arc<Window>) {
        *self = GraphicsContext::Initialized(InitializedGraphicsContext::new(deive, window));
    }
}

pub struct InitializedGraphicsContext {
    device: Arc<Device>,
    window: Arc<Window>,
    render_pipeline: Box<dyn RenderPipeline>,
}

impl InitializedGraphicsContext {
    pub fn new(device: Arc<Device>, window: Arc<Window>) -> Self {
        let render_pipeline = DeferredRenderPipeline::new(device.clone());

        Self {
            device,
            window,
            render_pipeline: Box::new(render_pipeline),
        }
    }

    pub fn render(&mut self) {
        //todo delete
        println!("{:?} {:?}", self.device, self.window);
        self.render_pipeline.render(&vec![]);
    }
}

pub mod deferred;

use cocos_renderer::FrameGraph;

use crate::scene::Camera;

pub trait RenderPipeline {
    fn render(&mut self, cameras: &[Camera]);
}

pub struct FrameGraphContext<'a> {
    pub fg: &'a mut FrameGraph,
    pub camera: &'a Camera,
}

pub trait RenderFlow: 'static {
    fn name(&self) -> &str;

    fn setup(&self, context: &mut FrameGraphContext);
}

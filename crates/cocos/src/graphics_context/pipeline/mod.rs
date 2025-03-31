pub mod deferred;

use cocos_renderer::FrameGraph;

use crate::Camera;

pub trait RenderPipeline {
    fn activate(&mut self);

    fn on_global_pipeline_state_changed(&mut self);

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

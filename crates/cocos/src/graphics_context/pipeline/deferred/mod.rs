use crate::Camera;

use super::RenderPipeline;
use cocos_renderer::{Device, FrameGraph, TransientResourceCache};
use std::sync::Arc;

pub struct DeferredRenderPipeline {
    fg: FrameGraph,
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
}

impl DeferredRenderPipeline {
    pub fn new(device: Arc<Device>) -> Self {
        DeferredRenderPipeline {
            fg: FrameGraph::default(),
            device,
            transient_resource_cache: Default::default(),
        }
    }
}

impl RenderPipeline for DeferredRenderPipeline {
    fn render(&mut self, _cameras: &[Camera]) {
        self.fg
            .compile(&self.device, &mut self.transient_resource_cache);

        self.fg
            .execute(&self.device, &mut self.transient_resource_cache);

        self.fg.reset();
    }
}

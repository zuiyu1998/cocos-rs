mod main_flow;

use crate::Camera;

use super::{FrameGraphContext, RenderFlow, RenderPipeline};
use cocos_renderer::{Device, FrameGraph, TransientResourceCache};
use main_flow::MainFlow;
use std::sync::Arc;

pub struct DeferredRenderPipeline {
    fg: FrameGraph,
    device: Arc<Device>,
    transient_resource_cache: TransientResourceCache,
    flows: Vec<Box<dyn RenderFlow>>,
}

impl DeferredRenderPipeline {
    pub fn new(device: Arc<Device>) -> Self {
        let flows: Vec<Box<dyn RenderFlow>> = vec![Box::new(MainFlow::default())];

        DeferredRenderPipeline {
            fg: FrameGraph::default(),
            device,
            transient_resource_cache: Default::default(),
            flows,
        }
    }
}

impl RenderPipeline for DeferredRenderPipeline {
    fn render(&mut self, cameras: &[Camera]) {
        for camera in cameras.iter() {
            let mut context = FrameGraphContext {
                camera,
                fg: &mut self.fg,
            };

            for flow in self.flows.iter() {
                flow.setup(&mut context);
            }
        }

        self.fg
            .compile(&self.device, &mut self.transient_resource_cache);

        self.fg
            .execute(&self.device, &mut self.transient_resource_cache);

        self.fg.reset();
    }
}

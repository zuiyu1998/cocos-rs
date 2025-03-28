#[allow(dead_code)]
mod frame_graph;
#[allow(dead_code)]
mod gfx_base;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {}

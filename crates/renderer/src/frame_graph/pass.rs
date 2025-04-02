use super::{PassNodeBuilder, render_context::RenderContext};

pub trait Pass {
    fn setup(&mut self, builder: &mut PassNodeBuilder);

    fn execute(&mut self, render_context: &mut RenderContext);
}

pub type DynPass = Box<dyn Pass>;

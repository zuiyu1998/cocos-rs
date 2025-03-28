use crate::{FrameGraphContext, RenderFlow};

pub struct MainFlow {
    name: String,
}

impl Default for MainFlow {
    fn default() -> Self {
        MainFlow {
            name: "MainFlow".to_string(),
        }
    }
}

impl RenderFlow for MainFlow {
    fn name(&self) -> &str {
        &self.name
    }

    fn setup(&self, _context: &mut FrameGraphContext) {
        println!("{} RenderFlow setup", self.name())
    }
}

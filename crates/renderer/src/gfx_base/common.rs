use downcast_rs::{Downcast, impl_downcast};

pub trait GfxObject: Downcast {}

impl_downcast!(GfxObject);

pub struct PipelineCache {}

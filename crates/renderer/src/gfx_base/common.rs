pub const INVALID_BINDING: u32 = 0;

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Clone, Copy)]
pub enum AccessFlags {
    #[default]
    None,
}

#[derive(Debug, Default)]
pub struct SubpassInfo {
    pub colors: Vec<u32>,
    pub depth_stencil: u32,

    pub inputs: Vec<u32>,
    pub resolves: Vec<u32>,
    pub preserves: Vec<u32>,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Viewport {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rect {}

#[derive(Debug, Default, Clone)]
pub struct PassBarrierPair {}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy)]
pub enum LoadOp {
    // Load the previous content from memory
    #[default]
    Load,
    // // Clear the content to a fixed value
    // Clear,
    // Discard,
}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy)]
pub enum StoreOp {
    #[default]
    Store, // Store the pending content to memory
    Discard, // Discard the pending content
}

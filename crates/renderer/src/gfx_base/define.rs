pub struct Viewport {}

pub struct Rect {}

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

pub trait GFXObject {}

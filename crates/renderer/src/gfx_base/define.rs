pub struct Viewport {}

pub struct Rect {}

pub struct PassBarrierPair {}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord)]
pub enum LoadOp {
    // Load the previous content from memory
    #[default]
    Load,
    // // Clear the content to a fixed value
    // Clear,
    // Discard,
}
pub trait GFXObject {}

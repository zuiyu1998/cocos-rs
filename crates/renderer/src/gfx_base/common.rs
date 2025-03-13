use bitflags::bitflags;

pub const INVALID_BINDING: u32 = 0;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextFormat {
    Unknown,
}

bitflags! {
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SampleCount: u32 {
        const X1 = 0;
    }
}

bitflags! {
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TextureFlags: u32 {
        const NONE = 0;
    }
}

bitflags! {
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TextureUsage: u32 {
        const NONE = 0;
        const TRANSFER_SRC = 0x1;
        const TRANSFER_DST = 0x2;
        const SAMPLED = 0x4;
        const STORAGE = 0x8;
        const COLOR_ATTACHMENT = 0x10;
        const DEPTH_STENCIL_ATTACHMENT = 0x20;
        const INPUT_ATTACHMENT = 0x40;
        const SHADING_RATE = 0x80;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TextureType {
    Tex1d,
    #[default]
    Tex2d,
    Tex3d,
    Cube,
    Tex1dArray,
    Tex2dArray,
}

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

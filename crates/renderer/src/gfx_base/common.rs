use bitflags::bitflags;

pub const INVALID_BINDING: u32 = 0;

pub struct Color {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub barrier: Option<GeneralBarrier>,
    pub prev_accesses: AccessFlags,
    pub next_accesses: AccessFlags,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneralBarrier {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DepthStencilAttachment {
    pub format: Format,
    pub sample_count: SampleCount,
    pub depth_load_op: LoadOp,
    pub depth_store_op: StoreOp,
    pub stencil_load_op: LoadOp,
    pub stencil_store_op: StoreOp,
    pub barrier: Option<GeneralBarrier>,
}

impl Default for DepthStencilAttachment {
    fn default() -> Self {
        DepthStencilAttachment {
            format: Format::Unknown,
            sample_count: SampleCount::X1,
            depth_load_op: LoadOp::Clear,
            depth_store_op: StoreOp::Store,
            stencil_load_op: LoadOp::Clear,
            stencil_store_op: StoreOp::Store,
            barrier: None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorAttachment {
    pub format: Format,
    pub sample_count: SampleCount,
    pub load_op: LoadOp,
    pub store_op: StoreOp,
    pub barrier: Option<GeneralBarrier>,
}

impl Default for ColorAttachment {
    fn default() -> Self {
        ColorAttachment {
            format: Format::Unknown,
            sample_count: SampleCount::X1,
            load_op: LoadOp::Clear,
            store_op: StoreOp::Store,
            barrier: None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
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

bitflags! {
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
    pub struct AccessFlags: u32 {
        const NONE = 0;

    }
}

impl Default for AccessFlags {
    fn default() -> Self {
        AccessFlags::NONE
    }
}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Clone, Hash, Ord)]
pub struct SubpassInfo {
    pub colors: Vec<u32>,
    pub depth_stencil: u32,
    pub inputs: Vec<u32>,
    pub resolves: Vec<u32>,
    pub preserves: Vec<u32>,
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Viewport {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Clone)]
pub struct PassBarrierPair {}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LoadOp {
    // Load the previous content from memory
    #[default]
    Load,
    // Clear the content to a fixed value
    Clear,
    // Discard,
}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum StoreOp {
    #[default]
    Store, // Store the pending content to memory
    Discard, // Discard the pending content
}

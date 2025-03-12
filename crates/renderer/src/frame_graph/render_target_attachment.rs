use crate::gfx_base::{AccessFlags, LoadOp, StoreOp, Texture};

use super::handle::TypedHandle;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct RenderTargetAttachment {
    pub desc: RenderTargetAttachmentDescriptor,
    pub texture_handle: TypedHandle<Texture>,
    pub level: u8,
    pub layer: u8,
    pub index: u8,
    pub store_op: StoreOp,
}

impl RenderTargetAttachment {
    pub const DEPTH_STENCIL_SLOT_START: u8 = 13;

    pub fn to_info(&self) -> RenderTargetAttachmentInfo {
        RenderTargetAttachmentInfo {
            texture_handle_index: self.texture_handle.index,
            store_op: self.store_op,
            write_mask: self.desc.write_mask,
            load_op: self.desc.load_op,
            usage: self.desc.usage,
            slot: self.desc.slot,
            end_accesses: self.desc.end_accesses,
        }
    }
}

pub struct RenderTargetAttachmentInfo {
    pub texture_handle_index: usize,
    pub store_op: StoreOp,
    pub write_mask: u8,
    pub load_op: LoadOp,
    pub usage: RenderTargetAttachmentUsage,
    pub slot: u8,
    pub end_accesses: AccessFlags,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub enum RenderTargetAttachmentUsage {
    #[default]
    Color = 0,
    Depth = 1,
    Stencil = 2,
    DepthStencil = 3,
}

#[derive(Debug, PartialEq, Eq, Default, PartialOrd)]
pub struct RenderTargetAttachmentDescriptor {
    pub usage: RenderTargetAttachmentUsage,
    pub slot: u8,
    pub write_mask: u8,
    pub load_op: LoadOp,

    pub end_accesses: AccessFlags,
}

impl Ord for RenderTargetAttachment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.desc.usage == other.desc.usage {
            return self.desc.slot.cmp(&other.desc.slot);
        }

        self.desc.usage.cmp(&other.desc.usage)
    }
}
impl PartialOrd for RenderTargetAttachment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod test {

    #[test]
    fn test_attachment_ord() {
        use super::{
            RenderTargetAttachment, RenderTargetAttachmentDescriptor, RenderTargetAttachmentUsage,
        };

        let a = RenderTargetAttachment {
            desc: RenderTargetAttachmentDescriptor {
                slot: 0,
                usage: RenderTargetAttachmentUsage::Color,
                ..Default::default()
            },
            texture_handle: Default::default(),
            ..Default::default()
        };

        let mut b = RenderTargetAttachment {
            desc: RenderTargetAttachmentDescriptor {
                slot: 1,
                usage: RenderTargetAttachmentUsage::Color,
                ..Default::default()
            },
            texture_handle: Default::default(),
            ..Default::default()
        };

        let v = a < b;
        assert!(v);

        b.desc.slot = 0;
        let v = a == b;
        assert!(v);

        b.desc.usage = RenderTargetAttachmentUsage::Depth;
        let v = a < b;
        assert!(v);
    }
}

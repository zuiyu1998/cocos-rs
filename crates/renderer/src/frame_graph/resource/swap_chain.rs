use crate::{SwapChain, SwapChainInfo};

use super::{AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor};

impl FGResource for SwapChain {
    type Descriptor = SwapChainInfo;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::OwnedSwapChain(res) => &res,
            _ => {
                unimplemented!()
            }
        }
    }
}

impl FGResourceDescriptor for SwapChainInfo {
    type Resource = SwapChain;
}

impl From<SwapChainInfo> for AnyFGResourceDescriptor {
    fn from(value: SwapChainInfo) -> Self {
        AnyFGResourceDescriptor::SwapChain(value)
    }
}

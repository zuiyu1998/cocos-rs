use std::collections::HashMap;

use crate::{Device, TypeHandle};

use super::{
    AnyFGResource, AnyFGResourceDescriptor, FGResource, ImportedVirtualResource,
    TransientResourceCache, VirtualResource, VirtualResourceState,
};

#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<TypeHandle<VirtualResource>, AnyFGResource>,
}

impl ResourceTable {
    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &TypeHandle<VirtualResource>,
    ) -> Option<&ResourceType> {
        self.resources
            .get(handle)
            .map(|any| ResourceType::borrow_resource(any))
    }

    pub fn request_resources(
        &mut self,
        resource: &VirtualResource,
        device: &Device,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let handle = resource.info.handle;

        let resource = match &resource.state {
            VirtualResourceState::Imported(state) => match &state.resource {
                ImportedVirtualResource::Texture(resource) => {
                    AnyFGResource::ImportedTexture(resource.clone())
                }
            },
            VirtualResourceState::Setup(desc) => match desc {
                AnyFGResourceDescriptor::Texture(texture_desc) => transient_resource_cache
                    .get_image(texture_desc)
                    .map(AnyFGResource::OwnedTexture)
                    .unwrap_or_else(|| device.create(desc)),
                _ => {
                    return;
                }
            },
        };

        self.resources.insert(handle, resource);
    }

    pub fn release_resource(
        &mut self,
        handle: &TypeHandle<VirtualResource>,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        if let Some(resource) = self.resources.remove(handle) {
            match resource {
                AnyFGResource::OwnedSwapChain(mut res) => {
                    res.present();
                }
                AnyFGResource::ImportedTexture(_) => {}
                AnyFGResource::OwnedTexture(texture) => {
                    transient_resource_cache.insert_image(texture.get_desc().clone(), texture);
                }
            }
        }
    }
}

use std::collections::HashMap;

use crate::TypeHandle;

use super::{
    AnyFGResource, AnyFGResourceDescriptor, ImportedVirtualResource, ResourceCreator,
    TransientResourceCache, VirtualResource, VirtualResourceState,
};

#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<TypeHandle<VirtualResource>, AnyFGResource>,
}

impl ResourceTable {
    pub fn request_resources(
        &mut self,
        resource: &VirtualResource,
        creator: &impl ResourceCreator,
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
                    .unwrap_or_else(|| creator.create(desc)),
            },
        };

        self.resources.insert(handle, resource);
    }

    pub fn release_resources(self, transient_resource_cache: &mut TransientResourceCache) {
        for resource in self.resources.into_values() {
            match resource {
                AnyFGResource::ImportedTexture(_) => {}
                AnyFGResource::OwnedTexture(texture) => {
                    transient_resource_cache.insert_image(texture.get_desc().clone(), texture);
                }
            }
        }
    }
}

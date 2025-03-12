use std::{mem::swap, sync::Arc};

use super::{Allocator, AnyResource, FGResource, StringHandle, pass::PassNode};

#[derive(Default)]
pub struct VirtualResourceInfo {
    pub first_pass_index: Option<usize>,
    pub last_pass_index: Option<usize>,
    pub ref_count: u32,
    pub writer_count: u32,
    pub imported: bool,
    pub never_loaded: bool,
    pub never_stored: bool,
    pub memoryless: bool,
    pub memoryless_msaa: bool,
    pub name: StringHandle,
    pub id: usize,
    pub version: u8,
}

impl VirtualResourceInfo {
    pub fn update_lifetime(&mut self, pass_node: &PassNode) {
        if self.first_pass_index.is_none() {
            self.first_pass_index = Some(pass_node.id);
        }

        self.last_pass_index = Some(pass_node.id)
    }

    pub fn new_version(&mut self) {
        self.version += 1;
    }
}

///资源
pub trait VirtualResource {
    fn get_info(&self) -> &VirtualResourceInfo;
    fn get_mut_info(&mut self) -> &mut VirtualResourceInfo;
    fn request(&mut self, allocator: &Allocator);
    fn release(&mut self, allocator: &Allocator);

    fn get_any_resource(&self) -> Option<Arc<AnyResource>> {
        None
    }
}

pub enum ResourceEntryState<ResourceType: FGResource> {
    Uninitialized(ResourceType::Descriptor),
    Initialization {
        resource: AnyResource,
        desc: ResourceType::Descriptor,
    },
}

pub struct ResourceEntry<ResourceType>
where
    ResourceType: FGResource,
{
    resource: ResourceEntryState<ResourceType>,
    info: VirtualResourceInfo,
}

impl<ResourceType> ResourceEntry<ResourceType>
where
    ResourceType: FGResource,
{
    pub fn new(id: usize, name: StringHandle, desc: ResourceType::Descriptor) -> Self {
        Self {
            resource: ResourceEntryState::Uninitialized(desc),
            info: VirtualResourceInfo {
                name,
                id,
                ..Default::default()
            },
        }
    }
}

impl<ResourceType> VirtualResource for ResourceEntry<ResourceType>
where
    ResourceType: FGResource,
{
    fn get_any_resource(&self) -> Option<Arc<AnyResource>> {
        todo!()
    }

    fn get_info(&self) -> &VirtualResourceInfo {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VirtualResourceInfo {
        &mut self.info
    }

    fn release(&mut self, allocator: &Allocator) {
        let desc = match &mut self.resource {
            ResourceEntryState::Uninitialized(_) => {
                return;
            }
            ResourceEntryState::Initialization { desc, .. } => desc.clone(),
        };

        let mut temp_state = ResourceEntryState::Uninitialized(desc);
        swap(&mut temp_state, &mut self.resource);

        if let ResourceEntryState::Initialization { resource, .. } = temp_state {
            ResourceType::destroy_transient(allocator, resource);
        }
    }

    fn request(&mut self, allocator: &Allocator) {
        if let ResourceEntryState::Uninitialized(desc) = &mut self.resource {
            self.resource = ResourceEntryState::Initialization {
                desc: desc.clone(),
                resource: ResourceType::create_transient(allocator, desc),
            };
        }
    }
}

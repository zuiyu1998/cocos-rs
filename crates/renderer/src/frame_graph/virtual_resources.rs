use std::mem::swap;

use crate::gfx_base::GFXObject;

use super::{FrameResource, ResourceRef, StringHandle, pass::PassNode};

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
pub trait VirtualResource: GFXObject {
    fn get_info(&self) -> &VirtualResourceInfo;
    fn get_mut_info(&mut self) -> &mut VirtualResourceInfo;
    fn request(&mut self);
    fn release(&mut self);
}

pub enum ResourceEntryState<ResourceType: FrameResource> {
    Uninitialized(ResourceType::Descriptor),
    Initialization(ResourceRef<ResourceType, ResourceType::Descriptor>),
}

pub struct ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    resource: ResourceEntryState<ResourceType>,
    info: VirtualResourceInfo,
}

impl<ResourceType> ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
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

impl<ResourceType> GFXObject for ResourceEntry<ResourceType> where ResourceType: FrameResource {}

impl<ResourceType> VirtualResource for ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    fn get_info(&self) -> &VirtualResourceInfo {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VirtualResourceInfo {
        &mut self.info
    }

    fn release(&mut self) {
        let desc = match &mut self.resource {
            ResourceEntryState::Uninitialized(_) => {
                return;
            }
            ResourceEntryState::Initialization(resource_ref) => resource_ref.desc.clone(),
        };

        let mut temp_state = ResourceEntryState::Uninitialized(desc);
        swap(&mut temp_state, &mut self.resource);

        if let ResourceEntryState::Initialization(resource_ref) = temp_state {
            ResourceType::destroy_transient(resource_ref);
        }
    }

    fn request(&mut self) {
        if let ResourceEntryState::Uninitialized(desc) = &mut self.resource {
            self.resource =
                ResourceEntryState::Initialization(ResourceType::create_transient(desc));
        }
    }
}

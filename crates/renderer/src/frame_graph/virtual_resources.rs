use std::{mem::swap, rc::Rc};

use super::pass::PassNode;

use crate::gfx_base::{Allocator, AnyFGResource, AnyResource, FGResource, Handle};

#[derive(Default)]
pub struct VirtualResourceInfo {
    pub first_pass_index: Option<Handle>,
    pub last_pass_index: Option<Handle>,
    pub ref_count: u32,
    pub writer_count: u32,
    pub imported: bool,
    pub never_loaded: bool,
    pub never_stored: bool,
    pub memoryless: bool,
    pub memoryless_msaa: bool,
    pub name: String,
    pub handle: Handle,
    pub version: u8,
}

impl VirtualResourceInfo {
    pub fn update_lifetime(&mut self, pass_node: &PassNode) {
        if self.first_pass_index.is_none() {
            self.first_pass_index = Some(pass_node.handle);
        }

        self.last_pass_index = Some(pass_node.handle)
    }

    pub fn new_version(&mut self) {
        self.version += 1;
    }
}

///资源
pub trait VirtualResource: 'static {
    fn info(&self) -> &VirtualResourceInfo;
    fn info_mut(&mut self) -> &mut VirtualResourceInfo;

    fn request(&mut self, allocator: &mut Allocator);
    fn release(&mut self, allocator: &mut Allocator);

    fn get_any_resource(&self) -> Option<Rc<AnyFGResource>> {
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
    pub fn new(handle: Handle, name: &str, desc: ResourceType::Descriptor) -> Self {
        Self {
            resource: ResourceEntryState::Uninitialized(desc),
            info: VirtualResourceInfo {
                name: name.to_owned(),
                handle,
                ..Default::default()
            },
        }
    }
}

impl<ResourceType> VirtualResource for ResourceEntry<ResourceType>
where
    ResourceType: FGResource,
{
    fn get_any_resource(&self) -> Option<Rc<AnyFGResource>> {
        match &self.resource {
            ResourceEntryState::Uninitialized(_) => None,
            ResourceEntryState::Initialization { resource, .. } => Some(resource.resource.clone()),
        }
    }

    fn info(&self) -> &VirtualResourceInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut VirtualResourceInfo {
        &mut self.info
    }

    fn release(&mut self, allocator: &mut Allocator) {
        let name = self.info.name.clone();

        let desc = match &mut self.resource {
            ResourceEntryState::Uninitialized(_) => {
                return;
            }
            ResourceEntryState::Initialization { desc, .. } => desc.clone(),
        };

        let mut temp_state = ResourceEntryState::Uninitialized(desc);
        swap(&mut temp_state, &mut self.resource);

        if let ResourceEntryState::Initialization { resource, .. } = temp_state {
            allocator.free(&name, resource);
        }
    }

    fn request(&mut self, allocator: &mut Allocator) {
        if let ResourceEntryState::Uninitialized(desc) = &mut self.resource {
            self.resource = ResourceEntryState::Initialization {
                desc: desc.clone(),
                resource: allocator.alloc(&self.info.name, &desc.clone().into()),
            };
        }
    }
}

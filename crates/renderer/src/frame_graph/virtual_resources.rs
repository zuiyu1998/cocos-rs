use crate::gfx_base::GFXObject;

use super::{FrameResource, StringHandle, pass::PassNode};

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
}

///资源
pub trait VirtualResource: GFXObject {
    fn get_info(&self) -> &VirtualResourceInfo;
    fn get_mut_info(&mut self) -> &mut VirtualResourceInfo;
}

pub enum FrameResourceState<ResourceType: FrameResource> {
    Uninitialized(ResourceType::Descriptor),
    Initialization(ResourceType),
}

pub struct ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    resource: FrameResourceState<ResourceType>,
    info: VirtualResourceInfo,
}

impl<ResourceType> ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    pub fn new(id: usize, name: StringHandle, desc: ResourceType::Descriptor) -> Self {
        Self {
            resource: FrameResourceState::Uninitialized(desc),
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
}

use crate::{CommandBuffer, Device};

use super::{FGResource, GpuRead, ResourceRef, ResourceTable, TransientResourceCache};

pub struct RenderContext<'a> {
    device: &'a Device,
    cb: Option<CommandBuffer>,
    pub(crate) resource_table: ResourceTable,
    pub(crate) transient_resource_cache: &'a mut TransientResourceCache,
}

impl<'a> RenderContext<'a> {

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &ResourceRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(&handle.resource_handle())
    }

    pub fn device(&self) -> &Device {
        self.device
    }

    pub fn set_cb(&mut self, cb: CommandBuffer) {
        self.cb = Some(cb);
    }

    pub fn take_cb(&mut self) -> Option<CommandBuffer> {
        self.cb.take()
    }

    pub fn new(
        device: &'a Device,
        transient_resource_cache: &'a mut TransientResourceCache,
    ) -> Self {
        Self {
            device,
            cb: None,
            resource_table: Default::default(),
            transient_resource_cache,
        }
    }
}

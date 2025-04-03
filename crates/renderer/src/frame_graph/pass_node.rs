use std::marker::PhantomData;

use crate::gfx_base::TypeHandle;

use super::{DynPass, FrameGraph, ResourceNode, ResourceNodeHandle, VirtualResource};

pub trait GpuViewType {
    const IS_WRITABLE: bool;
}

#[derive(Debug)]
pub struct GpuRead;

impl GpuViewType for GpuRead {
    const IS_WRITABLE: bool = false;
}

#[derive(Debug)]
pub struct GpuWrite;

impl GpuViewType for GpuWrite {
    const IS_WRITABLE: bool = true;
}

#[derive(Debug)]
pub struct ResourceRef<ResourceType, ViewType> {
    handle: ResourceNodeHandle<ResourceType>,
    _marker: PhantomData<ViewType>,
}

impl<ResourceType, ViewType>  Clone for ResourceRef<ResourceType, ViewType>  {
    fn clone(&self) -> Self {
        Self { handle: self.handle.clone(), _marker: PhantomData }
    }
}

impl<ResourceType, ViewType> ResourceRef<ResourceType, ViewType> {
    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle.resource_node_handle()
    }

    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.handle.resource_handle()
    }

    pub fn new(handle: ResourceNodeHandle<ResourceType>) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }
}

pub struct PassNode {
    pub insert_point: usize,
    pub name: String,
    pub handle: TypeHandle<PassNode>,
    pub pass: Option<DynPass>,
    pub resource_request_array: Vec<TypeHandle<VirtualResource>>,
    ///使用资源的释放生命周期
    pub resource_release_array: Vec<TypeHandle<VirtualResource>>,

    pub writes: Vec<TypeHandle<ResourceNode>>,
    pub reads: Vec<TypeHandle<ResourceNode>>,
}

impl PassNode {
    pub fn write<ResourceType>(
        &mut self,
        graph: &mut FrameGraph,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuWrite> {
        let resource_handle = graph
            .get_resource_node(&resource_node_handle.resource_node_handle())
            .resource_handle;
        let resource = graph.get_resource_mut(&resource_handle);
        resource.info.new_version();

        let resource_info = resource.info.clone();
        let new_resource_node_handle = graph.create_resource_node(resource_info);
        let new_resource_node = graph.get_resource_node_mut(&new_resource_node_handle);
        new_resource_node.pass_node_writer_handle = Some(self.handle);

        self.writes.push(new_resource_node_handle);

        ResourceRef::new(ResourceNodeHandle::new(
            new_resource_node_handle,
            resource_handle,
        ))
    }

    pub fn read<ResourceType>(
        &mut self,
        graph: &FrameGraph,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, GpuRead> {
        let resource_node_handle = resource_node_handle.resource_node_handle();

        if !self.reads.contains(&resource_node_handle) {
            self.reads.push(resource_node_handle);
        }

        let resource_handle = graph
            .get_resource_node(&resource_node_handle)
            .resource_handle;

        ResourceRef::new(ResourceNodeHandle::new(
            resource_node_handle,
            resource_handle,
        ))
    }

    pub fn new(insert_point: usize, name: &str, handle: TypeHandle<PassNode>) -> Self {
        PassNode {
            name: name.to_string(),
            handle,
            pass: None,
            writes: vec![],
            reads: vec![],
            insert_point,
            resource_request_array: vec![],
            resource_release_array: vec![],
        }
    }
}

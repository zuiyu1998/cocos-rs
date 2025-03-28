use std::marker::PhantomData;

use crate::gfx_base::TypeHandle;

use super::{PassNode, VirtualResource};

pub struct ResourceNode {
    ///资源索引
    pub resource_handle: TypeHandle<VirtualResource>,
    ///自身索引
    pub handle: TypeHandle<ResourceNode>,
    /// 资源版本
    pub version: u32,
    /// 当前写入此资源节点的渲染节点
    pub pass_node_writer_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceNode {
    pub fn new(
        handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<VirtualResource>,
        version: u32,
    ) -> Self {
        ResourceNode {
            handle,
            version,
            pass_node_writer_handle: None,
            resource_handle,
        }
    }
}

pub struct ResourceNodeHandle<ResourceType> {
    resource_node_handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<VirtualResource>,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> ResourceNodeHandle<ResourceType> {
    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.resource_node_handle
    }

    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.resource_handle
    }

    pub fn new(
        resource_node_handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<VirtualResource>,
    ) -> Self {
        ResourceNodeHandle {
            resource_node_handle,
            resource_handle,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType> Clone for ResourceNodeHandle<ResourceType> {
    fn clone(&self) -> Self {
        ResourceNodeHandle {
            resource_node_handle: self.resource_node_handle,
            resource_handle: self.resource_handle,
            _marker: PhantomData,
        }
    }
}

use crate::gfx_base::TypeHandle;

use super::{
    FGResource, FGResourceDescriptor, FrameGraph, GpuRead, GpuWrite, PassNode, ResourceNodeHandle,
    ResourceNodeRef, TypeEquals,
};

pub struct PassNodeBuilder<'a> {
    graph: &'a mut FrameGraph,
    pass_node: Option<PassNode>,
}

impl<'a> PassNodeBuilder<'a> {
    pub fn build(mut self) -> PassNode {
        self.pass_node.take().unwrap()
    }

    pub fn new(
        insert_point: usize,
        name: &str,
        handle: TypeHandle<PassNode>,
        graph: &'a mut FrameGraph,
    ) -> Self {
        Self {
            graph,
            pass_node: Some(PassNode::new(insert_point, name, handle)),
        }
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> ResourceNodeHandle<DescriptorType::Resource>
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn read<ResourceType>(
        &mut self,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuRead> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read(self.graph, resource_node_handle)
    }

    pub fn write<ResourceType>(
        &mut self,
        resource_node_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuWrite> {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(self.graph, resource_node_handle)
    }
}

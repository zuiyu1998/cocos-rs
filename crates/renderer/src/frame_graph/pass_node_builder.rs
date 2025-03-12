use super::{FrameGraph, StringHandle, pass::PassNode};
use crate::gfx_base::{FGResource, FGResourceDescriptor, TypeEquals, TypedHandle};

pub struct PassNodeBuilder<'a> {
    pub pass_node: PassNode,
    pub graph: &'a mut FrameGraph,
}

impl PassNodeBuilder<'_> {
    pub fn create<DescriptorType>(
        &mut self,
        name: StringHandle,
        desc: DescriptorType,
    ) -> TypedHandle<DescriptorType::Resource>
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn build(self) {
        self.graph.create_pass_node(self.pass_node);
    }

    pub fn read<Resource: FGResource>(
        &mut self,
        input_handle: TypedHandle<Resource>,
    ) -> TypedHandle<Resource> {
        self.pass_node.read(input_handle.handle());
        input_handle
    }

    pub fn write<Resource: FGResource>(
        &mut self,
        out_handle: TypedHandle<Resource>,
    ) -> TypedHandle<Resource> {
        let out_handle_index = self.pass_node.write(self.graph, out_handle.handle());
        TypedHandle::new(out_handle_index)
    }
}

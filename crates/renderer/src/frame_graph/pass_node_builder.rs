use super::{
    FrameGraph, FrameResource, FrameResourceDescriptor, StringHandle, TypeEquals,
    handle::TypedHandle, pass::PassNode,
};

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
    DescriptorType: FrameResourceDescriptor + TypeEquals<Other = <<DescriptorType as FrameResourceDescriptor>::Resource as FrameResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn build(self) {
        self.graph.create_pass_node(self.pass_node);
    }

    pub fn read<Resource: FrameResource>(
        &mut self,
        input_handle: TypedHandle<Resource>,
    ) -> TypedHandle<Resource> {
        self.pass_node.read(input_handle.index);
        input_handle
    }

    pub fn write<Resource: FrameResource>(
        &mut self,
        out_handle: TypedHandle<Resource>,
    ) -> TypedHandle<Resource> {
        let out_handle_index = self.pass_node.write(self.graph, out_handle.index);
        TypedHandle::new(out_handle_index)
    }
}

use std::marker::PhantomData;

use crate::utils::IndexHandle;

type StringHandle = IndexHandle<String, u32>;
type Id = u16;

pub struct TypedHandle<ResourceType> {
    index: u16,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> TypedHandle<ResourceType> {
    fn new(index: u16) -> Self {
        Self {
            index,
            _marker: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct FrameGraph {
    virtual_resources: Vec<Box<dyn VirtualResource>>,
    resource_nodes: Vec<ResourceNode>,
}

impl FrameGraph {
    pub fn create<DescriptorType>(&mut self, name: StringHandle, desc: DescriptorType) -> TypedHandle<DescriptorType::Resource>
    where
        DescriptorType: FrameResourceDescriptor + TypeEquals<Other = <<DescriptorType as FrameResourceDescriptor>::Resource as FrameResource>::Descriptor>,
    {
        let virtual_resource: Box<dyn VirtualResource> =
            Box::new(ResourceEntry::<DescriptorType::Resource>::new(
                self.virtual_resources.len() as Id,
                name,
                TypeEquals::same(desc),
            ));

        let index = self.create_resource_node(virtual_resource);

        TypedHandle::new(index as u16)
    }

    pub fn create_resource_node(&mut self, virtual_resource: Box<dyn VirtualResource>) -> usize {
        let id = virtual_resource.get_id();
        let version = virtual_resource.get_version();
        self.virtual_resources.push(virtual_resource);

        let index = self.resource_nodes.len();

        self.resource_nodes.push(ResourceNode {
            virtual_resource_id: id,
            version,
        });

        index
    }
}

pub struct ResourceNode {
    virtual_resource_id: Id,
    version: u8,
}

///资源
pub trait FrameResource: 'static {
    type Descriptor: FrameResourceDescriptor;
}

///资源描述符
pub trait FrameResourceDescriptor {
    type Resource: FrameResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
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
    first_use_pass: Option<PassNode>,
    last_use_pass: Option<PassNode>,
    ref_count: u32,
    writer_count: u32,
    imported: bool,
    never_loaded: bool,
    never_stored: bool,
    memoryless: bool,
    memoryless_msaa: bool,
    name: StringHandle,
    id: Id,
    version: u8,
}

impl<ResourceType> ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    pub fn new(id: Id, name: StringHandle, desc: ResourceType::Descriptor) -> Self {
        Self {
            resource: FrameResourceState::Uninitialized(desc),
            first_use_pass: None,
            last_use_pass: None,
            ref_count: 0,
            writer_count: 0,
            imported: false,
            never_loaded: true,
            never_stored: true,
            memoryless: false,
            memoryless_msaa: false,
            id,
            name,
            version: 0,
        }
    }
}

impl<ResourceType> GFXObject for ResourceEntry<ResourceType> where ResourceType: FrameResource {}

impl<ResourceType> VirtualResource for ResourceEntry<ResourceType>
where
    ResourceType: FrameResource,
{
    fn get_id(&self) -> Id {
        self.id
    }

    fn request(&self) -> usize {
        todo!()
    }

    fn release(&self) -> usize {
        todo!()
    }

    fn is_imported(&self) -> bool {
        self.imported
    }

    fn update_lifetime(&mut self, _pass_node: &PassNode) -> bool {
        todo!()
    }

    fn get_version(&self) -> u8 {
        self.version
    }

    fn new_version(&mut self) {
        self.version += 1;
    }
}

pub trait GFXObject {}

pub struct PassNode {}

///资源节点？
pub trait VirtualResource: GFXObject {
    fn get_id(&self) -> Id {
        todo!()
    }
    fn request(&self) -> usize;
    fn release(&self) -> usize;
    fn is_imported(&self) -> bool;
    fn update_lifetime(&mut self, pass_node: &PassNode) -> bool;
    fn new_version(&mut self);
    fn get_version(&self) -> u8;
}

mod test {

    use super::{FrameResource, FrameResourceDescriptor};

    pub struct TestFrameResource;

    pub struct TestFrameResourceDescriptor;

    impl FrameResource for TestFrameResource {
        type Descriptor = TestFrameResourceDescriptor;
    }

    impl FrameResourceDescriptor for TestFrameResourceDescriptor {
        type Resource = TestFrameResource;
    }

    #[test]
    fn test_create() {
        use super::{FrameGraph, StringHandle};

        let mut frame_graph = FrameGraph::default();

        let desc = TestFrameResourceDescriptor;

        let name = StringHandle::new("test".to_string(), 1);

        let handle = frame_graph.create(name, desc);

        assert_eq!(handle.index, 0);
    }
}

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{FrameResource, FrameResourceDescriptor, ResourceRef};

pub struct Allocator<Resource, Descriptor, Creator>(
    Arc<Mutex<AllocatorInternal<Resource, Descriptor, Creator>>>,
);

impl<Resource, Descriptor, Creator> Clone for Allocator<Resource, Descriptor, Creator> {
    fn clone(&self) -> Self {
        Allocator(Arc::clone(&self.0))
    }
}

pub trait ResourceCreator {
    type Resource;
    type Descriptor;
    fn create(&self, desc: &Self::Descriptor) -> Self::Resource;
}

impl<Resource, Descriptor, Creator> Allocator<Resource, Descriptor, Creator>
where
    Resource: FrameResource,
    Descriptor: FrameResourceDescriptor,
    Creator: ResourceCreator<Resource = Resource, Descriptor = Descriptor>,
{
    pub fn new(creator: Creator) -> Self {
        Allocator(Arc::new(Mutex::new(AllocatorInternal::new(creator))))
    }

    pub fn alloc(&self, desc: &Descriptor) -> ResourceRef<Resource, Descriptor> {
        let mut guard = self.0.lock().unwrap();
        guard.alloc(desc)
    }

    pub fn free(&self, resource: ResourceRef<Resource, Descriptor>) {
        let mut guard = self.0.lock().unwrap();
        guard.free(resource)
    }
}

pub struct AllocatorInternal<Resource, Descriptor, Creator> {
    pool: HashMap<Descriptor, ResourceState<Resource>>,
    creator: Creator,
}

pub struct ResourceState<Resource> {
    resource: Arc<Resource>,
    count: usize,
}

impl<Resource, Descriptor, Creator> AllocatorInternal<Resource, Descriptor, Creator>
where
    Resource: FrameResource,
    Descriptor: FrameResourceDescriptor,
    Creator: ResourceCreator<Resource = Resource, Descriptor = Descriptor>,
{
    pub fn new(creator: Creator) -> Self {
        Self {
            pool: Default::default(),
            creator,
        }
    }

    fn alloc(&mut self, desc: &Descriptor) -> ResourceRef<Resource, Descriptor> {
        if let Some(state) = self.pool.get_mut(desc) {
            state.count += 1;
            return ResourceRef {
                desc: desc.clone(),
                resource: state.resource.clone(),
            };
        }

        let resource = Arc::new(self.creator.create(desc));

        let state = ResourceState {
            resource: resource.clone(),
            count: 0,
        };

        self.pool.insert(desc.clone(), state);

        ResourceRef {
            desc: desc.clone(),
            resource,
        }
    }

    fn free(&mut self, resource: ResourceRef<Resource, Descriptor>) {
        if let Some(state) = self.pool.get_mut(&resource.desc) {
            state.count -= 1;

            if state.count == 0 {
                self.pool.remove(&resource.desc);
            }
        }
    }
}

mod test {

    use std::sync::OnceLock;

    use super::{Allocator, ResourceCreator};
    use crate::frame_graph::{
        FrameResource, FrameResourceAllocator, FrameResourceDescriptor, ResourceRef,
    };

    static TEST_ALLOCATOR: OnceLock<TestAllocator> = OnceLock::new();

    pub struct TestCreator;

    impl ResourceCreator for TestCreator {
        type Descriptor = TestResourceDescriptor;
        type Resource = TestResource;

        fn create(&self, _desc: &Self::Descriptor) -> Self::Resource {
            TestResource {}
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TestResource {}

    impl FrameResource for TestResource {
        type Allocator = TestAllocator;
        type Descriptor = TestResourceDescriptor;
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TestResourceDescriptor {
        id: usize,
    }

    impl FrameResourceDescriptor for TestResourceDescriptor {
        type Resource = TestResource;
    }

    #[derive(Clone)]
    pub struct TestAllocator(pub Allocator<TestResource, TestResourceDescriptor, TestCreator>);

    impl TestAllocator {
        pub fn initialization(creator: TestCreator) {
            TEST_ALLOCATOR.get_or_init(|| TestAllocator(Allocator::new(creator)));
        }
    }

    impl FrameResourceAllocator for TestAllocator {
        type Descriptor = TestResourceDescriptor;
        type Resource = TestResource;

        fn alloc(&self, desc: &Self::Descriptor) -> ResourceRef<Self::Resource, Self::Descriptor> {
            self.0.alloc(desc)
        }

        fn get_instance() -> Self {
            TEST_ALLOCATOR.get().cloned().unwrap()
        }

        fn free(&self, resource: ResourceRef<Self::Resource, Self::Descriptor>) {
            self.0.free(resource)
        }
    }

    #[test]
    fn test_allocator() {
        TestAllocator::initialization(TestCreator {});

        let desc = TestResourceDescriptor { id: 0 };

        let resource_ref_0 = TestAllocator::get_instance().alloc(&desc);
        let resource_ref_1 = TestAllocator::get_instance().alloc(&desc);
        let resource_ref_2 = TestAllocator::get_instance().alloc(&desc);

        assert_eq!(resource_ref_0.resource, resource_ref_1.resource);
        assert_eq!(resource_ref_1.resource, resource_ref_2.resource);
        assert_eq!(resource_ref_0.resource, resource_ref_2.resource);

        {
            let allocator = TestAllocator::get_instance();
            let guard = allocator.0.0.lock().unwrap();
            let count = guard.pool.get(&desc).unwrap().count;
            assert_eq!(count, 2);
        }

        TestAllocator::get_instance().free(resource_ref_0);
        TestAllocator::get_instance().free(resource_ref_1);
        TestAllocator::get_instance().free(resource_ref_2);

        {
            let allocator = TestAllocator::get_instance();
            let guard = allocator.0.0.lock().unwrap();
            let none = !guard.pool.contains_key(&desc);
            assert!(none);
        }
    }
}

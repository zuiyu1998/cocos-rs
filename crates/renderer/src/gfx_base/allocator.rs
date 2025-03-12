use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{AnyFGResource, AnyFGResourceDescriptor, AnyResource};

pub struct Allocator(Arc<Mutex<AllocatorInternal>>);

impl Clone for Allocator {
    fn clone(&self) -> Self {
        Allocator(Arc::clone(&self.0))
    }
}

pub trait ResourceCreator: 'static + Send + Sync {
    fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource;
}

impl Allocator {
    pub fn new(creator: impl ResourceCreator) -> Self {
        let creator: Box<dyn ResourceCreator> = Box::new(creator);

        Allocator(Arc::new(Mutex::new(AllocatorInternal::new(creator))))
    }

    pub fn alloc(&self, desc: &AnyFGResourceDescriptor) -> AnyResource {
        let mut guard = self.0.lock().unwrap();
        guard.alloc(desc)
    }

    pub fn free(&self, resource: AnyResource) {
        let mut guard = self.0.lock().unwrap();
        guard.free(resource)
    }
}

pub struct AllocatorInternal {
    pool: HashMap<AnyFGResourceDescriptor, ResourceState>,
    creator: Box<dyn ResourceCreator>,
}

pub struct ResourceState {
    resource: Arc<AnyFGResource>,
    count: usize,
}

impl ResourceState {
    pub fn new(resource: Arc<AnyFGResource>, count: usize) -> Self {
        ResourceState { resource, count }
    }
}

impl AllocatorInternal {
    pub fn new(creator: Box<dyn ResourceCreator>) -> Self {
        Self {
            pool: Default::default(),
            creator,
        }
    }

    fn alloc(&mut self, desc: &AnyFGResourceDescriptor) -> AnyResource {
        if let Some(state) = self.pool.get_mut(desc) {
            state.count += 1;
            return AnyResource::new(desc.clone(), state.resource.clone());
        }

        let resource = Arc::new(self.creator.create(desc.clone()));

        let state = ResourceState::new(resource.clone(), 1);

        self.pool.insert(desc.clone(), state);

        AnyResource {
            desc: desc.clone(),
            resource,
        }
    }

    fn free(&mut self, resource: AnyResource) {
        if let Some(state) = self.pool.get_mut(&resource.desc) {
            state.count -= 1;

            if state.count == 0 {
                self.pool.remove(&resource.desc);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Allocator, ResourceCreator};
    use crate::gfx_base::{AnyFGResource, AnyFGResourceDescriptor, Texture, TextureDescriptor};

    pub struct TestResourceCreator {}

    impl ResourceCreator for TestResourceCreator {
        fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
            match desc {
                AnyFGResourceDescriptor::Texture(desc) => {
                    AnyFGResource::Texture(Texture::new(desc))
                }
            }
        }
    }

    #[test]
    fn test_allocator() {
        let allocator = Allocator::new(TestResourceCreator {});

        let texture_desc = TextureDescriptor::default().into();

        let a = allocator.alloc(&texture_desc);
        let b = allocator.alloc(&texture_desc);

        assert_eq!(a.resource, b.resource);

        {
            let guard = allocator.0.lock().unwrap();
            let state = guard.pool.get(&texture_desc).unwrap();
            assert_eq!(state.count, 2);
        }

        allocator.free(a);
        allocator.free(b);

        {
            let guard = allocator.0.lock().unwrap();
            assert!(!guard.pool.contains_key(&texture_desc));
        }
    }
}

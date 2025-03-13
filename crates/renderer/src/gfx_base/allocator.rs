use std::{collections::HashMap, rc::Rc};

use super::{AnyFGResource, AnyFGResourceDescriptor, AnyResource};

pub trait ResourceCreator: 'static + Send + Sync {
    fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AllocatorKey {
    desc: AnyFGResourceDescriptor,
    name: String,
}

impl AllocatorKey {
    pub fn new(name: &str, desc: &AnyFGResourceDescriptor) -> Self {
        Self {
            desc: desc.clone(),
            name: name.to_string(),
        }
    }
}

pub struct Allocator {
    pool: HashMap<AllocatorKey, ResourceState>,
    creator: Box<dyn ResourceCreator>,
}

pub struct ResourceState {
    resource: Rc<AnyFGResource>,
    count: usize,
}

impl ResourceState {
    pub fn new(resource: Rc<AnyFGResource>, count: usize) -> Self {
        ResourceState { resource, count }
    }
}

impl Allocator {
    pub fn new<T>(creator: T) -> Self
    where
        T: ResourceCreator,
    {
        Self {
            pool: Default::default(),
            creator: Box::new(creator),
        }
    }

    pub fn alloc(&mut self, name: &str, desc: &AnyFGResourceDescriptor) -> AnyResource {
        let key = AllocatorKey::new(name, desc);

        if let Some(state) = self.pool.get_mut(&key) {
            state.count += 1;
            return AnyResource::new(desc.clone(), state.resource.clone());
        }

        let resource = Rc::new(self.creator.create(desc.clone()));

        let state = ResourceState::new(resource.clone(), 1);

        self.pool.insert(key, state);

        AnyResource {
            desc: desc.clone(),
            resource,
        }
    }

    pub fn free(&mut self, name: &str, resource: AnyResource) {
        let key = AllocatorKey::new(name, &resource.desc);

        if let Some(state) = self.pool.get_mut(&key) {
            state.count -= 1;

            if state.count == 0 {
                self.pool.remove(&key);
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::{Allocator, ResourceCreator};
    use crate::gfx_base::{
        AllocatorKey, AnyFGResource, AnyFGResourceDescriptor, BaseTexture, Texture,
        TextureDescriptor,
    };

    #[derive(Debug)]
    pub struct TestTexture;

    impl BaseTexture for TestTexture {
        fn test(&self) {}
    }

    pub struct TestResourceCreator {}

    impl ResourceCreator for TestResourceCreator {
        fn create(&self, desc: AnyFGResourceDescriptor) -> AnyFGResource {
            match desc {
                AnyFGResourceDescriptor::Texture(_desc) => {
                    AnyFGResource::Texture(Texture::new(TestTexture))
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }

    #[test]
    fn test_allocator() {
        let mut allocator = Allocator::new(TestResourceCreator {});

        let texture_desc = TextureDescriptor::default().into();

        let a = allocator.alloc("a", &texture_desc);
        let b = allocator.alloc("a", &texture_desc);

        assert_eq!(a.resource, b.resource);

        let key = AllocatorKey::new("a", &texture_desc);

        let state = allocator.pool.get(&key).unwrap();
        assert_eq!(state.count, 2);

        allocator.free("a", a);
        allocator.free("a", b);

        assert!(!allocator.pool.contains_key(&key));
    }
}

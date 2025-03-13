use std::{
    marker::PhantomData,
    ops::{Add, Index, IndexMut},
};

const INVALID: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Handle(usize);

impl Handle {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl Default for Handle {
    fn default() -> Self {
        Handle(INVALID)
    }
}

impl Add<usize> for Handle {
    type Output = Handle;

    fn add(self, rhs: usize) -> Self::Output {
        Handle(self.0 + rhs)
    }
}

impl Handle {
    pub fn new(v: usize) -> Handle {
        Handle(v)
    }

    pub fn is_valid(&self) -> bool {
        !self.0 == INVALID
    }
}

impl<T> IndexMut<Handle> for Vec<T> {
    fn index_mut(&mut self, handle: Handle) -> &mut Self::Output {
        &mut self[handle.0]
    }
}

impl<T> Index<Handle> for Vec<T> {
    type Output = T;

    fn index(&self, handle: Handle) -> &Self::Output {
        &self[handle.0]
    }
}

#[derive(Debug)]
pub struct TypedHandle<T> {
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> TypedHandle<T> {
    pub fn handle(&self) -> Handle {
        Handle::new(self.index)
    }

    pub fn new(handle: Handle) -> Self {
        TypedHandle {
            index: handle.0,
            _marker: PhantomData,
        }
    }
}

impl<T> TypedHandle<T> {
    pub fn is_valid(&self) -> bool {
        !self.index == INVALID
    }
}

impl<T> Clone for TypedHandle<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _marker: self._marker,
        }
    }
}

impl<T> Ord for TypedHandle<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> PartialOrd for TypedHandle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl<T> PartialEq for TypedHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self._marker == other._marker
    }
}

impl<T> Eq for TypedHandle<T> {}

impl<ResourceType> TypedHandle<ResourceType> {}

impl<ResourceType> Default for TypedHandle<ResourceType> {
    fn default() -> Self {
        Self {
            index: INVALID,
            _marker: Default::default(),
        }
    }
}

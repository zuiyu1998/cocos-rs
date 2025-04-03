use std::{any::TypeId, fmt::Debug, hash::Hash, marker::PhantomData};

//类型索引
pub struct TypeHandle<T> {
    index: usize,
    _marker: PhantomData<T>,
}

impl<T> Debug for TypeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeHandle")
            .field("index", &self.index)
            .finish()
    }
}

impl<T> Default for TypeHandle<T> {
    fn default() -> Self {
        TypeHandle {
            index: Self::UNINITIALIZED,
            _marker: PhantomData,
        }
    }
}

impl<T> TypeHandle<T> {
    const UNINITIALIZED: usize = usize::MAX;

    pub fn is_valid(&self) -> bool {
        self.index != Self::UNINITIALIZED
    }

    pub fn new(index: usize) -> Self {
        TypeHandle {
            index,
            _marker: PhantomData,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T> PartialEq for TypeHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T: 'static> Hash for TypeHandle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        TypeId::of::<T>().hash(state);
    }
}

impl<T> Copy for TypeHandle<T> {}

impl<T> Eq for TypeHandle<T> {}

impl<T> Clone for TypeHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

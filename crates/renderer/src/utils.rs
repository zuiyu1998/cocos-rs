use std::hash::Hash;

pub struct IndexHandle<T, I> {
    index: I,
    value: T,
}

impl<T, I> IndexHandle<T, I> {
    pub fn new(value: T, index: I) -> Self {
        Self { index, value }
    }
}

pub trait IndexSealed: Hash {}

impl IndexSealed for u32 {}
impl IndexSealed for u16 {}

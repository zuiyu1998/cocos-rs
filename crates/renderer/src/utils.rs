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

impl<T, I> Default for IndexHandle<T, I>
where
    T: Default,
    I: Default,
{
    fn default() -> Self {
        Self {
            index: Default::default(),
            value: Default::default(),
        }
    }
}

pub trait IndexSealed: Hash {}

impl IndexSealed for u32 {}
impl IndexSealed for u16 {}

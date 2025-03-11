use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedHandle<ResourceType> {
    pub index: usize,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> TypedHandle<ResourceType> {
    pub fn is_valid(&self) -> bool {
        !self.index == Self::INVALID
    }
}

impl<ResourceType: Ord> Ord for TypedHandle<ResourceType> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<ResourceType: PartialEq> PartialOrd for TypedHandle<ResourceType> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<ResourceType> TypedHandle<ResourceType> {
    const INVALID: usize = usize::MAX;
}

impl<ResourceType> Default for TypedHandle<ResourceType> {
    fn default() -> Self {
        Self {
            index: Self::INVALID,
            _marker: Default::default(),
        }
    }
}

impl<ResourceType> TypedHandle<ResourceType> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _marker: Default::default(),
        }
    }
}

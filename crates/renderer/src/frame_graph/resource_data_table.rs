use std::{any::Any, fmt::Debug};

use crate::gfx_base::{Handle, TypedHandle};

pub trait ResourceData: Any + Debug + 'static {
    fn as_any_raw(&self) -> &dyn Any;
}

pub trait ResourceDataTable {
    fn get(&self, handle: Handle) -> Option<&dyn ResourceData>;
}

impl dyn ResourceDataTable {
    pub fn get_resource_data<T: ResourceData>(&self, handle: TypedHandle<T>) -> Option<&T> {
        self.get(handle.handle())
            .and_then(|temp| temp.as_any_raw().downcast_ref::<T>())
    }
}

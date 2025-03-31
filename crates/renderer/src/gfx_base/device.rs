use std::fmt::Debug;

pub trait DeviceTrait: 'static + Sync + Send + Debug {}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Debug {}

impl<T> ErasedDeviceTrait for T where T: DeviceTrait {}

#[derive(Debug)]
pub struct Device {
    _value: Box<dyn ErasedDeviceTrait>,
}

impl Device {
    pub fn new<T: DeviceTrait>(value: T) -> Self {
        Device {
            _value: Box::new(value),
        }
    }
}

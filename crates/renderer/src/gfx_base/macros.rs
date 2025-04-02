#[macro_export]
macro_rules! define_atomic_id {
    ($atomic_id_type:ident) => {
        #[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
        pub struct $atomic_id_type(core::num::NonZero<u32>);

        impl $atomic_id_type {
            #[expect(
                clippy::new_without_default,
                reason = "Implementing the `Default` trait on atomic IDs would imply that two `<AtomicIdType>::default()` equal each other. By only implementing `new()`, we indicate that each atomic ID created will be unique."
            )]
            pub fn new() -> Self {
                use core::sync::atomic::{AtomicU32, Ordering};

                static COUNTER: AtomicU32 = AtomicU32::new(1);

                let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
                Self(core::num::NonZero::<u32>::new(counter).unwrap_or_else(|| {
                    panic!(
                        "The system ran out of unique `{}`s.",
                        stringify!($atomic_id_type)
                    );
                }))
            }
        }

        impl From<$atomic_id_type> for core::num::NonZero<u32> {
            fn from(value: $atomic_id_type) -> Self {
                value.0
            }
        }

        impl From<core::num::NonZero<u32>> for $atomic_id_type {
            fn from(value: core::num::NonZero<u32>) -> Self {
                Self(value)
            }
        }
    };
}

#[macro_export]
macro_rules! define_gfx_type {
    ($gfx_type:ident, $gfx_type_type_id: ident, $gfx_type_trait: ident, $erased_gfx_type_trait:ident) => {
        use downcast_rs::impl_downcast;

        #[derive(Debug)]
        pub struct $gfx_type {
            id: $gfx_type_type_id,
            value: Box<dyn $erased_gfx_type_trait>,
        }

        impl_downcast!($erased_gfx_type_trait);

        impl PartialEq for $gfx_type {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl $gfx_type {
            pub fn new<T: $gfx_type_trait>(value: T) -> Self {
                $gfx_type {
                    value: Box::new(value),
                    id: $gfx_type_type_id::new(),
                }
            }

            pub fn downcast_ref<T: $gfx_type_trait>(&self) -> Option<&T> {
                self.value.downcast_ref()
            }

            pub fn downcast<T: $gfx_type_trait>(self) -> Option<Box<T>> {
                self.value.downcast().ok()
            }
        }
    };
}

#[macro_export]
macro_rules! define_gfx_frame_graph_type {
    ($gfx_type:ident, $gfx_type_type_id: ident, $gfx_type_trait: ident, $erased_gfx_type_trait:ident, $desc: ident) => {
        use downcast_rs::impl_downcast;

        #[derive(Debug)]
        pub struct $gfx_type {
            id: $gfx_type_type_id,
            value: Box<dyn $erased_gfx_type_trait>,
            desc: $desc,
        }

        impl_downcast!($erased_gfx_type_trait);

        impl PartialEq for $gfx_type {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl $gfx_type {
            pub fn new<T: $gfx_type_trait>(value: T, desc: $desc) -> Self {
                $gfx_type {
                    value: Box::new(value),
                    id: $gfx_type_type_id::new(),
                    desc,
                }
            }

            pub fn downcast_ref<T: $gfx_type_trait>(&self) -> Option<&T> {
                self.value.downcast_ref()
            }

            pub fn downcast<T: $gfx_type_trait>(self) -> Option<Box<T>> {
                self.value.downcast().ok()
            }

            pub fn get_desc(&self) -> &$desc {
                &self.desc
            }
        }
    };
}

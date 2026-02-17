#[cfg(feature = "alloc")]
mod alloc;
mod core;
mod primitive;
#[cfg(feature = "std")]
mod std;

#[macro_export]
macro_rules! container_derive_impl {
    ($t:ty) => {
        impl<T: Freezable> Freezable for $t {
            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};
                #[allow(deprecated)]
                let mut h = core::hash::SipHasher::new();
                core::any::type_name::<$t>().hash(&mut h);
                T::freeze().hash(&mut h);
                h.finish()
            }
        }
    };
    ($($t:ty),*) => {
        $(container_derive_impl!($t);)*
    }
}

#[macro_export]
macro_rules! generic_derive_impl_no_inner_bound {
    ($t:ty) => {
        impl<T> Freezable for $t {
            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};
                #[allow(deprecated)]
                let mut h = core::hash::SipHasher::new();
                core::any::type_name::<$t>().hash(&mut h);
                h.finish()
            }
        }
    };
    ($($t:ty),*) => {
        $(generic_derive_impl_no_inner_bound!($t);)*
    }
}

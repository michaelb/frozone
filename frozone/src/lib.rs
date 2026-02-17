#![cfg_attr(not(feature = "std"), no_std)]
mod types;

pub use frozone_derive::Freezable;

pub trait Freezable {
    fn freeze() -> u64;
}

#[macro_export]
macro_rules! assume_frozen {
    ($t:ty) => {

        impl Freezable for $t {
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
        $(assume_frozen!($t);)*
    }
}

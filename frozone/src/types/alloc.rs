use crate::{Freezable, assume_frozen};
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

assume_frozen!(String);

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

container_derive_impl!(Box<T>, Vec<T>);

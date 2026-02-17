use crate::{Freezable, assume_frozen, container_derive_impl, generic_derive_impl_no_inner_bound};
use core::any::TypeId;
use core::cell::{Cell, LazyCell, OnceCell, Ref, RefCell, RefMut, UnsafeCell};
use core::cmp::{Ordering, Reverse};
use core::future::{Pending, Ready};
use core::iter::{
    Chain, Cloned, Copied, Cycle, Empty, Enumerate, Filter, Fuse, Inspect, Map, Rev, Skip, StepBy,
    Take, Zip,
};
use core::marker::{PhantomData, PhantomPinned};
use core::mem::{Discriminant, ManuallyDrop, MaybeUninit};
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use core::num::*;
use core::ops::Bound;
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use core::panic::{Location, PanicInfo};
use core::pin::Pin;
use core::ptr::NonNull;
use core::str::{Bytes, CharIndices, Chars, Lines};
use core::sync::atomic::*;

// even if one could argue all phantomdata<T> resolve to the same actual 'type'
container_derive_impl!(
    Option<T>,
    Pin<T>,
    MaybeUninit<T>,
    PhantomData<T>,
    Discriminant<T>,
    ManuallyDrop<T>
);

impl<T: Freezable, E: Freezable> Freezable for Result<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Result<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

assume_frozen!(TypeId, Ordering, PhantomPinned);
container_derive_impl!(Reverse<T>, Pending<T>, Ready<T>);

assume_frozen!(
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroIsize,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
    NonZeroUsize,
    FpCategory
);

container_derive_impl!(
    NonNull<T>,
    Wrapping<T>,
    Cell<T>,
    RefCell<T>,
    UnsafeCell<T>,
    Ref<'_, T>,
    RefMut<'_, T>,
    LazyCell<T>,
    OnceCell<T>
);

impl<T: Freezable> Freezable for (T,) {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<(T,)>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}
impl<T: Freezable, U: Freezable> Freezable for (T, U) {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<(T, U)>().hash(&mut h);
        T::freeze().hash(&mut h);
        U::freeze().hash(&mut h);
        h.finish()
    }
}

#[macro_export]
macro_rules! tuple_derive_impl {
    ($t:ty, ) => {
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
}

macro_rules! tuple_derive_impl {
    ($($ty:ident),*) => {
        impl<$($ty),*> Freezable for ($($ty,)*)
        where
            $($ty: Freezable),*
        {

            fn freeze() -> u64 {
                use core::hash::{Hash, Hasher};
                #[allow(deprecated)]
                let mut h = core::hash::SipHasher::new();
                core::any::type_name::<($($ty,)*)>().hash(&mut h);
                ($($ty::freeze().hash(&mut h),)*);
                h.finish()
            }
        }
    };
}

// macro-generated impls for tuple of size 3 -> 13
tuple_derive_impl!(A, B, C);
tuple_derive_impl!(A, B, C, D);
tuple_derive_impl!(A, B, C, D, E);
tuple_derive_impl!(A, B, C, D, E, F);
tuple_derive_impl!(A, B, C, D, E, F, G);
tuple_derive_impl!(A, B, C, D, E, F, G, H);
tuple_derive_impl!(A, B, C, D, E, F, G, H, I);
tuple_derive_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_derive_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_derive_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_derive_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);

// slice & iterator-related types (only the simplests and most common)
container_derive_impl!(
    core::slice::Iter<'_, T>,
    core::slice::IterMut<'_, T>,
    core::slice::ChunksMut<'_, T>,
    core::slice::Chunks<'_, T>,
    core::slice::Windows<'_, T>
);
container_derive_impl!(
    Cloned<T>,
    Copied<T>,
    Empty<T>,
    Enumerate<T>,
    Rev<T>,
    Take<T>,
    Skip<T>,
    StepBy<T>,
    Fuse<T>,
    Cycle<T>
);
// Flatten and Once have additional requirements, so they're not included
impl<T: Freezable, E: Freezable> Freezable for Chain<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Chain<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable, E: Freezable> Freezable for Filter<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Filter<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable, E: Freezable> Freezable for Inspect<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Inspect<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable, E: Freezable> Freezable for Map<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Map<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable, E: Freezable> Freezable for Zip<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Zip<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}

//core::net types
assume_frozen!(
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6
);

//core::ops
assume_frozen!(RangeFull);
container_derive_impl!(Bound<T>);
generic_derive_impl_no_inner_bound!(
    Range<T>,
    RangeFrom<T>,
    RangeTo<T>,
    RangeInclusive<T>,
    RangeToInclusive<T>
);

assume_frozen!(Location<'_>, PanicInfo<'_>);

// core::str
assume_frozen!(Bytes<'_>, Chars<'_>, CharIndices<'_>, Lines<'_>);

// atomics
assume_frozen!(
    AtomicBool,
    AtomicI8,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicIsize,
    AtomicU8,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicUsize,
    core::sync::atomic::Ordering
);
container_derive_impl!(AtomicPtr<T>);

// future & time
container_derive_impl!(core::task::Poll<T>);
assume_frozen!(
    core::time::Duration,
    core::task::RawWaker,
    core::task::Waker,
    core::task::Context<'_>
);

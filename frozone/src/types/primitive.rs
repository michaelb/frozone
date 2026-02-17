use crate::{Freezable, assume_frozen};
use core::ffi::{CStr, c_void};

assume_frozen!(CStr, c_void);

assume_frozen!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    bool,
    char,
    ()
);

impl<T: Freezable, const N: usize> Freezable for [T; N] {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<[T; N]>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable> Freezable for [T] {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<[T]>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable> Freezable for &T {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<&T>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable> Freezable for *const T {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<*const T>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

// mut and const ptr evaluate to the same (*const's) freeze hash
impl<T: Freezable> Freezable for *mut T {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<*const T>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

impl<T: Freezable> Freezable for &[T] {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<&[T]>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

impl Freezable for &str {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        "&str".hash(&mut h);
        h.finish()
    }
}

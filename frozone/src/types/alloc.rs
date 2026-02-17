use crate::{Freezable, assume_frozen, container_derive_impl};
extern crate alloc;

use alloc::alloc::Layout;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;

assume_frozen!(String, CString, Layout);

container_derive_impl!(
    Box<T>,
    Vec<T>,
    BTreeSet<T>,
    BinaryHeap<T>,
    VecDeque<T>,
    alloc::rc::Rc<T>,
    alloc::rc::Weak<T>,
    alloc::sync::Arc<T>,
    alloc::sync::Weak<T>
);

impl<T: Freezable, E: Freezable> Freezable for BTreeMap<T, E> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<BTreeMap<T, E>>().hash(&mut h);
        T::freeze().hash(&mut h);
        E::freeze().hash(&mut h);
        h.finish()
    }
}
impl<T: Freezable + Clone> Freezable for Cow<'_, T> {
    fn freeze() -> u64 {
        use core::hash::{Hash, Hasher};
        #[allow(deprecated)]
        let mut h = core::hash::SipHasher::new();
        core::any::type_name::<Cow<'_, T>>().hash(&mut h);
        T::freeze().hash(&mut h);
        h.finish()
    }
}

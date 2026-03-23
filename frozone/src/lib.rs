#![cfg_attr(not(feature = "std"), no_std)]
mod types;

pub use frozone_derive::Freezable;
use heapless::Vec;

pub const TYPE_RECURSION_LIMIT: usize = 1024;

#[derive(Debug)]
pub struct FreezeCtx {
    // type, depth where type was found at
    pub cache: Vec<(core::any::TypeId, u32), TYPE_RECURSION_LIMIT>,
    pub depth: u32,
}

impl FreezeCtx {
    fn new() -> Self {
        FreezeCtx {
            cache: Vec::new(),
            depth: 0,
        }
    }
}

pub trait Freezable {
    fn freeze() -> u64 {
        let mut ctx = FreezeCtx::new();
        Self::freeze_with_context(&mut ctx)
    }
    fn freeze_with_context(ctx: &mut FreezeCtx) -> u64;
}

/// internals to reuse from frozone-derive
/// to simplify the code in the proc-macro
pub mod internals {
    pub use super::*;
    pub use core::hash::{Hash, Hasher};

    pub const TYPE_RECURSION_MESSAGE: &str = "exceeded the 1024 nested types limit";

    #[cfg(not(feature = "std"))]
    extern crate alloc;
    #[cfg(not(feature = "std"))]
    pub use alloc::boxed::Box;

    /// "Name and Freeze"-returning function
    pub type NF = Box<dyn Fn(&mut FreezeCtx) -> (&str, u64)>;
    /// "(only) Freeze"-returning function
    pub type F = Box<dyn Fn(&mut FreezeCtx) -> u64>;

    pub fn nf_freeze(x: &NF, ctx: &mut FreezeCtx, acc: u64) -> u64 {
        #[allow(deprecated)]
        let mut hasher = core::hash::SipHasher::new();
        let y = x(ctx);
        y.0.hash(&mut hasher);
        y.1.hash(&mut hasher);
        acc.overflowing_add(hasher.finish()).0
    }

    pub fn f_freeze(x: &NF, ctx: &mut FreezeCtx, acc: u64) -> u64 {
        #[allow(deprecated)]
        let mut hasher = core::hash::SipHasher::new();
        x(ctx).hash(&mut hasher);
        acc.overflowing_add(hasher.finish()).0
    }
}

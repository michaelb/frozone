#![cfg_attr(not(feature = "std"), no_std)]
mod types;

pub use frozone_derive::Freezable;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub const TYPE_RECURSION_LIMIT: usize = 1024;

/// The main trait provided by this crate
/// By deriving this trait on your structures, you can assert they don't
/// change (semantically speaking) from version to version by comparing
/// their `::freeze()` to a known value
pub trait Freezable {
    /// Computes a 'hash of the type, subtypes, field names etc...'
    /// If the freeze doesn't change, you can be sure that the structure
    /// will have the same fields, serialize/deserialize to/from the same string
    /// (assuming your serializer itself doesn't change ofc) ...
    fn freeze() -> u64 {
        let mut ctx = FreezeCtx::default();
        Self::freeze_with_context(&mut ctx)
    }

    /// Useful for debug (e.g inspecting what part of your structure
    /// /sub-structures have changed since last version), this function
    /// prints (std::println!) frozone hashes along the structure's definition
    #[cfg(feature = "std")]
    fn display() {
        let mut ctx = FreezeCtx {
            cache: Vec::new(),
            depth: 0,
            display: true,
        };
        Self::freeze_with_context(&mut ctx);
    }

    /// actual entry point, useful to break frozone but otherwise
    /// shouldn't be used. `::freeze()` is the better choice in 100% of the cases
    fn freeze_with_context(ctx: &mut FreezeCtx) -> u64;
}

#[derive(Debug, Default)]
pub struct FreezeCtx {
    // type, depth where type was found at
    pub cache: Vec<(core::any::TypeId, u32)>,
    pub depth: u32,
    pub display: bool,
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
        #[allow(unused_variables)]
        let (display, depth) = (ctx.display, ctx.depth as usize);
        #[allow(deprecated)]
        let mut hasher = core::hash::SipHasher::new();
        let y = x(ctx);

        #[cfg(feature = "std")]
        if display {
            println!("{:\t<3$} - {} : {:#018x}", "", y.0, y.1, depth - 1);
        }
        y.0.hash(&mut hasher);
        y.1.hash(&mut hasher);
        acc.overflowing_add(hasher.finish()).0
    }

    pub fn f_freeze(x: &F, ctx: &mut FreezeCtx, acc: u64) -> u64 {
        #[allow(unused_variables)]
        let (display, depth) = (ctx.display, ctx.depth as usize);
        #[allow(deprecated)]
        let mut hasher = core::hash::SipHasher::new();
        let y = x(ctx);

        #[cfg(feature = "std")]
        if display {
            println!("{:\t<2$} : {:#x}", "", y, depth);
        }
        y.hash(&mut hasher);
        acc.overflowing_add(hasher.finish()).0
    }
}

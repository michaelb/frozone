#![cfg_attr(not(feature = "std"), no_std)]
mod types;

pub use frozone_derive::Freezable;
use heapless::Vec;

pub struct FreezeCtx {
    // type, depth where type was found at
    pub cache: Vec<(core::any::TypeId, u32), 1024>,
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

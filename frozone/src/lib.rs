#![cfg_attr(not(feature = "std"), no_std)]
mod types;

pub use frozone_derive::Freezable;

pub trait Freezable {
    fn freeze() -> u64;
}

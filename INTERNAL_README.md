# useful debug tools
cargo +nightly expand --test main
cargo +nightly expand --test main > ../../frozone-debug/src/main.rs

(keeping only the test function renamed to main +
```rust
#![feature(prelude_import)]
#![allow(unused)]
#![allow(unexpected_cfgs)]
#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(test)]
#![allow(deprecated)]
extern crate std;
use frozone::Freezable;
#[prelude_import]
use std::prelude::rust_2024::*;
```
)

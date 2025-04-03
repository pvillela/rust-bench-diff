#![doc = include_str!("lib.md")]
//!
//! ## Simple Bench Example
//!
//! This example compares the latencies of functions `f1` and `f2` and prints basic summary statistics for each.
//!
//! ```rust
#![doc = include_str!("../benches/simple1_bench.rs")]
//! ````
//! ## A More Elaborate `print_diff_out` Function
//!
//! The above example can be easily modified to use the following more elaborate `print_diff_out` function. This
//! function prints a broad suite of descriptive and inferential statistics.
//! See the [`DiffOut`] documentation for further details on the available methods.
//!
//! ```rust
#![doc = include_str!("../benches/elaborate_print_diff_out.rs")]
//! ````

#![deny(clippy::unwrap_used)]

mod core;
pub use core::*;

pub mod statistics;

#[doc(hidden)]
pub mod dev_utils;

// intended only to be used by benches
#[doc(hidden)]
#[cfg(feature = "bench")]
pub mod bench_support;

#[doc(hidden)]
#[cfg(feature = "test_support")]
pub mod test_support;

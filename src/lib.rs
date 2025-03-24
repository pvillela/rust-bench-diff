#![doc = include_str!("lib.md")]
#![deny(clippy::unwrap_used)]

mod bench_diff;
pub use bench_diff::*;

mod summary_stats;
pub use summary_stats::*;

pub mod statistics;

#[cfg(feature = "dev_utils")]
pub mod dev_utils;

// intended only to be used by benches
#[cfg(feature = "bench")]
#[cfg(feature = "test_support")]
pub mod bench_support;

#[cfg(feature = "test_support")]
pub mod test_support;

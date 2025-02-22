#![doc = include_str!("lib.md")]
#![deny(clippy::unwrap_used)]

mod bench_diff;
pub use bench_diff::*;

mod summary_stats;
pub use summary_stats::*;

mod statistics;
pub use statistics::*;

#[cfg(feature = "dev_utils")]
pub mod dev_utils;

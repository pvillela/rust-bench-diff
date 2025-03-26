#![doc = include_str!("lib.md")]
#![deny(clippy::unwrap_used)]

mod bench_diff;
pub use bench_diff::*;

mod summary_stats;
pub use summary_stats::*;

pub mod statistics;

#[doc(hidden)]
#[cfg(feature = "dev_utils")]
pub mod dev_utils;

// intended only to be used by benches
#[doc(hidden)]
#[cfg(feature = "bench")]
pub mod bench_support;

#[doc(hidden)]
#[cfg(feature = "test_support")]
pub mod test_support;

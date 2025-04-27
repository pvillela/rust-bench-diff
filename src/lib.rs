#![doc = include_str!("lib1-intro.md")]
//!
//! ```rust
#![doc = include_str!("../benches/simple_bench.rs")]
//! ````
//!
#![doc = include_str!("lib2-details.md")]
#![doc = include_str!("lib3-model.md")]
#![doc = include_str!("lib4-ending.md")]
#![deny(clippy::unwrap_used)]
#![allow(clippy::too_many_arguments)]

mod core;
pub use core::*;

#[doc(hidden)]
pub mod basic_stats;

#[cfg(feature = "_bench")]
pub mod bench_support;

#[doc(hidden)]
pub mod bench_utils;

#[cfg(feature = "_dev_support")]
pub(crate) mod dev_utils;

#[cfg(feature = "_test_support")]
pub(crate) mod test_support;

/// Structs and enums for confidence intervals and hypothesis tests.
pub mod stats_types {
    pub use super::basic_stats::core::{AltHyp, Ci, Hyp, HypTestResult, PositionWrtCi};
}

#[deprecated(since = "1.0.4", note = "use mod `stats_types` instead")]
/// Structs and enums for confidence intervals and hypothesis tests.
pub mod statistics {
    pub use super::stats_types::*;
}

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

pub(crate) mod basic_stats;

#[doc(hidden)]
#[cfg(feature = "_test_support")]
pub mod dev_utils;

// intended only to be used by benches
#[doc(hidden)]
#[cfg(feature = "_bench")]
pub mod bench_support;

#[doc(hidden)]
#[cfg(feature = "_test_support")]
pub mod test_support;

/// Structs and enums for confidence intervals and hypothesis tests.
pub mod stats_types {
    pub use crate::basic_stats::core::{AltHyp, Ci, Hyp, HypTestResult, PositionWrtCi};
}

#[deprecated(since = "1.0.4", note = "use mod `stats_types` instead")]
/// Structs and enums for confidence intervals and hypothesis tests.
pub mod statistics {
    pub use super::stats_types::*;
}

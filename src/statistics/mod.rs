//! Structs and enums for confidence intervals and hypothesis tests.

mod basic;
#[doc(hidden)]
pub use basic::*;

#[allow(unused)]
#[cfg(feature = "dev_support")]
mod wilcoxon;

#[doc(hidden)]
#[cfg(feature = "dev_support")]
pub use wilcoxon::*;

mod types;
pub use types::*;

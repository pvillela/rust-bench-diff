mod basic;
pub use basic::*;

#[cfg(feature = "wilcoxon")]
mod wilcoxon;

#[cfg(feature = "wilcoxon")]
pub use wilcoxon::*;

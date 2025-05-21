mod latency;
pub use latency::*;

#[cfg(feature = "_bench")]
pub mod work_fns;

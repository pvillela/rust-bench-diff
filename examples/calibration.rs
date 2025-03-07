//! Example demonstrating `real_work` and `calibrate_real_work`.
//!
//! To run the example:
//! ```
//! cargo run -r --example calibration --features dev_utils
//! ```

use bench_diff::dev_utils::{calibrate_real_work, real_work};
use bench_diff::latency;
use std::time::Duration;

fn main() {
    let target_latency = Duration::from_nanos(2000);
    let target_effort = calibrate_real_work(target_latency);
    let latency = latency(|| real_work(target_effort));
    let latency_nanos = latency.as_nanos();
    println!("target_effort={}", target_effort);
    println!("target_latency_nanos={}", target_latency.as_nanos());
    println!("latency_nanos={}", latency_nanos);
}

//! Example demonstrating [`busy_work`] and [`calibrate_busy_work`].
//! This example requires the undocumented feature "_dev_support".
//!
//! To run the example:
//! ```
//! cargo run -r --example calibration --features _dev_support
//! ```

use bench_diff::bench_utils::{
    latency,
    work_fns::{busy_work, calibrate_busy_work},
};
use std::time::Duration;

fn main() {
    let target_latency = Duration::from_nanos(2000);
    let target_effort = calibrate_busy_work(target_latency);
    let latency = latency(|| busy_work(target_effort));
    let latency_nanos = latency.as_nanos();
    println!("target_effort={}", target_effort);
    println!("target_latency_nanos={}", target_latency.as_nanos());
    println!("latency_nanos={}", latency_nanos);
}

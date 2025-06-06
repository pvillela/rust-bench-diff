//! Simple benchmark example using [`bench_diff`] and functions defined with [`busy_work`].
//!
//! To run the bench:
//! ```
//! cargo bench --bench busy_bench --features _bench
//! ```

use bench_diff::{
    DiffOut, bench_diff_with_status, bench_support::print_diff_out::comprehensive_print_diff_out,
};
use bench_utils::{LatencyUnit, busy_work, calibrate_busy_work};
use std::time::Duration;

fn f1(effort: u32) {
    busy_work(effort);
}

fn f2(effort: u32) {
    let hi_effort = ((effort as f64) * 1.05) as u32;
    busy_work(hi_effort);
}

fn main() {
    let effort = calibrate_busy_work(Duration::from_micros(100));

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(
            LatencyUnit::Nano,
            || f1(effort),
            || f2(effort),
            1000,
            |_, _| {
                println!("Comparing latency of f1 vs. f2.");
                println!();
            },
        );
        comprehensive_print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(
            LatencyUnit::Nano,
            || f1(effort),
            || f1(effort),
            1000,
            |_, _| {
                println!("Comparing latency of f1 vs. f1.");
                println!();
            },
        );
        comprehensive_print_diff_out(&out);
    }
}

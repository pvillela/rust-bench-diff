//! Simple example benchmark that differs from `simple_bench` by using [`elaborate_print_diff_out::print_diff_out`]
//! to output statistics.
//!
//! To run the bench:
//! ```
//! cargo bench --bench simple_x_bench
//! ```

mod elaborate_print_diff_out;

use bench_diff::{DiffOut, LatencyUnit, bench_diff_with_status};
use elaborate_print_diff_out::print_diff_out;
use std::{thread, time::Duration};

/// This function's latency is at least 100µs.
fn f1() {
    thread::sleep(Duration::from_micros(100));
}

/// This function's latency is at least 105µs.
fn f2() {
    thread::sleep(Duration::from_micros(105));
}

// The difference in latency between these functions should be approximately 5µs.

// Because the magnitude of latencies involved is hundreds of microseconds, it is
// a good idea to use `LatencyUnit::Nano` below. As a rule of thumb, always use
// the closest finer-grained latency unit.

fn main() {
    // Output values are in the selected latench unit, i.e., nanoseconds.

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(LatencyUnit::Nano, f1, f2, 1000, |_, _| {
            println!("Comparing latency of f1 vs. f2.");
            println!();
        });
        print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(LatencyUnit::Nano, f1, f1, 1000, |_, _| {
            println!("Comparing latency of f1 vs. f1.");
            println!();
        });
        print_diff_out(&out);
    }
}

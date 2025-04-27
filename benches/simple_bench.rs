//! Simple example benchmark.
//!
//! To run the bench (assuming the source is at `benches/simple_bench.rs`):
//! ```
//! cargo bench --bench simple_bench
//! ```

use bench_diff::{DiffOut, LatencyUnit, bench_diff_with_status, stats_types::AltHyp};
use std::{thread, time::Duration};

/// This function's latency is at least 21ms.
fn f1() {
    thread::sleep(Duration::from_millis(21));
}

/// This function's latency is at least 20ms.
fn f2() {
    thread::sleep(Duration::from_millis(20));
}

// The latency ratio between these functions should be approximately 1.05.

// Because the magnitude of latencies involved is milliseconds, it is
// a good idea to use `LatencyUnit::Micro` below. As a rule of thumb, always use
// the closest finer-grained latency unit.

fn main() {
    // Output values are in the selected latench unit, i.e., microseconds.

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(LatencyUnit::Micro, f1, f2, 100, |_, _| {
            println!("Comparing latency of f1 vs. f2.");
            println!();
        });
        print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(LatencyUnit::Micro, f1, f1, 100, |_, _| {
            println!("Comparing latency of f1 vs. f1.");
            println!();
        });
        print_diff_out(&out);
    }
}

fn print_diff_out(out: &DiffOut) {
    const ALPHA: f64 = 0.05;

    println!();
    println!("ratio_medians_f1_f2={}", out.ratio_medians_f1_f2(),);
    println!("student_ratio_ci={:?}", out.student_ratio_ci(ALPHA),);
    println!(
        "student_diff_ln_test_gt:{:?}",
        out.student_diff_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("summary_f1={:?}", out.summary_f1());
    println!();
    println!("summary_f2={:?}", out.summary_f2());
    println!();
}

//! Simple benchmark example using [`bench_diff`] and functions defined with [`busy_work`].
//!
//! To run the bench:
//! ```
//! cargo bench --bench busy_bench --features _bench
//! ```

use bench_diff::{DiffOut, bench_diff_with_status, bench_support::comprehensive_print_diff_out};
use bench_utils::{BusyWork, RunLength};
use std::time::Duration;

fn main() {
    let bw = BusyWork::new(Duration::from_micros(100));
    let hi_effort = (bw.effort() as f64 * 1.05) as u32;

    let f1 = bw.fun();
    let f2 = BusyWork::from_effort(hi_effort).fun();

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(&f1, &f2, RunLength::Count(1000), |_| {
            println!("Comparing latency of f1 vs. f2.");
            println!();
        });
        comprehensive_print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_with_status(&f1, &f1, RunLength::Count(1000), |_| {
            println!("Comparing latency of f1 vs. f1.");
            println!();
        });
        comprehensive_print_diff_out(&out);
    }
}

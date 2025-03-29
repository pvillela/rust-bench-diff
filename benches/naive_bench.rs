//! Naive benchmrk example for comparison with `basic_bench`.

use bench_diff::{
    LatencyUnit,
    bench_support::bench_naive::bench_naive,
    dev_utils::{busy_work, calibrate_busy_work},
};

fn main() {
    let unit = LatencyUnit::Nano;
    let base_median = 100_000.0;
    let base_effort = calibrate_busy_work(unit.latency_from_f64(base_median));
    let exec_count = 2_000;

    {
        let name = "hi_5pct_median_no_var";
        let effort = (base_effort as f64 * 1.05) as u32;
        let f = || busy_work(effort);
        let out = bench_naive(LatencyUnit::Nano, f, exec_count);
        println!("\n{} summary: {:?}", name, out.summary_f1());
        println!();
    }

    {
        let name = "base_median_no_var";
        let effort = base_effort;
        let f = || busy_work(effort);
        let out = bench_naive(LatencyUnit::Nano, f, exec_count);
        println!("\n{} summary: {:?}", name, out.summary_f1());
        println!();
    }
}

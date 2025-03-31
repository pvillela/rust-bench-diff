//! Naive benchmrk example for comparison with `basic_bench`.

use bench_diff::{
    LatencyUnit,
    bench_support::bench_basic_naive::{
        Args, bench_naive, get_args, report_median_mean_anomalies, too_close,
    },
    dev_utils::{busy_work, calibrate_busy_work},
};

fn main() {
    let args = get_args();
    println!("args={args:?}");

    let Args {
        target_relative_diff_pct,
        latency_unit,
        base_median,
        exec_count,
    } = args;

    let base_effort = calibrate_busy_work(latency_unit.latency_from_f64(base_median));

    let (median1, mean1) = {
        let name = format!("hi_{}pct_median_no_var", target_relative_diff_pct);
        let effort = (base_effort as f64 * (1. + target_relative_diff_pct as f64 / 100.)) as u32;
        let f = || busy_work(effort);
        let out = bench_naive(LatencyUnit::Nano, f, exec_count);
        let summary = out.summary_f1();
        println!("\n{} summary: {:?}", name, summary);
        println!();

        (summary.median, summary.mean)
    };

    let (median2, mean2) = {
        let name = "base_median_no_var";
        let effort = base_effort;
        let f = || busy_work(effort);
        let out = bench_naive(LatencyUnit::Nano, f, exec_count);
        let summary = out.summary_f1();
        println!("\n{} summary: {:?}", name, summary);
        println!();

        (summary.median, summary.mean)
    };

    report_median_mean_anomalies(
        median1 as f64,
        median2 as f64,
        mean1,
        mean2,
        too_close(target_relative_diff_pct),
    );
}

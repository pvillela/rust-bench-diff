//! Basic `bench_diff`` benchmrk example for comparison with `naive_bench`.

use bench_diff::{DiffOut, statistics::AltHyp};
use bench_diff::{
    bench_diff_print,
    bench_support::bench_basic_naive::{
        Args, ANOMALY_TOLERANCE, get_args, report_median_mean_anomalies,
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

    let name1 = format!("hi_{}pct_median_no_var", target_relative_diff_pct);
    let name2 = "base_median_no_var";

    let f1 = {
        let effort = (base_effort as f64 * (1. + target_relative_diff_pct as f64 / 100.)) as u32;
        move || busy_work(effort)
    };

    let f2 = {
        let effort = base_effort;
        move || busy_work(effort)
    };

    let out = bench_diff_print(latency_unit, f1, f2, exec_count, || {
        println!("\nbench_diff: f1={name1}, f2={name2}")
    });

    print_diff_out(&out);

    let median1 = out.median_f1();
    let median2 = out.median_f2();
    let mean1 = out.mean_f1();
    let mean2 = out.mean_f2();

    report_median_mean_anomalies(
        median1,
        median2,
        mean1,
        mean2,
        target_relative_diff_pct,
        ANOMALY_TOLERANCE,
    );
}

const ALPHA: f64 = 0.05;

pub fn print_diff_out(out: &DiffOut) {
    let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();
    let ratio_medians_f1_f2_from_lns = out.ratio_medians_f1_f2_from_lns();

    println!();
    println!("summary_f1={:?}", out.summary_f1());
    println!();
    println!("summary_f2={:?}", out.summary_f2());
    println!();
    println!(
        "ratio_medians_f1_f2={}, ratio_medians_f1_f2_from_lns={}, diff={}",
        ratio_medians_f1_f2,
        ratio_medians_f1_f2_from_lns,
        ratio_medians_f1_f2 - ratio_medians_f1_f2_from_lns
    );
    println!();
    println!("welch_ratio_ci={:?}", out.welch_ratio_ci(ALPHA),);
    println!(
        "welch_ln_test_gt:{:?}",
        out.welch_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("student_ratio_ci={:?}", out.student_ratio_ci(ALPHA),);
    println!(
        "student_diff_ln_test_gt:{:?}",
        out.student_diff_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!(
        "mean_diff_f1_f2={}, relative_mean_diff_f1_f2={}",
        out.mean_diff_f1_f2(),
        out.mean_diff_f1_f2() / (out.mean_f1() + out.mean_f2()) * 2.
    );
    println!(
        "diff_medians_f1_f2={}, relative_diff_medians_f1_f2={}",
        out.diff_medians_f1_f2(),
        out.diff_medians_f1_f2() / (out.median_f1() + out.median_f2()) * 2.
    );
    println!();
}

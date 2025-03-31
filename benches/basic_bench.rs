//! Basic `bench_diff`` benchmrk example for comparison with `naive_bench`.

use bench_diff::{
    LatencyUnit, bench_diff_print,
    dev_utils::{busy_work, calibrate_busy_work},
};
use std::{env, sync::LazyLock};

fn relative_diff(x: f64, y: f64) -> f64 {
    (x - y) / ((x + y) / 2.0)
}

pub fn get_args() -> u32 {
    const DEFAULT_RELATIVE_DIFF_PCT: u32 = 5;
    let target_relative_diff_pct_str = match env::var("TARGET_RELATIVE_DIFF_PCT") {
        Ok(v) if !v.is_empty() => v,
        _ => return DEFAULT_RELATIVE_DIFF_PCT,
    };
    target_relative_diff_pct_str
        .parse::<u32>()
        .expect(&format!(
            "TARGET_RELATIVE_DIFF_PCT, if provided, must be non-negative integer; was \"{target_relative_diff_pct_str}\""
        ))
}

static TARGET_RELATIVE_DIFF_PCT: LazyLock<u32> = LazyLock::new(|| get_args());
static TOO_CLOSE: LazyLock<f64> =
    LazyLock::new(|| (*TARGET_RELATIVE_DIFF_PCT as f64 / 100.0) * 0.1);

fn main() {
    let unit = LatencyUnit::Nano;
    let base_median = 100_000.0;
    let base_effort = calibrate_busy_work(unit.latency_from_f64(base_median));
    let exec_count = 2_000;

    let name1 = format!("hi_{}pct_median_no_var", *TARGET_RELATIVE_DIFF_PCT);
    let name2 = "base_median_no_var";

    let f1 = {
        let effort = (base_effort as f64 * (1.0 + *TARGET_RELATIVE_DIFF_PCT as f64 / 100.)) as u32;
        move || busy_work(effort)
    };

    let f2 = {
        let effort = base_effort;
        move || busy_work(effort)
    };

    let out = bench_diff_print(unit, f1, f2, exec_count, || {
        println!("\nbench_diff: f1={name1}, f2={name2}")
    });

    print_diff_out(&out);
}

use bench_diff::{DiffOut, statistics::AltHyp};

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
        out.mean_diff_f1_f2() / (out.mean_f1() + out.mean_f2()) * 2.0
    );
    println!(
        "diff_medians_f1_f2={}, relative_diff_medians_f1_f2={}",
        out.diff_medians_f1_f2(),
        out.diff_medians_f1_f2() / (out.median_f1() + out.median_f2()) * 2.0
    );
    println!();

    let median1 = out.median_f1();
    let median2 = out.median_f2();
    let mean1 = out.mean_f1();
    let mean2 = out.mean_f2();
    match () {
        _ if median1 < median2 && mean1 < mean2 => println!("### INVERTED MEAN AND MEDIAN"),
        _ if median1 < median2 => println!("### INVERTED MEDIAN"),
        _ if mean1 < mean2 => println!("### INVERTED MEAN"),
        _ => (),
    }

    let fmedian1 = median1 as f64;
    let fmedian2 = median2 as f64;
    if relative_diff(fmedian1, fmedian2) <= *TOO_CLOSE && relative_diff(mean1, mean2) <= *TOO_CLOSE
    {
        println!("=== TOO CLOSE: MEAN AND MEDIAN")
    }
    if relative_diff(fmedian1, fmedian2) <= *TOO_CLOSE {
        println!("=== TOO CLOSE: MEDIAN")
    }
    if relative_diff(mean1, mean2) <= *TOO_CLOSE {
        println!("=== TOO CLOSE: MEAN")
    }
}

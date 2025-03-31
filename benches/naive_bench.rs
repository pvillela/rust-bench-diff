//! Naive benchmrk example for comparison with `basic_bench`.

use bench_diff::{
    LatencyUnit,
    bench_support::bench_naive::bench_naive,
    dev_utils::{busy_work, calibrate_busy_work},
};
use std::{env, sync::LazyLock};

fn relative_diff(x: f64, y: f64) -> f64 {
    (x - y) / ((x + y) / 2.)
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
static TOO_CLOSE: LazyLock<f64> = LazyLock::new(|| (*TARGET_RELATIVE_DIFF_PCT as f64 / 100.) * 0.1);

fn main() {
    let unit = LatencyUnit::Nano;
    let base_median = 100_000.;
    let base_effort = calibrate_busy_work(unit.latency_from_f64(base_median));
    let exec_count = 2_000;

    let (median1, mean1) = {
        let name = format!("hi_{}pct_median_no_var", *TARGET_RELATIVE_DIFF_PCT);
        let effort = (base_effort as f64 * (1. + *TARGET_RELATIVE_DIFF_PCT as f64 / 100.)) as u32;
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
    } else if relative_diff(fmedian1, fmedian2) <= *TOO_CLOSE {
        println!("=== TOO CLOSE: MEDIAN")
    } else if relative_diff(mean1, mean2) <= *TOO_CLOSE {
        println!("=== TOO CLOSE: MEAN")
    }
}

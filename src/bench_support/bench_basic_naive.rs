//! Functions to support the "naive" comparison benchmarking approach, where each function is benchmarked separately.

use crate::{DiffOut, DiffState};
use bench_utils::{LatencyUnit, latency};
use std::{
    env::{self, VarError},
    io::{Write, stderr, stdout},
    time::{Duration, Instant},
};

const WARMUP_MILLIS: u64 = 3_000;
const WARMUP_INCREMENT_COUNT: usize = 20;

fn execute(
    state: &mut DiffState,
    unit: LatencyUnit,
    mut f: impl FnMut(),
    exec_count: usize,
    pre_exec: impl Fn(),
    mut exec_status: impl FnMut(),
) {
    pre_exec();
    for _ in 1..=exec_count {
        let latency = latency(&mut f);
        let elapsed = unit.latency_as_u64(latency);
        state.capture_data(elapsed, 1);
        exec_status();
    }
}

fn warm_up(
    state: &mut DiffState,
    unit: LatencyUnit,
    mut f: impl FnMut(),
    mut warm_up_status: impl FnMut(usize, u64, u64),
) {
    let start = Instant::now();
    for i in 1.. {
        execute(state, unit, &mut f, WARMUP_INCREMENT_COUNT, || {}, || {});
        let elapsed = Instant::now().duration_since(start);
        warm_up_status(i, elapsed.as_millis() as u64, WARMUP_MILLIS);
        if elapsed.ge(&Duration::from_millis(WARMUP_MILLIS)) {
            break;
        }
    }
}

pub fn bench_naive(unit: LatencyUnit, mut f: impl FnMut(), exec_count: usize) -> DiffOut {
    let mut warm_up_status = {
        let mut status_len: usize = 0;

        move |_: usize, elapsed_millis: u64, warm_up_millis: u64| {
            if status_len == 0 {
                print!("Warming up ... ");
                stdout().flush().expect("unexpected I/O error");
            }
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{elapsed_millis} millis of {warm_up_millis}.");
            if elapsed_millis.lt(&warm_up_millis) {
                status_len = status.len();
            } else {
                status_len = 0;
            };
            eprint!("{status}");
            stderr().flush().expect("unexpected I/O error");
        }
    };

    let pre_exec = || {
        print!(" Executing bench_naive ... ");
        stdout().flush().expect("unexpected I/O error");
    };

    let exec_status = {
        let mut status_len: usize = 0;
        let mut i = 0;

        move || {
            i += 1;
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{i} of {exec_count}.");
            status_len = status.len();
            eprint!("{status}");
            stdout().flush().expect("unexpected I/O error");
        }
    };

    let mut out = DiffOut::new(unit);
    let mut state = DiffState::new(&mut out);
    warm_up(&mut state, unit, &mut f, &mut warm_up_status);
    state.reset();

    execute(&mut state, unit, &mut f, exec_count, pre_exec, exec_status);
    out
}

fn relative_diff(x: f64, y: f64) -> f64 {
    (x - y) / ((x + y) / 2.)
}

#[derive(Debug)]
pub struct Args {
    pub target_relative_diff_pct: u32,
    pub latency_unit: LatencyUnit,
    pub base_median: f64,
    pub exec_count: usize,
}

pub fn get_args() -> Args {
    fn with_default(res: Result<String, VarError>, deflt: &str) -> String {
        match res {
            Ok(s) if !s.is_empty() => s,
            _ => deflt.into(),
        }
    }

    let target_relative_diff_pct_str = with_default(env::var("TARGET_RELATIVE_DIFF_PCT"), "5");
    let target_relative_diff_pct =target_relative_diff_pct_str
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("TARGET_RELATIVE_DIFF_PCT, if provided, must be a non-negative integer; was \"{target_relative_diff_pct_str}\""));

    let latency_unit_str = with_default(env::var("LATENCY_UNIT"), "nano");
    let latency_unit = match latency_unit_str.to_lowercase() {
        s if s == "nano" => LatencyUnit::Nano,
        s if s == "micro" => LatencyUnit::Micro,
        s if s == "milli" => LatencyUnit::Milli,
        s => panic!("invalid LATENCY_UNIT environment variable value: {s}"),
    };

    let base_median_str = with_default(env::var("BASE_MEDIAN"), "100000");
    let base_median = base_median_str.parse::<f64>().unwrap_or_else(|_| {
        panic!("BASE_MEDIAN, if provided, must be a non-negative number; was \"{base_median_str}\"")
    });
    assert!(
        base_median >= 0.,
        "BASE_MEDIAN, if provided, must be a non-negative number; was \"{base_median_str}\""
    );

    let exec_count_str = with_default(env::var("EXEC_COUNT"), "2000");
    let exec_count = exec_count_str.parse::<usize>().unwrap_or_else(|_| {
        panic!("EXEC_COUNT, if provided, must be a non-negative integer; was \"{exec_count_str}\"")
    });

    Args {
        target_relative_diff_pct,
        latency_unit,
        base_median,
        exec_count,
    }
}

fn too_deviant(v1: f64, v2: f64, target_relative_diff_pct: u32, tolerance: f64) -> bool {
    let target_relative_diff = target_relative_diff_pct as f64 / 100.;
    let low = target_relative_diff * (1. - tolerance);
    let high = target_relative_diff * (1. + tolerance);
    let rdiff = relative_diff(v1, v2);
    rdiff <= low || rdiff >= high
}

pub fn report_median_mean_anomalies(
    median1: f64,
    median2: f64,
    mean1: f64,
    mean2: f64,
    target_relative_diff_pct: u32,
    tolerance: f64,
) {
    let rdiff_medians = relative_diff(median1, median2);
    let rdiff_means = relative_diff(mean1, mean2);

    match () {
        _ if median1 < median2 && mean1 < mean2 => println!("### INVERTED MEAN AND MEDIAN"),
        _ if median1 < median2 => println!("### INVERTED MEDIAN"),
        _ if mean1 < mean2 => println!("### INVERTED MEAN"),
        _ => (),
    }

    if too_deviant(median1, median2, target_relative_diff_pct, tolerance)
        && too_deviant(mean1, mean2, target_relative_diff_pct, tolerance)
    {
        println!(
            "=== TOO DEVIANT: MEAN AND MEDIAN (tolerance={tolerance:?}, rdiff_means={rdiff_means:?}, rdiff_medians={rdiff_medians:?})"
        )
    } else if too_deviant(median1, median2, target_relative_diff_pct, tolerance) {
        println!(
            "=== TOO DEVIANT: MEDIAN (tolerance={tolerance:?}, rdiff_medians={rdiff_medians:?})"
        )
    } else if too_deviant(mean1, mean2, target_relative_diff_pct, tolerance) {
        println!("=== TOO DEVIANT: MEAN (tolerance={tolerance:?}, rdiff_means={rdiff_means:?})")
    }
}

pub const ANOMALY_TOLERANCE: f64 = 0.4;

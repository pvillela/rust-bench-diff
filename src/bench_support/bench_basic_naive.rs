use crate::{DiffOut, DiffState, LatencyUnit, latency};
use std::{
    env,
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

    let mut state = DiffState::new();
    warm_up(&mut state, unit, &mut f, &mut warm_up_status);
    state.reset();

    execute(&mut state, unit, &mut f, exec_count, pre_exec, exec_status);
    state
}

fn relative_diff(x: f64, y: f64) -> f64 {
    (x - y) / ((x + y) / 2.)
}

pub struct Args {
    pub target_relative_diff_pct: u32,
}

pub fn get_args() -> Args {
    let target_relative_diff_pct_str = env::var("TARGET_RELATIVE_DIFF_PCT").unwrap_or("5".into());
    let target_relative_diff_pct =target_relative_diff_pct_str
        .parse::<u32>()
        .expect(&format!(
            "TARGET_RELATIVE_DIFF_PCT, if provided, must be non-negative integer; was \"{target_relative_diff_pct_str}\""
        ));

    Args {
        target_relative_diff_pct,
    }
}

pub fn too_close(target_relative_diff_pct: u32) -> f64 {
    (target_relative_diff_pct as f64 / 100.) / 2.
}

pub fn report_median_mean_anomalies(
    median1: f64,
    median2: f64,
    mean1: f64,
    mean2: f64,
    too_close: f64,
) {
    match () {
        _ if median1 < median2 && mean1 < mean2 => println!("### INVERTED MEAN AND MEDIAN"),
        _ if median1 < median2 => println!("### INVERTED MEDIAN"),
        _ if mean1 < mean2 => println!("### INVERTED MEAN"),
        _ => (),
    }

    let fmedian1 = median1;
    let fmedian2 = median2;
    if relative_diff(fmedian1, fmedian2) <= too_close && relative_diff(mean1, mean2) <= too_close {
        println!("=== TOO CLOSE: MEAN AND MEDIAN")
    } else if relative_diff(fmedian1, fmedian2) <= too_close {
        println!("=== TOO CLOSE: MEDIAN")
    } else if relative_diff(mean1, mean2) <= too_close {
        println!("=== TOO CLOSE: MEAN")
    }
}

//! Process that generates noise that can be executed concurrently with benchmarks.

use bench_utils::{BusyWork, latency};
use std::{
    hint::black_box,
    io::{Write, stderr},
    thread,
    time::Duration,
};

fn noise_core(mut f: impl FnMut(), exec_count: usize, mut exec_status: impl FnMut(usize)) {
    for i in 1..=exec_count {
        black_box(latency(&mut f));
        exec_status(i);
    }
}

struct NoiseArgs {
    exec_secs: usize,
    n_threads: usize,
}

fn cmd_line_args() -> NoiseArgs {
    let mut args = std::env::args();

    let throw_err_1 = |v: &str| panic!("*** 1st argument must be positive integer; was \"{v}\"");
    let throw_err_2 = |v: &str| panic!("*** 2nd argument must be positive integer; was \"{v}\"");

    let exec_secs = match args.nth(1) {
        Some(v) if v.ne("--bench") => v.parse::<usize>().unwrap_or_else(|_| throw_err_1(&v)),
        Some(v) => throw_err_1(&v),
        _ => throw_err_1(""),
    };

    let n_threads = match args.next() {
        Some(v) => v.parse::<usize>().unwrap_or_else(|_| throw_err_2(&v)),
        _ => throw_err_2(""),
    };

    NoiseArgs {
        exec_secs,
        n_threads,
    }
}

fn noise(exec_secs: usize, n_threads: usize) {
    const TARGET_LATENCY: Duration = Duration::from_millis(20);
    let exec_count = (exec_secs as f64 / TARGET_LATENCY.as_secs_f64()) as usize;

    eprintln!("exec_secs={exec_secs}, exec_count={exec_count}");

    let bw = BusyWork::new(TARGET_LATENCY);

    let mut exec_status = {
        let mut status_len: usize = 0;

        move |i: usize, n: usize| {
            if status_len == 0 {
                eprintln!(" Executing noise({n})");
                stderr().flush().expect("unexpected I/O error");
            }
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!(" {i} of  {exec_count}. ");
            status_len = status.len();
            eprint!("{status}");
            stderr().flush().expect("unexpected I/O error");
        }
    };

    thread::scope(|s| {
        for n in 0..n_threads {
            let f = bw.fun();
            s.spawn(move || noise_core(f, exec_count, |i| exec_status(i, n)));
        }
    });

    eprintln!();
}

fn main() {
    let NoiseArgs {
        exec_secs,
        n_threads,
    } = cmd_line_args();

    noise(exec_secs, n_threads);
}

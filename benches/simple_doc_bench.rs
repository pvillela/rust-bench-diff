use bench_diff::{DiffOut, LatencyUnit, bench_diff_print};
use std::{thread, time::Duration};

/// This function's latency is at least 100µs.
fn f1() {
    thread::sleep(Duration::from_micros(100));
}

/// This function's latency is at least 105µs.
fn f2() {
    thread::sleep(Duration::from_micros(105));
}

// The difference in latency between these functions should be approximately 5µs.

// Because the magnitude of latencies involved is hundreds of microseconds, it is
// a good idea to use `LatencyUnit::Nano` below. As a rule of thumb, always use
// the closest finer-grained latency unit.

fn main() {
    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_print(LatencyUnit::Nano, f1, f2, 1000, || {
            println!("Comparing latency of f1 vs. f2.")
        });
        print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_print(LatencyUnit::Nano, f1, f1, 1000, || {
            println!("Comparing latency of f1 vs. f1.")
        });
        print_diff_out(&out);
    }
}

fn print_diff_out(out: &DiffOut) {
    println!();
    println!("summary_f1={:?}", out.summary_f1());
    println!();
    println!("summary_f2={:?}", out.summary_f2());
    println!();
}

mod elaborate_print_diff_out;

use bench_diff::{
    DiffOut, LatencyUnit, bench_diff_print,
    dev_utils::{busy_work, calibrate_busy_work},
};
use elaborate_print_diff_out::print_diff_out;
use std::time::Duration;

fn f1(effort: u32) {
    busy_work(effort);
}

fn f2(effort: u32) {
    let hi_effort = ((effort as f64) * 1.05) as u32;
    busy_work(hi_effort);
}

fn main() {
    let effort = calibrate_busy_work(Duration::from_micros(100));

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_print(
            LatencyUnit::Nano,
            || f1(effort),
            || f2(effort),
            1000,
            || println!("Comparing latency of f1 vs. f2."),
        );
        print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_print(
            LatencyUnit::Nano,
            || f1(effort),
            || f1(effort),
            1000,
            || println!("Comparing latency of f1 vs. f1."),
        );
        print_diff_out(&out);
    }
}

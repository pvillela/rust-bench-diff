//! Tests this crate.

mod bench_support;

use bench_diff::dev_utils::{calibrate_real_work, real_work};
use bench_diff::{bench_diff_print, latency, LatencyUnit};
use bench_support::{make_fn_tuple, print_diff_out};

fn main() {
    const EXEC_COUNT: usize = 10_000;
    let unit = LatencyUnit::Nano;
    let target_effort = calibrate_real_work(unit, 1000);
    let latency_nanos = latency(unit, || real_work(target_effort));
    println!("target_effort={}", target_effort);
    println!("latency_nanos={}", latency_nanos);

    let (
        mut base_median_lo_var,
        mut base_median_hi_var,
        mut hi_median_lo_var,
        mut hi_median_hi_var,
    ) = make_fn_tuple();

    bench_diff_print(
        LatencyUnit::Nano,
        &mut base_median_lo_var,
        &mut base_median_hi_var,
        EXEC_COUNT,
        || println!("base_median_lo_var -> base_median_hi_var"),
        print_diff_out,
    );

    bench_diff_print(
        LatencyUnit::Nano,
        &mut base_median_lo_var,
        &mut hi_median_lo_var,
        EXEC_COUNT,
        || println!("base_median_lo_var -> hi_median_lo_var"),
        print_diff_out,
    );

    bench_diff_print(
        LatencyUnit::Nano,
        &mut base_median_lo_var,
        &mut hi_median_hi_var,
        EXEC_COUNT,
        || println!("base_median_lo_var -> hi_median_hi_var"),
        print_diff_out,
    );
}

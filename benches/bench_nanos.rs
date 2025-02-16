//! Tests this crate for the case of nanosecond-scale functions.

mod bench_support;

use bench_diff::LatencyUnit;
use bench_support::{bench, Params};

fn main() {
    let params: Params = Params {
        unit: LatencyUnit::Nano,
        exec_count: 100_000,
        base_median: 400.0,
        hi_median: 440.0,
        lo_stdev_log: 1.25_f64.ln(),
        hi_stdev_log: 2.0_f64.ln(),
    };

    bench(params);
}

//! Tests this crate for the case of microsecond-scale functions.

mod bench_support;

use bench_diff::LatencyUnit;
use bench_support::{bench, Params};

fn main() {
    let params: Params = Params {
        unit: LatencyUnit::Micro,
        exec_count: 10_000,
        base_median: 100.0,
        hi_median: 110.0,
        lo_stdev_log: 1.25_f64.ln(),
        hi_stdev_log: 2.0_f64.ln(),
    };

    bench(params);
}

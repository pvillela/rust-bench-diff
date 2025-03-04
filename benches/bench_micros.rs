//! Tests this crate for the case of microsecond-scale functions.

use bench_diff::LatencyUnit;
use bench_diff::bench_support::{
    Params, bench_t, default_hi_median_ratio, default_hi_stdev_log, default_lo_stdev_log,
};

fn main() {
    let base_median = 100.0;

    let params: Params = Params {
        unit: LatencyUnit::Micro,
        exec_count: 10_000,
        base_median,
        hi_median: base_median * default_hi_median_ratio(),
        lo_stdev_log: default_lo_stdev_log(),
        hi_stdev_log: default_hi_stdev_log(),
    };

    bench_t(params);
}

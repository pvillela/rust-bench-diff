//! Tests this crate for the case of millisecond-scale functions.

use bench_diff::LatencyUnit;
use bench_diff::bench_support::{
    FN_NAME_PAIRS, Params, bench_t, default_hi_median_ratio, default_hi_stdev_log,
    default_lo_stdev_log,
};

fn main() {
    let base_median = 20_000.0;

    let params: Params = Params {
        unit: LatencyUnit::Micro,
        exec_count: 1000,
        base_median,
        hi_median: base_median * default_hi_median_ratio(),
        lo_stdev_log: default_lo_stdev_log(),
        hi_stdev_log: default_hi_stdev_log(),
    };

    bench_t(params, &FN_NAME_PAIRS);
}

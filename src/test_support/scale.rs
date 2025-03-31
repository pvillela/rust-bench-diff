use crate::{
    LatencyUnit,
    test_support::{default_hi_stdev_log, default_lo_stdev_log},
};
use std::sync::LazyLock;

pub struct ScaleParams {
    pub name: String,
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

pub static SCALE_PARAMS: LazyLock<Vec<ScaleParams>> = LazyLock::new(|| {
    vec![
        //
        // Revised params.
        //
        {
            let base_median = 400.;
            ScaleParams {
                name: "nanos_scale".into(),
                unit: LatencyUnit::Nano,
                exec_count: 10_000,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        {
            let base_median = 100_000.;
            ScaleParams {
                name: "micros_scale".into(),
                unit: LatencyUnit::Nano,
                exec_count: 2_000,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        {
            let base_median = 5_000.;
            ScaleParams {
                name: "millis_scale".into(),
                unit: LatencyUnit::Micro,
                exec_count: 200,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        //
        // Original params.
        //
        {
            let base_median = 400.;
            ScaleParams {
                name: "nanos_scale_original".into(),
                unit: LatencyUnit::Nano,
                exec_count: 100_000,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        {
            let base_median = 100_000.;
            ScaleParams {
                name: "micros_scale_original".into(),
                unit: LatencyUnit::Nano,
                exec_count: 10_000,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        {
            let base_median = 10_000.;
            ScaleParams {
                name: "millis_scale_original".into(),
                unit: LatencyUnit::Micro,
                exec_count: 600,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
    ]
});

pub fn get_scale_params(name: &str) -> &ScaleParams {
    let valid_names = SCALE_PARAMS
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>();
    &SCALE_PARAMS
        .iter()
        .find(|pair| pair.name == name)
        .expect(&format!(
            "invalid params name: {name}; valid names are: {valid_names:?}"
        ))
}

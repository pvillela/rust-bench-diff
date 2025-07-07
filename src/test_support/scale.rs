use bench_utils::LatencyUnit;
use std::sync::LazyLock;

pub struct ScaleParams {
    pub name: String,
    pub recording_unit: LatencyUnit,
    pub reporting_unit: LatencyUnit,
    pub exec_count: usize,
    pub base_latency: f64,
}

static SCALE_PARAMS: LazyLock<Vec<ScaleParams>> = LazyLock::new(|| {
    vec![
        //
        // Revised params.
        //
        {
            let base_latency = 400.;
            ScaleParams {
                name: "nanos_scale".into(),
                recording_unit: LatencyUnit::Nano,
                reporting_unit: LatencyUnit::Nano,
                exec_count: 10_000, // was 100_0000
                base_latency,
            }
        },
        {
            let base_latency = 100_000.;
            ScaleParams {
                name: "micros_scale".into(),
                recording_unit: LatencyUnit::Nano,
                reporting_unit: LatencyUnit::Micro,
                exec_count: 2_000, // was 10_000
                base_latency,
            }
        },
        {
            let base_latency = 20_000.; // was 10_000
            ScaleParams {
                name: "millis_scale".into(),
                recording_unit: LatencyUnit::Micro,
                reporting_unit: LatencyUnit::Milli,
                exec_count: 200, // was 600
                base_latency,
            }
        },
    ]
});

pub fn get_scale_params(name: &str) -> &ScaleParams {
    let valid_names = SCALE_PARAMS
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>();
    SCALE_PARAMS
        .iter()
        .find(|pair| pair.name == name)
        .unwrap_or_else(|| panic!("invalid params name: {name}; valid names are: {valid_names:?}"))
}

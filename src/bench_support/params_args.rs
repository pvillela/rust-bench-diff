use crate::{LatencyUnit, PositionInCi, dev_utils::real_work};
use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
use rand_distr::LogNormal;
use std::sync::LazyLock;

pub struct Params {
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub hi_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

pub fn default_hi_median_ratio() -> f64 {
    1.01
}

pub fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.0
}

pub fn default_hi_stdev_log() -> f64 {
    2.4_f64.ln() / 2.0
}

pub(super) enum MyFnMut {
    Constant {
        median_effort: u32,
    },

    Variable {
        median_effort: u32,
        lognormal: LogNormal<f64>,
        rng: StdRng,
    },
}

impl MyFnMut {
    fn new_constant(median_effort: u32) -> Self {
        Self::Constant { median_effort }
    }

    fn new_variable(median_effort: u32, stdev_log: f64) -> Self {
        let mu = 0.0_f64;
        let sigma = stdev_log;
        Self::Variable {
            median_effort,
            lognormal: LogNormal::new(mu, sigma).expect("stdev_log must be > 0"),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    pub(super) fn invoke(&mut self) {
        match self {
            Self::Constant { median_effort } => {
                real_work(*median_effort);
            }

            Self::Variable {
                median_effort,
                lognormal,
                rng,
            } => {
                let factor = lognormal.sample(rng);
                let effort = (*median_effort as f64) * factor;
                real_work(effort as u32);
            }
        }
    }
}

fn make_base_median_no_var(base_effort: u32, _: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_constant(effort)
}

fn make_hi_median_no_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_constant(effort)
}

fn make_base_median_lo_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_hi_median_lo_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_base_median_hi_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
}

fn make_hi_median_hi_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
}

fn cmd_line_args() -> Option<usize> {
    let mut args = std::env::args();

    let nrepeats = match args.nth(1) {
        Some(v) if v.eq("--bench") => return None,
        Some(v) => v
            .parse::<usize>()
            .expect("argument, if provided, must be integer"),
        None => return None,
    };
    Some(nrepeats)
}

pub static FN_NAME_PAIRS: [(&'static str, &'static str); 8] = [
    ("base_median_no_var", "base_median_no_var"),
    ("base_median_no_var", "hi_median_no_var"),
    ("hi_median_no_var", "base_median_no_var"),
    ("base_median_lo_var", "base_median_lo_var"),
    ("base_median_lo_var", "base_median_hi_var"),
    ("base_median_hi_var", "base_median_lo_var"),
    ("base_median_lo_var", "hi_median_lo_var"),
    ("base_median_lo_var", "hi_median_hi_var"),
];
pub static VERBOSE: bool = false;

pub(super) struct ScenarioSpec {
    pub(super) name1: &'static str,
    pub(super) name2: &'static str,
    pub(super) position_in_ci: PositionInCi,
    pub(super) must_pass1: bool,
    pub(super) must_pass2: bool,
    pub(super) must_pass3: bool,
}

impl ScenarioSpec {
    pub(super) const fn new(
        name1: &'static str,
        name2: &'static str,
        position_in_ci: PositionInCi,
        must_pass1: bool,
        must_pass2: bool,
        must_pass3: bool,
    ) -> Self {
        Self {
            name1,
            name2,
            position_in_ci,
            must_pass1,
            must_pass2,
            must_pass3,
        }
    }
}

const NAMED_FNS: [(&str, fn(u32, &Params) -> MyFnMut); 6] = [
    ("base_median_no_var", make_base_median_no_var),
    ("hi_median_no_var", make_hi_median_no_var),
    ("base_median_lo_var", make_base_median_lo_var),
    ("hi_median_lo_var", make_hi_median_lo_var),
    ("base_median_hi_var", make_base_median_hi_var),
    ("hi_median_hi_var", make_hi_median_hi_var),
];
const SCENARIO_SPECS: [ScenarioSpec; 8] = [
    ScenarioSpec::new(
        "base_median_no_var",
        "base_median_no_var",
        PositionInCi::In,
        true,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_no_var",
        "hi_median_no_var",
        PositionInCi::Above,
        true,
        true,
        true,
    ),
    ScenarioSpec::new(
        "hi_median_no_var",
        "base_median_no_var",
        PositionInCi::Below,
        true,
        true,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "base_median_lo_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "base_median_hi_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_hi_var",
        "base_median_lo_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "hi_median_lo_var",
        PositionInCi::Above,
        true,
        true,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "hi_median_hi_var",
        PositionInCi::Above,
        false,
        true,
        false,
    ),
];

pub fn get_fn(name: &str) -> fn(u32, &Params) -> MyFnMut {
    NAMED_FNS
        .iter()
        .find(|pair| pair.0 == name)
        .expect("invalid fn name")
        .1
}

pub fn get_spec(name1: &str, name2: &str) -> &'static ScenarioSpec {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.name1 == name1 && spec.name2 == name2)
        .expect("invalid fn name pair")
}

pub(super) struct Args {
    pub(super) params_idx: usize,
    pub(super) fn_name_pairs: Vec<(String, String)>,
    pub(super) verbose: bool,
    pub(super) nrepeats: usize,
}

pub fn get_args() -> Args {
    let nrepeats = cmd_line_args().unwrap_or(1);
    todo!("get args from environment and command line")
}

pub static PARAMS_VEC: LazyLock<Vec<Params>> = LazyLock::new(|| {
    vec![
        // latency magnitude: nanos
        {
            let base_median = 400.0;
            Params {
                unit: LatencyUnit::Nano,
                exec_count: 100_000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        // latency magnitude: micros
        {
            let base_median = 100_000.0;
            Params {
                unit: LatencyUnit::Nano,
                exec_count: 10_000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        // latency magnitude: millis
        {
            let base_median = 20_000.0;
            Params {
                unit: LatencyUnit::Micro,
                exec_count: 1000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
    ]
});

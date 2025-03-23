//! Definition of target functions, test scenarios, and their parameterization.

use super::scenario::{Claim, Scenario};
use crate::{
    LatencyUnit,
    dev_utils::{busy_work, calibrate_busy_work},
    statistics::{AltHyp, Hyp},
};
use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
use rand_distr::LogNormal;
use std::{env, sync::LazyLock};

pub const ALPHA: f64 = 0.05;
pub const HI_1PCT_FACTOR: f64 = 1.01;
pub const HI_10PCT_FACTOR: f64 = 1.1;
pub const HI_25PCT_FACTOR: f64 = 1.25;

pub fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.0
}

pub fn default_hi_stdev_log() -> f64 {
    2.4_f64.ln() / 2.0
}

pub struct ScaleParams {
    pub name: String,
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

impl ScaleParams {
    pub fn to_calibrated_fn_params(&self) -> CalibratedFnParams {
        let effort = calibrate_busy_work(self.unit.latency_from_f64(self.base_median));
        CalibratedFnParams {
            effort,
            lo_stdev_log: self.lo_stdev_log,
            hi_stdev_log: self.hi_stdev_log,
        }
    }
}

pub struct CalibratedFnParams {
    pub effort: u32,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

pub enum MyFnMut {
    Det {
        median_effort: u32,
    },

    NonDet {
        median_effort: u32,
        lognormal: LogNormal<f64>,
        rng: StdRng,
    },
}

impl MyFnMut {
    fn new_deterministic(median_effort: u32) -> Self {
        Self::Det { median_effort }
    }

    fn new_non_deterministic(median_effort: u32, stdev_log: f64) -> Self {
        let mu = 0.0_f64;
        let sigma = stdev_log;
        Self::NonDet {
            median_effort,
            lognormal: LogNormal::new(mu, sigma).expect("stdev_log must be > 0"),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    pub fn invoke(&mut self) {
        match self {
            Self::Det { median_effort } => {
                busy_work(*median_effort);
            }

            Self::NonDet {
                median_effort,
                lognormal,
                rng,
            } => {
                let factor = lognormal.sample(rng);
                let effort = (*median_effort as f64) * factor;
                busy_work(effort as u32);
            }
        }
    }
}

const NAMED_FNS: [(&str, fn(&CalibratedFnParams) -> MyFnMut); 12] = {
    const fn hi_1pct_effort(c: &CalibratedFnParams) -> u32 {
        (c.effort as f64 * HI_1PCT_FACTOR) as u32
    }

    const fn hi_10pct_effort(c: &CalibratedFnParams) -> u32 {
        (c.effort as f64 * HI_10PCT_FACTOR) as u32
    }

    const fn hi_25pct_effort(c: &CalibratedFnParams) -> u32 {
        (c.effort as f64 * HI_25PCT_FACTOR) as u32
    }

    [
        ("base_median_no_var", |c| {
            MyFnMut::new_deterministic(c.effort)
        }),
        ("hi_1pct_median_no_var", |c| {
            MyFnMut::new_deterministic(hi_1pct_effort(c))
        }),
        ("hi_10pct_median_no_var", |c| {
            MyFnMut::new_deterministic(hi_10pct_effort(c))
        }),
        ("hi_25pct_median_no_var", |c| {
            MyFnMut::new_deterministic(hi_25pct_effort(c))
        }),
        ("base_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(c.effort, c.lo_stdev_log)
        }),
        ("hi_1pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_1pct_effort(c), c.lo_stdev_log)
        }),
        ("hi_10pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_10pct_effort(c), c.lo_stdev_log)
        }),
        ("hi_25pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_25pct_effort(c), c.lo_stdev_log)
        }),
        ("base_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(c.effort, c.hi_stdev_log)
        }),
        ("hi_1pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_1pct_effort(c), c.hi_stdev_log)
        }),
        ("hi_10pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_10pct_effort(c), c.hi_stdev_log)
        }),
        ("hi_25pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_25pct_effort(c), c.hi_stdev_log)
        }),
    ]
};

pub fn get_fn(name: &str) -> fn(&CalibratedFnParams) -> MyFnMut {
    NAMED_FNS
        .iter()
        .find(|pair| pair.0 == name)
        .expect(&format!("invalid fn name: {name}"))
        .1
}

fn claims(accept_hyp: Hyp, target: f64) -> Vec<Claim> {
    vec![
        Claim::welch_ratio_test(accept_hyp),
        Claim::student_diff_test(accept_hyp),
        Claim::student_ratio_test(accept_hyp),
        Claim::wilcoxon_rank_sum_test(accept_hyp),
        Claim::bernoulli_test(accept_hyp),
        //
        Claim::ratio_medians_f1_f2_near_ratio_from_lns(),
        Claim::ratio_medians_f1_f2_near_target(target),
        Claim::target_ratio_medians_f1_f2_in_welch_ratio_ci(target),
        Claim::target_ratio_medians_f1_f2_in_student_ratio_ci(target),
    ]
}

static SCENARIO_SPECS: LazyLock<[Scenario; 14]> = LazyLock::new(|| {
    [
        Scenario::new(
            "base_median_no_var",
            "base_median_no_var",
            claims(Hyp::Null, 1.0),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_1pct_median_no_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.01),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_10pct_median_no_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.1),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_25pct_median_no_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.25),
        ),
        Scenario::new(
            "hi_1pct_median_no_var",
            "base_median_no_var",
            claims(Hyp::Alt(AltHyp::Gt), 1.01),
        ),
        Scenario::new(
            "base_median_lo_var",
            "base_median_lo_var",
            claims(Hyp::Null, 1.0),
        ),
        Scenario::new(
            "base_median_lo_var",
            "base_median_hi_var",
            claims(Hyp::Null, 1.0),
        ),
        Scenario::new(
            "base_median_hi_var",
            "base_median_lo_var",
            claims(Hyp::Null, 1.0),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_1pct_median_lo_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.01),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_10pct_median_lo_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.1),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_25pct_median_lo_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.25),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_1pct_median_hi_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.01),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_10pct_median_hi_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.1),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_25pct_median_hi_var",
            claims(Hyp::Alt(AltHyp::Lt), 1.0 / 1.25),
        ),
    ]
});

pub static FN_NAME_PAIRS: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    SCENARIO_SPECS
        .iter()
        .map(|s| (s.name1, s.name2))
        .collect::<Vec<_>>()
});

pub fn get_spec(name1: &str, name2: &str) -> &'static Scenario {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.name1 == name1 && spec.name2 == name2)
        .expect(&format!(
            "invalid fn name pair: ({name1}, {name2}); valid name pairs are: {FN_NAME_PAIRS:?}"
        ))
}

pub static SCALE_PARAMS: LazyLock<Vec<ScaleParams>> = LazyLock::new(|| {
    vec![
        //
        // Revised params.
        //
        {
            let base_median = 400.0;
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
            let base_median = 100_000.0;
            ScaleParams {
                name: "micros_scale".into(),
                unit: LatencyUnit::Nano,
                exec_count: 1_000,
                base_median,
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        },
        {
            let base_median = 5_000.0;
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
            let base_median = 400.0;
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
            let base_median = 100_000.0;
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
            let base_median = 10_000.0;
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

pub struct Args {
    pub scale_name: String,
    pub fn_name_pairs: Vec<(String, String)>,
    pub verbose: bool,
    pub noise_stats: bool,
    pub nrepeats: usize,
    pub run_name: String,
}

fn cmd_line_args() -> Option<(usize, String)> {
    let mut args = std::env::args();

    let nrepeats = match args.nth(1) {
        Some(v) if v.ne("--bench") => v.parse::<usize>().expect(&format!(
            "1st argument, if provided, must be non-negative integer; was \"{v}\""
        )),
        _ => return None,
    };

    let run_name = match args.next() {
        Some(v) if v.ne("--bench") => v,
        _ => String::new(),
    };

    Some((nrepeats, run_name))
}

pub fn get_args() -> Args {
    let (nrepeats, run_name) = cmd_line_args().unwrap_or((1, "".to_string()));

    let scale_name = env::var("SCALE_NAME").unwrap_or("micros_scale".into());

    let fn_name_pairs: Vec<(String, String)> = {
        let fn_name_pairs_res = env::var("FN_NAME_PAIRS");
        match &fn_name_pairs_res {
            Ok(s) if s == "all" => FN_NAME_PAIRS
                .iter()
                .map(|(name1, name2)| (name1.to_string(), name2.to_string()))
                .collect(),
            Ok(s) => s
                .split_whitespace()
                .map(|x| {
                    let pair_v = x.split("/").collect::<Vec<_>>();
                    let err_msg =
                    "properly formatted function name pair must contain one `/` and no whitespace: "
                        .to_string() + x;
                    assert!(pair_v.len() == 2, "{err_msg}");
                    (pair_v[0].to_string(), pair_v[1].to_string())
                })
                .collect::<Vec<_>>(),
            Err(_) => {
                vec![
                    ("base_median_no_var".into(), "base_median_no_var".into()),
                    ("base_median_no_var".into(), "hi_1pct_median_no_var".into()),
                ]
            }
        }
    };

    let verbose: bool = {
        let verbose_str = env::var("VERBOSE").unwrap_or("true".into());
        verbose_str
            .parse()
            .expect("VERBOSE environment variable has invalid string representation of boolean")
    };

    let noise_stats: bool = {
        let noise_stats_str = env::var("NOISE_STATS").unwrap_or("true".into());
        noise_stats_str
            .parse()
            .expect("NOISE_STATS environment variable has invalid string representation of boolean")
    };

    Args {
        scale_name,
        fn_name_pairs,
        verbose,
        noise_stats,
        nrepeats,
        run_name,
    }
}

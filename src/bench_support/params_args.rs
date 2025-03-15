//! Definition of target functions, test scenarios, and their parameterization.

use super::{Scenario, claim};
use crate::{LatencyUnit, dev_utils::real_work};
use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
use rand_distr::LogNormal;
use std::{env, sync::LazyLock};

pub struct FnParams {
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub hi_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

pub(super) fn default_hi_median_ratio() -> f64 {
    1.01
}

pub(super) fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.0
}

pub(super) fn default_hi_stdev_log() -> f64 {
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

fn make_base_median_no_var(base_effort: u32, _: &FnParams) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_constant(effort)
}

fn make_hi_median_no_var(base_effort: u32, params: &FnParams) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_constant(effort)
}

fn make_base_median_lo_var(base_effort: u32, params: &FnParams) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_hi_median_lo_var(base_effort: u32, params: &FnParams) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_base_median_hi_var(base_effort: u32, params: &FnParams) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
}

fn make_hi_median_hi_var(base_effort: u32, params: &FnParams) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
}

pub(super) static FN_NAME_PAIRS: [(&'static str, &'static str); 8] = [
    ("base_median_no_var", "base_median_no_var"),
    ("base_median_no_var", "hi_median_no_var"),
    ("hi_median_no_var", "base_median_no_var"),
    ("base_median_lo_var", "base_median_lo_var"),
    ("base_median_lo_var", "base_median_hi_var"),
    ("base_median_hi_var", "base_median_lo_var"),
    ("base_median_lo_var", "hi_median_lo_var"),
    ("base_median_lo_var", "hi_median_hi_var"),
];

const NAMED_FNS: [(&str, fn(u32, &FnParams) -> MyFnMut); 6] = [
    ("base_median_no_var", make_base_median_no_var),
    ("hi_median_no_var", make_hi_median_no_var),
    ("base_median_lo_var", make_base_median_lo_var),
    ("hi_median_lo_var", make_hi_median_lo_var),
    ("base_median_hi_var", make_base_median_hi_var),
    ("hi_median_hi_var", make_hi_median_hi_var),
];

pub(super) fn get_fn(name: &str) -> fn(u32, &FnParams) -> MyFnMut {
    NAMED_FNS
        .iter()
        .find(|pair| pair.0 == name)
        .expect(&format!("invalid fn name: {name}"))
        .1
}

static SCENARIO_SPECS: LazyLock<[Scenario; 8]> = LazyLock::new(|| {
    let lt_claims_strict = vec![
        (&claim::WELCH_RATIO_LT_1, true),
        (&claim::STUDENT_DIFF_LT_0, false),
        (&claim::STUDENT_RATIO_LT_1, true),
        (&claim::WILCOXON_RANK_SUM_F1_LT_F2, true),
        (&claim::BERNOULLI_F1_LT_F2, true),
        //
        (&claim::RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI, true),
        (&claim::MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI, true),
    ];

    let lt_claims = vec![
        (&claim::WELCH_RATIO_LT_1, false),
        (&claim::STUDENT_DIFF_LT_0, false),
        (&claim::STUDENT_RATIO_LT_1, false),
        (&claim::WILCOXON_RANK_SUM_F1_LT_F2, false),
        (&claim::BERNOULLI_F1_LT_F2, false),
        //
        (&claim::RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI, true),
        (&claim::MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI, true),
    ];

    let eq_claims_strict = vec![
        (&claim::WELCH_RATIO_EQ_1, true),
        (&claim::STUDENT_DIFF_EQ_0, false),
        (&claim::STUDENT_RATIO_EQ_1, true),
        (&claim::WILCOXON_RANK_SUM_F1_EQ_F2, true),
        (&claim::BERNOULLI_F1_EQ_F2, true),
        //
        (&claim::RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI, true),
        (&claim::MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI, true),
    ];

    let eq_claims = vec![
        (&claim::WELCH_RATIO_EQ_1, false),
        (&claim::STUDENT_DIFF_EQ_0, false),
        (&claim::STUDENT_RATIO_EQ_1, false),
        (&claim::WILCOXON_RANK_SUM_F1_EQ_F2, false),
        (&claim::BERNOULLI_F1_EQ_F2, false),
        //
        (&claim::RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI, true),
        (&claim::MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI, true),
    ];

    let gt_claims_strict = vec![
        (&claim::WELCH_RATIO_GT_1, true),
        (&claim::STUDENT_DIFF_GT_0, false),
        (&claim::STUDENT_RATIO_GT_1, true),
        (&claim::WILCOXON_RANK_SUM_F1_GT_F2, true),
        (&claim::BERNOULLI_F1_GT_F2, true),
        //
        (&claim::RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI, true),
        (&claim::RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI, true),
        (&claim::MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI, true),
    ];

    [
        Scenario::new("base_median_no_var", "base_median_no_var", eq_claims_strict),
        Scenario::new("base_median_no_var", "hi_median_no_var", lt_claims_strict),
        Scenario::new("hi_median_no_var", "base_median_no_var", gt_claims_strict),
        Scenario::new(
            "base_median_lo_var",
            "base_median_lo_var",
            eq_claims.clone(),
        ),
        Scenario::new(
            "base_median_lo_var",
            "base_median_hi_var",
            eq_claims.clone(),
        ),
        Scenario::new("base_median_hi_var", "base_median_lo_var", eq_claims),
        Scenario::new("base_median_lo_var", "hi_median_lo_var", lt_claims.clone()),
        Scenario::new("base_median_lo_var", "hi_median_hi_var", lt_claims),
    ]
});

pub(super) fn get_spec(name1: &str, name2: &str) -> &'static Scenario {
    let valid_name_pairs = SCENARIO_SPECS
        .iter()
        .map(|s| (s.name1, s.name2))
        .collect::<Vec<_>>();
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.name1 == name1 && spec.name2 == name2)
        .expect(&format!(
            "invalid fn name pair: ({name1}, {name2}); valid name pairs are: {valid_name_pairs:?}"
        ))
}

pub(super) static NAMED_PARAMS: LazyLock<Vec<(&'static str, FnParams)>> = LazyLock::new(|| {
    vec![
        // latency magnitude: nanos
        ("nanos_scale", {
            let base_median = 400.0;
            FnParams {
                unit: LatencyUnit::Nano,
                exec_count: 100_000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        }),
        // latency magnitude: micros
        ("micros_scale", {
            let base_median = 100_000.0;
            FnParams {
                unit: LatencyUnit::Nano,
                exec_count: 10_000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        }),
        // latency magnitude: millis
        ("millis_scale", {
            let base_median = 20_000.0;
            FnParams {
                unit: LatencyUnit::Micro,
                exec_count: 1000,
                base_median,
                hi_median: base_median * default_hi_median_ratio(),
                lo_stdev_log: default_lo_stdev_log(),
                hi_stdev_log: default_hi_stdev_log(),
            }
        }),
    ]
});

pub(super) fn get_params(name: &str) -> &FnParams {
    let valid_names = NAMED_PARAMS.iter().map(|p| p.0).collect::<Vec<_>>();
    &NAMED_PARAMS
        .iter()
        .find(|pair| pair.0 == name)
        .expect(&format!(
            "invalid params name: {name}; valid names are: {valid_names:?}"
        ))
        .1
}

pub(super) struct Args {
    pub(super) params_name: String,
    pub(super) fn_name_pairs: Vec<(String, String)>,
    pub(super) verbose: bool,
    pub(super) nrepeats: usize,
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

pub(super) fn get_args() -> Args {
    let nrepeats = cmd_line_args().unwrap_or(1);
    let params_name = env::var("PARAMS_NAME").unwrap_or("micros_scale".into());
    let fn_name_pairs_res = env::var("FN_NAME_PAIRS");
    let verbose_str = env::var("VERBOSE").unwrap_or("true".into());

    let verbose: bool = verbose_str
        .parse()
        .expect("VERBOSE environment variable has invalid string representation of boolean");

    let fn_name_pairs: Vec<(String, String)> = match &fn_name_pairs_res {
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
                ("base_median_no_var".into(), "hi_median_no_var".into()),
            ]
        }
    };

    Args {
        params_name,
        fn_name_pairs,
        verbose,
        nrepeats,
    }
}

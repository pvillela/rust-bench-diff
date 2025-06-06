//! Module that supports test benchmarks, with the definition of target functions, test scenarios,
//! their parameterization, and passing of arguments from environment variables and the command line.

use crate::test_support::{
    FN_NAME_PAIRS, HI_1PCT_FACTOR, HI_10PCT_FACTOR, HI_25PCT_FACTOR, ScaleParams,
};
use bench_utils::{busy_work, calibrate_busy_work};
use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
use rand_distr::LogNormal;
use std::env;

pub fn calibrated_fn_params(s: &ScaleParams) -> CalibratedFnParams {
    let effort = calibrate_busy_work(s.unit.latency_from_f64(s.base_median));
    CalibratedFnParams {
        effort,
        lo_stdev_ln: s.lo_stdev_ln,
        hi_stdev_ln: s.hi_stdev_ln,
    }
}

pub struct CalibratedFnParams {
    pub effort: u32,
    pub lo_stdev_ln: f64,
    pub hi_stdev_ln: f64,
}

#[allow(clippy::large_enum_variant)]
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

    fn new_non_deterministic(median_effort: u32, stdev_ln: f64) -> Self {
        let mu = 0.0_f64;
        let sigma = stdev_ln;
        Self::NonDet {
            median_effort,
            lognormal: LogNormal::new(mu, sigma).expect("stdev_ln must be > 0"),
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

#[allow(clippy::type_complexity)]
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
            MyFnMut::new_non_deterministic(c.effort, c.lo_stdev_ln)
        }),
        ("hi_1pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_1pct_effort(c), c.lo_stdev_ln)
        }),
        ("hi_10pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_10pct_effort(c), c.lo_stdev_ln)
        }),
        ("hi_25pct_median_lo_var", |c| {
            MyFnMut::new_non_deterministic(hi_25pct_effort(c), c.lo_stdev_ln)
        }),
        ("base_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(c.effort, c.hi_stdev_ln)
        }),
        ("hi_1pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_1pct_effort(c), c.hi_stdev_ln)
        }),
        ("hi_10pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_10pct_effort(c), c.hi_stdev_ln)
        }),
        ("hi_25pct_median_hi_var", |c| {
            MyFnMut::new_non_deterministic(hi_25pct_effort(c), c.hi_stdev_ln)
        }),
    ]
};

pub fn get_fn(name: &str) -> fn(&CalibratedFnParams) -> MyFnMut {
    NAMED_FNS
        .iter()
        .find(|pair| pair.0 == name)
        .unwrap_or_else(|| panic!("invalid fn name: {name}"))
        .1
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
        Some(v) if v.ne("--bench") => v.parse::<usize>().unwrap_or_else(|_| {
            panic!("*** 1st argument, if provided, must be non-negative integer; was \"{v}\"")
        }),
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

    let all_fn_name_pairs = || -> Vec<(String, String)> {
        FN_NAME_PAIRS
            .iter()
            .map(|(name1, name2)| (name1.to_string(), name2.to_string()))
            .collect()
    };

    let fn_name_pairs: Vec<(String, String)> = {
        let fn_name_pairs_res = env::var("FN_NAME_PAIRS");
        match &fn_name_pairs_res {
            Ok(s) if s == "all" => all_fn_name_pairs(),
            Ok(s) => s
                .split_whitespace()
                .map(|x| {
                    let pair_v = x.split("/").collect::<Vec<_>>();
                    let err_msg =
                    "*** properly formatted function name pair must contain one `/` and no whitespace: "
                        .to_string() + x;
                    assert!(pair_v.len() == 2, "{err_msg}");
                    (pair_v[0].to_string(), pair_v[1].to_string())
                })
                .collect::<Vec<_>>(),
            Err(_) => all_fn_name_pairs(),
        }
    };

    let verbose: bool = {
        let verbose_str = env::var("VERBOSE").unwrap_or("false".into());
        verbose_str
            .parse()
            .expect("VERBOSE environment variable has invalid string representation of boolean")
    };

    let noise_stats: bool = {
        let noise_stats_str = env::var("NOISE_STATS").unwrap_or("false".into());
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

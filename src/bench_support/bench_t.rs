//! Implementaton of main benchmarking logic to verify [`bench_diff`].

use super::params_args::{Args, FnParams, get_args, get_fn, get_params, get_spec};
use crate::{
    BenchDiffOut, bench_diff, bench_diff_print,
    dev_utils::{ApproxEq, calibrate_real_work},
    statistics::{PositionInCi, SampleMoments, collect_moments},
};
use std::{collections::BTreeMap, fmt::Debug, ops::Deref};

#[derive(Clone)]
pub(super) struct Claim {
    name: &'static str,
    f: fn(&BenchDiffOut) -> Option<String>,
}

pub(super) mod claim {
    use super::*;

    fn eq_result<T: Debug + PartialEq>(expected: T, actual: T) -> Option<String> {
        if expected == actual {
            None
        } else {
            Some(format!("expected={:?}, actual={:?}", expected, actual))
        }
    }

    pub static WELCH_1_IS_BELOW_RATIO_CI: Claim = Claim {
        name: "welch_1_is_below_ratio_ci",
        f: |out: &BenchDiffOut| {
            let expected = PositionInCi::Below;
            let actual = out.welch_position_of_1_in_ratio_ci(ALPHA);
            eq_result(expected, actual)
        },
    };

    pub static WELCH_1_IS_IN_RATIO_CI: Claim = Claim {
        name: "welch_1_is_in_ratio_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.welch_position_of_1_in_ratio_ci(ALPHA);
            let expected = PositionInCi::In;
            eq_result(expected, actual)
        },
    };

    pub static WELCH_1_IS_ABOVE_RATIO_CI: Claim = Claim {
        name: "welch_1_is_above_ratio_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.welch_position_of_1_in_ratio_ci(ALPHA);
            let expected = PositionInCi::Above;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_0_IS_BELOW_DIFF_CI: Claim = Claim {
        name: "student_0_is_below_diff_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_0_in_diff_ci(ALPHA);
            let expected = PositionInCi::Below;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_0_IS_IN_DIFF_CI: Claim = Claim {
        name: "student_0_is_in_diff_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_0_in_diff_ci(ALPHA);
            let expected = PositionInCi::In;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_0_IS_ABOVE_DIFF_CI: Claim = Claim {
        name: "student_0_is_above_diff_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_0_in_diff_ci(ALPHA);
            let expected = PositionInCi::Above;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_1_IS_BELOW_RATIO_CI: Claim = Claim {
        name: "student_1_is_below_ratio_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_1_in_ratio_ci(ALPHA);
            let expected = PositionInCi::Below;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_1_IS_IN_RATIO_CI: Claim = Claim {
        name: "student_1_is_in_ratio_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_1_in_ratio_ci(ALPHA);
            let expected = PositionInCi::In;
            eq_result(expected, actual)
        },
    };

    pub static STUDENT_1_IS_ABOVE_RATIO_CI: Claim = Claim {
        name: "student_1_is_above_ratio_ci",
        f: |out: &BenchDiffOut| {
            let actual = out.student_position_of_1_in_ratio_ci(ALPHA);
            let expected = PositionInCi::Above;
            eq_result(expected, actual)
        },
    };

    pub static RATIO_MEDIANS_F1_F2_NEAR_RATIO_FROM_LNS: Claim = Claim {
        name: "ratio_medians_f1_f2_near_ratio_from_lns",
        f: |out: &BenchDiffOut| {
            let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();
            let ratio_medians_f1_f2_from_lns = out.ratio_medians_f1_f2_from_lns();

            if ratio_medians_f1_f2.approx_eq(ratio_medians_f1_f2_from_lns, 0.005) {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={ratio_medians_f1_f2}, ratio_medians_f1_f2_from_lns={ratio_medians_f1_f2_from_lns}"
                ))
            }
        },
    };

    pub static WILCOXON_RANK_SUM_F1_LT_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_lt_f2",
        f: |out: &BenchDiffOut| {
            let wilcoxon_rank_sum_f1_lt_f2_p = out.wilcoxon_rank_sum_f1_lt_f2_p();
            if wilcoxon_rank_sum_f1_lt_f2_p < ALPHA {
                None
            } else {
                Some(format!(
                    "wilcoxon_rank_sum_f1_lt_f2_p={wilcoxon_rank_sum_f1_lt_f2_p}, ALPHA={ALPHA}"
                ))
            }
        },
    };

    pub static WILCOXON_RANK_SUM_F1_EQ_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_eq_f2",
        f: |out: &BenchDiffOut| {
            let wilcoxon_rank_sum_f1_ne_f2_p = out.wilcoxon_rank_sum_f1_ne_f2_p();
            if wilcoxon_rank_sum_f1_ne_f2_p > ALPHA {
                None
            } else {
                Some(format!(
                    "wilcoxon_rank_sum_f1_ne_f2_p={wilcoxon_rank_sum_f1_ne_f2_p}, ALPHA={ALPHA}"
                ))
            }
        },
    };

    pub static WILCOXON_RANK_SUM_F1_GT_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_gt_f2",
        f: |out: &BenchDiffOut| {
            let wilcoxon_rank_sum_f1_gt_f2_p = out.wilcoxon_rank_sum_f1_gt_f2_p();
            if wilcoxon_rank_sum_f1_gt_f2_p < ALPHA {
                None
            } else {
                Some(format!(
                    "wilcoxon_rank_sum_f1_gt_f2_p={wilcoxon_rank_sum_f1_gt_f2_p}, ALPHA={ALPHA}"
                ))
            }
        },
    };
}

pub(super) struct Scenario {
    pub(super) name1: &'static str,
    pub(super) name2: &'static str,
    pub(super) claims: Vec<(&'static Claim, bool)>,
}

impl Scenario {
    pub(super) const fn new(
        name1: &'static str,
        name2: &'static str,
        claims: Vec<(&'static Claim, bool)>,
    ) -> Self {
        Self {
            name1,
            name2,
            claims,
        }
    }

    pub(super) fn run(&self, diff_out: &BenchDiffOut) -> Vec<ClaimResult> {
        self.claims
            .iter()
            .map(|(claim, must_pass)| ClaimResult {
                scenario_name: format!("fn1={}, fn2={}", self.name1, self.name2),
                claim_name: claim.name,
                result: (claim.f)(diff_out),
                must_pass: *must_pass,
            })
            .collect()
    }
}

#[derive(Debug)]
pub(super) struct ClaimResult {
    scenario_name: String,
    claim_name: &'static str,
    result: Option<String>,
    must_pass: bool,
}

impl ClaimResult {
    fn passed(&self) -> bool {
        self.result.is_none()
    }
}

struct ClaimFailures(Vec<ClaimResult>);

impl ClaimFailures {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push_failure(&mut self, result: ClaimResult) {
        if result.result.is_some() {
            self.0.push(result)
        };
    }

    fn run_scenario(&mut self, scenario: &Scenario, diff_out: &BenchDiffOut) {
        let results = scenario.run(diff_out);
        for result in results {
            self.push_failure(result);
        }
    }

    fn failed_must_pass(&self) -> Vec<&ClaimResult> {
        self.0
            .iter()
            .filter(|cr| !cr.passed() && cr.must_pass)
            .collect()
    }

    fn summary(&self) -> BTreeMap<(String, &'static str), u32> {
        let mut summary = BTreeMap::<(String, &'static str), u32>::new();
        for result in self.iter() {
            let count = summary
                .entry((result.scenario_name.clone(), result.claim_name))
                .or_insert(0);
            *count += 1;
        }
        summary
    }
}

impl Deref for ClaimFailures {
    type Target = Vec<ClaimResult>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const ALPHA: f64 = 0.05;

fn print_diff_out(diff_out: &BenchDiffOut) {
    let ratio_medians_f1_f2 = diff_out.ratio_medians_f1_f2();
    let ratio_medians_f1_f2_from_lns = diff_out.mean_diff_ln_f1_f2().exp();
    let welch_ratio_ci = diff_out.welch_ratio_ci(ALPHA);
    let welch_position_of_1_in_ratio_ci = diff_out.welch_position_of_1_in_ratio_ci(ALPHA);
    let mean_diff_f1_f2 = diff_out.mean_diff_f1_f2();
    let student_diff_ci = diff_out.student_diff_ci(ALPHA);
    let student_position_of_0_in_diff_ci = diff_out.student_position_of_0_in_diff_ci(ALPHA);
    let student_position_of_1_in_ratio_ci = diff_out.student_position_of_1_in_ratio_ci(ALPHA);

    let wilcoxon_rank_sum_f1_lt_f2_p = diff_out.wilcoxon_rank_sum_f1_lt_f2_p();
    let wilcoxon_rank_sum_f1_gt_f2_p = diff_out.wilcoxon_rank_sum_f1_gt_f2_p();

    println!("summary_f1={:?}", diff_out.summary_f1());
    println!("\nsummary_f2={:?}", diff_out.summary_f2());
    println!("\ncount_f1_lt_f2={}", diff_out.count_f1_lt_f2());
    println!("count_f1_eq_f2={}", diff_out.count_f1_eq_f2());
    println!("count_f1_gt_f2={}", diff_out.count_f1_gt_f2());
    println!("ratio_median_f1_f2={}", ratio_medians_f1_f2);
    println!(
        "ratio_medians_f1_f2_from_lns={}",
        ratio_medians_f1_f2_from_lns
    );
    println!(
        "ratio_medians_f1_f2-ratio_medians_f1_f2_from_lns={}",
        ratio_medians_f1_f2 - ratio_medians_f1_f2_from_lns
    );
    println!("welch_ratio_ci={:?}", welch_ratio_ci);
    println!(
        "welch_position_of_1_in_ratio_ci={:?}",
        welch_position_of_1_in_ratio_ci
    );
    println!("mean_diff_f1_f2={}", mean_diff_f1_f2);
    println!("diff_ci={:?}", student_diff_ci);
    println!(
        "student_position_of_0_in_diff_ci={:?}",
        student_position_of_0_in_diff_ci
    );
    println!(
        "student_position_of_1_in_ratio_ci={:?}",
        student_position_of_1_in_ratio_ci
    );

    println!(
        "wilcoxon_rank_sum_f1_lt_f2_p={:?}",
        wilcoxon_rank_sum_f1_lt_f2_p
    );
    println!(
        "wilcoxon_rank_sum_f1_gt_f2_p={:?}",
        wilcoxon_rank_sum_f1_gt_f2_p
    );
    println!();
}

/// Runs benchmarks with statistical t-tests for target functions and comparison scenarios defined by
/// environment variables and command line arguments.
/// Defaults are provided for environment variables and command line arguments not defined.
pub fn bench_diff_claims_with_args() {
    let Args {
        params_name,
        fn_name_pairs,
        verbose,
        nrepeats,
    } = get_args();
    let fn_params = get_params(&params_name);
    bench_diff_with_claims(fn_params, &fn_name_pairs, nrepeats, verbose);
}

/// Runs benchmarks with statistical t-tests for target functions parameterized by `fn_params`,
/// with comparison scenarios defined by `fn_name_pairs`.
pub fn bench_diff_with_claims<T: Deref<Target = str>>(
    fn_params: &FnParams,
    fn_name_pairs: &[(T, T)],
    nrepeats: usize,
    verbose: bool,
) {
    let unit = fn_params.unit;
    let base_effort = calibrate_real_work(unit.latency_from_f64(fn_params.base_median));

    let mut failures = ClaimFailures::new();
    let mut ratio_medians_from_lns_noises =
        BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
    let mut diff_ratio_medians_noises =
        BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
    let mut diff_ln_stdev_noises = BTreeMap::<(&'static str, &'static str), SampleMoments>::new();

    for i in 1..=nrepeats {
        println!("*** iteration = {i} ***");

        for (name1, name2) in fn_name_pairs {
            let scenario_name = format!("f1={}, f2={}", name1.deref(), name2.deref());

            let f1 = {
                let mut my_fn = get_fn(name1)(base_effort, &fn_params);
                move || my_fn.invoke()
            };

            let f2 = {
                let mut my_fn = get_fn(name2)(base_effort, &fn_params);
                move || my_fn.invoke()
            };

            let diff_out = if verbose {
                bench_diff_print(
                    fn_params.unit,
                    f1,
                    f2,
                    fn_params.exec_count,
                    || println!("{scenario_name}"),
                    print_diff_out,
                )
            } else {
                bench_diff(fn_params.unit, f1, f2, fn_params.exec_count)
            };

            let ratio_medians_from_lns_noise = ratio_medians_from_lns_noises
                .entry((name1, name2))
                .or_insert_with(|| SampleMoments::default());
            collect_moments(
                ratio_medians_from_lns_noise,
                diff_out.mean_diff_ln_f1_f2().exp(),
            );

            let diff_ratio_medians_noise = diff_ratio_medians_noises
                .entry((name1, name2))
                .or_insert_with(|| SampleMoments::default());
            collect_moments(
                diff_ratio_medians_noise,
                diff_out.ratio_medians_f1_f2() - diff_out.ratio_medians_f1_f2_from_lns(),
            );

            let diff_ln_stdev_noise = diff_ln_stdev_noises
                .entry((name1, name2))
                .or_insert_with(|| SampleMoments::default());
            collect_moments(diff_ln_stdev_noise, diff_out.stdev_diff_ln_f1_f2());

            let scenario = get_spec(name1, name2);
            failures.run_scenario(scenario, &diff_out);
        }
    }

    println!("*** failures ***");
    for claim_result in failures.iter() {
        println!("{claim_result:?}");
    }

    println!();
    println!("*** failures_summary ***");
    for ((scenario_name, test), count) in failures.summary() {
        println!("{scenario_name} | {test} ==> count={count}");
    }

    println!();
    println!("*** noise ***");
    for (name1, name2) in fn_name_pairs {
        println!();
        println!("scenario: fn1={}, fn2={}", name1.deref(), name2.deref());
        println!(
            "ratio_medians_from_lns_noise_mean={}, ratio_medians_from_lns_noise_stdev={}",
            ratio_medians_from_lns_noises
                .get(&(name1, name2))
                .unwrap()
                .mean(),
            ratio_medians_from_lns_noises
                .get(&(name1, name2))
                .unwrap()
                .stdev()
        );
        println!(
            "diff_ratio_medians_noise_mean={}, diff_ratio_medians_noise_stdev={}, diff_ratio_medians_noise_min={}, diff_ratio_medians_noise_max={}",
            diff_ratio_medians_noises
                .get(&(name1, name2))
                .unwrap()
                .mean(),
            diff_ratio_medians_noises
                .get(&(name1, name2))
                .unwrap()
                .stdev(),
            diff_ratio_medians_noises
                .get(&(name1, name2))
                .unwrap()
                .min(),
            diff_ratio_medians_noises
                .get(&(name1, name2))
                .unwrap()
                .max()
        );
        println!(
            "diff_ln_stdev_noise_mean={}, diff_ln_stdev_noise_stdev={}",
            diff_ln_stdev_noises.get(&(name1, name2)).unwrap().mean(),
            diff_ln_stdev_noises.get(&(name1, name2)).unwrap().stdev()
        );
    }

    let failed_must_pass = failures.failed_must_pass();

    assert!(
        failed_must_pass.len() == 0,
        "{} must-pass tests have failed",
        failed_must_pass.len()
    );
}

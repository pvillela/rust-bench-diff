//! Implementaton of main benchmarking logic to verify [`bench_diff`].

use super::params_args::{Args, FnParams, get_args, get_fn, get_params, get_spec};
use crate::{
    BenchDiffOut, bench_diff, bench_diff_print,
    dev_utils::{ApproxEq, calibrate_real_work},
    statistics::{AltHyp, SampleMoments, collect_moments},
};
use std::{collections::BTreeMap, fmt::Debug, ops::Deref};

#[derive(Clone)]
pub(super) struct Claim {
    name: &'static str,
    f: fn(&BenchDiffOut) -> Option<String>,
}

pub(super) mod claim {
    use super::*;
    use crate::statistics::{AltHyp, Hyp, HypTestResult, PositionWrtCi};

    fn check_hyp_test_result(res: HypTestResult, accept_hyp: Hyp) -> Option<String> {
        if res.accepted() == accept_hyp {
            None
        } else {
            Some(format!(
                "expected to accept {:?} but accepted {:?}: p={:?}, alpha={:?}, alt_hyp:{:?}",
                accept_hyp,
                res.accepted(),
                res.p(),
                res.alpha(),
                res.alt_hyp()
            ))
        }
    }

    pub static WELCH_RATIO_GT_1: Claim = Claim {
        name: "welch_ratio_gt_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.welch_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static WELCH_RATIO_EQ_1: Claim = Claim {
        name: "welch_ratio_eq_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Ne;
            let res = out.welch_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Null)
        },
    };

    pub static WELCH_RATIO_LT_1: Claim = Claim {
        name: "welch_ratio_lt_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Lt;
            let res = out.welch_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static STUDENT_DIFF_GT_0: Claim = Claim {
        name: "student_diff_gt_0",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.student_diff_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static STUDENT_DIFF_EQ_0: Claim = Claim {
        name: "student_diff_eq_0",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.student_diff_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Null)
        },
    };

    pub static STUDENT_DIFF_LT_0: Claim = Claim {
        name: "student_diff_lt_0",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Lt;
            let res = out.student_diff_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static STUDENT_RATIO_GT_1: Claim = Claim {
        name: "student_ratio_gt_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.student_diff_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static STUDENT_RATIO_EQ_1: Claim = Claim {
        name: "student_ratio_eq_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Ne;
            let res = out.student_diff_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Null)
        },
    };

    pub static STUDENT_RATIO_LT_1: Claim = Claim {
        name: "student_ratio_lt_1",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Lt;
            let res = out.student_diff_ln_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
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

    pub static RATIO_MEDIANS_F1_F2_IN_WELCH_RATIO_CI: Claim = Claim {
        name: "ratio_medians_f1_f2_in_welch_ratio_ci",
        f: |out: &BenchDiffOut| {
            let value = out.ratio_medians_f1_f2();
            let ci = out.welch_ratio_ci(ALPHA);

            if PositionWrtCi::position_of_value(value, ci.0, ci.1) == PositionWrtCi::In {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={value}, welch_ratio_ci={ci:?}"
                ))
            }
        },
    };

    pub static RATIO_MEDIANS_F1_F2_IN_STUDENT_RATIO_CI: Claim = Claim {
        name: "ratio_medians_f1_f2_in_student_ratio_ci",
        f: |out: &BenchDiffOut| {
            let value = out.ratio_medians_f1_f2();
            let ci = out.student_ratio_ci(ALPHA);

            if PositionWrtCi::position_of_value(value, ci.0, ci.1) == PositionWrtCi::In {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={value}, student_ratio_ci={ci:?}"
                ))
            }
        },
    };

    pub static MEAN_DIFF_F1_F2_IN_STUDENT_DIFF_CI: Claim = Claim {
        name: "mean_diff_f1_f2_in_student_diff_ci",
        f: |out: &BenchDiffOut| {
            let value = out.mean_diff_f1_f2();
            let ci = out.student_diff_ci(ALPHA);

            if PositionWrtCi::position_of_value(value, ci.0, ci.1) == PositionWrtCi::In {
                None
            } else {
                Some(format!("mean_diff_f1_f2={value}, student_diff_ci={ci:?}"))
            }
        },
    };

    pub static WILCOXON_RANK_SUM_F1_LT_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_lt_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Lt;
            let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static WILCOXON_RANK_SUM_F1_EQ_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_eq_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Ne;
            let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Null)
        },
    };

    pub static WILCOXON_RANK_SUM_F1_GT_F2: Claim = Claim {
        name: "wilcoxon_rank_sum_f1_gt_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static BERNOULLI_F1_GT_F2: Claim = Claim {
        name: "bernoulli_f1_gt_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Gt;
            let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
        },
    };

    pub static BERNOULLI_F1_EQ_F2: Claim = Claim {
        name: "bernoulli_f1_eq_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Ne;
            let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Null)
        },
    };

    pub static BERNOULLI_F1_LT_F2: Claim = Claim {
        name: "bernoulli_f1_lt_f2",
        f: |out: &BenchDiffOut| {
            let alt_hyp = AltHyp::Lt;
            let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
            check_hyp_test_result(res, Hyp::Alt(alt_hyp))
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

    println!();
    println!("summary_f1={:?}", diff_out.summary_f1());
    println!();
    println!("summary_f2={:?}", diff_out.summary_f2());
    println!();
    println!(
        "ratio_median_f1_f2={}, ratio_medians_f1_f2_from_lns={}, diff={}",
        ratio_medians_f1_f2,
        ratio_medians_f1_f2_from_lns,
        ratio_medians_f1_f2 - ratio_medians_f1_f2_from_lns
    );
    println!();
    println!(
        "welch_ratio_ci={:?}, welch_position_of_1_wrt_ratio_ci={:?}",
        diff_out.welch_ratio_ci(ALPHA),
        diff_out.welch_value_position_wrt_ratio_ci(1.0, ALPHA)
    );
    println!(
        "welch_ln_test_lt:{:?}",
        diff_out.welch_ln_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "welch_ln_test_eq:{:?}",
        diff_out.welch_ln_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "welch_ln_test_gt:{:?}",
        diff_out.welch_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!(
        "student_ratio_ci={:?}, student_position_of_1_wrt_ratio_ci={:?}",
        diff_out.student_ratio_ci(ALPHA),
        diff_out.student_value_position_wrt_ratio_ci(1.0, ALPHA)
    );
    println!(
        "student_diff_ln_test_lt:{:?}",
        diff_out.student_diff_ln_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "student_diff_ln_test_eq:{:?}",
        diff_out.student_diff_ln_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "student_diff_ln_test_gt:{:?}",
        diff_out.student_diff_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("mean_diff_f1_f2={}", diff_out.mean_diff_f1_f2());
    println!(
        "diff_ci={:?}, student_position_of_0_wrt_diff_ci={:?}",
        diff_out.student_diff_ci(ALPHA),
        diff_out.student_value_position_wrt_diff_ci(0.0, ALPHA)
    );
    println!(
        "student_diff_test_lt:{:?}",
        diff_out.student_diff_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "student_diff_test_eq:{:?}",
        diff_out.student_diff_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "student_diff_test_gt:{:?}",
        diff_out.student_diff_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!(
        "count_f1_lt_f2={}, count_f1_eq_f2={}, count_f1_gt_f2={}",
        diff_out.count_f1_lt_f2(),
        diff_out.count_f1_eq_f2(),
        diff_out.count_f1_gt_f2()
    );
    println!(
        "bernoulli_prob_f1_gt_f2={:?}",
        diff_out.bernoulli_prob_f1_gt_f2()
    );
    println!(
        "bernoulli_ci={:?}, bernoulli_position_of_half_wrt_ci={:?}",
        diff_out.bernoulli_ci(ALPHA),
        diff_out.bernoulli_value_position_wrt_ci(0.5, ALPHA)
    );
    println!(
        "bernoulli_eq_half_test_lt:{:?}",
        diff_out.bernoulli_eq_half_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "bernoulli_eq_half_test_eq:{:?}",
        diff_out.bernoulli_eq_half_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "bernoulli_eq_half_test_gt:{:?}",
        diff_out.bernoulli_eq_half_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!(
        "wilcoxon_rank_sum_test_lt:{:?}",
        diff_out.wilcoxon_rank_sum_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "wilcoxon_rank_sum_test_eq:{:?}",
        diff_out.wilcoxon_rank_sum_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "wilcoxon_rank_sum_test_gt:{:?}",
        diff_out.wilcoxon_rank_sum_test(AltHyp::Gt, ALPHA)
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

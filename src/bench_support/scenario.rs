//! Definition of [`Scenario`] and [`Claim`] types to support
//! implementaton of main benchmarking logic to verify [`bench_diff`].

use crate::{BenchDiffOut, dev_utils::ApproxEq};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Clone)]
pub enum ClaimRunnable {
    F0(fn(&BenchDiffOut) -> Option<String>),
    F1(fn(&BenchDiffOut, f64) -> Option<String>, f64),
}

impl ClaimRunnable {
    pub fn invoke(&self, out: &BenchDiffOut) -> Option<String> {
        match self {
            Self::F0(f) => f(out),
            Self::F1(f, arg) => f(out, *arg),
        }
    }
}

#[derive(Clone)]
pub struct Claim {
    name: &'static str,
    f: ClaimRunnable,
}

pub mod claim {
    use super::*;
    use crate::{
        bench_support::params_args::ALPHA,
        statistics::{AltHyp, Hyp, HypTestResult, PositionWrtCi},
    };

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

    pub fn welch_ratio_gt_1() -> Claim {
        Claim {
            name: "welch_ratio_gt_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.welch_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn welch_ratio_eq_1() -> Claim {
        Claim {
            name: "welch_ratio_eq_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Ne;
                let res = out.welch_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Null)
            }),
        }
    }

    pub fn welch_ratio_lt_1() -> Claim {
        Claim {
            name: "welch_ratio_lt_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Lt;
                let res = out.welch_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn student_diff_gt_0() -> Claim {
        Claim {
            name: "student_diff_gt_0",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.student_diff_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn student_diff_eq_0() -> Claim {
        Claim {
            name: "student_diff_eq_0",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.student_diff_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Null)
            }),
        }
    }

    pub fn student_diff_lt_0() -> Claim {
        Claim {
            name: "student_diff_lt_0",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Lt;
                let res = out.student_diff_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn student_ratio_gt_1() -> Claim {
        Claim {
            name: "student_ratio_gt_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.student_diff_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn student_ratio_eq_1() -> Claim {
        Claim {
            name: "student_ratio_eq_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Ne;
                let res = out.student_diff_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Null)
            }),
        }
    }

    pub fn student_ratio_lt_1() -> Claim {
        Claim {
            name: "student_ratio_lt_1",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Lt;
                let res = out.student_diff_ln_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn ratio_medians_f1_f2_near_ratio_from_lns() -> Claim {
        Claim {
            name: "ratio_medians_f1_f2_near_ratio_from_lns",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();
                let ratio_medians_f1_f2_from_lns = out.ratio_medians_f1_f2_from_lns();

                if ratio_medians_f1_f2.approx_eq(ratio_medians_f1_f2_from_lns, 0.005) {
                    None
                } else {
                    Some(format!(
                        "ratio_medians_f1_f2={ratio_medians_f1_f2}, ratio_medians_f1_f2_from_lns={ratio_medians_f1_f2_from_lns}"
                    ))
                }
            }),
        }
    }

    pub fn ratio_medians_f1_f2_near_target(target: f64) -> Claim {
        Claim {
            name: "ratio_medians_f1_f2_near_target",
            f: ClaimRunnable::F1(
                |out: &BenchDiffOut, value: f64| {
                    let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();

                    if ratio_medians_f1_f2.approx_eq(value, 0.005) {
                        None
                    } else {
                        Some(format!(
                            "ratio_medians_f1_f2={ratio_medians_f1_f2}, target={value}"
                        ))
                    }
                },
                target,
            ),
        }
    }

    pub fn target_ratio_medians_f1_f2_in_welch_ratio_ci(target: f64) -> Claim {
        Claim {
            name: "target_ratio_medians_f1_f2_in_welch_ratio_ci",
            f: ClaimRunnable::F1(
                |out: &BenchDiffOut, value: f64| {
                    let ci = out.welch_ratio_ci(ALPHA);

                    if PositionWrtCi::position_of_value(value, ci.0, ci.1) == PositionWrtCi::In {
                        None
                    } else {
                        Some(format!(
                            "ratio_medians_f1_f2={value}, welch_ratio_ci={ci:?}"
                        ))
                    }
                },
                target,
            ),
        }
    }

    pub fn target_ratio_medians_f1_f2_in_student_ratio_ci(target: f64) -> Claim {
        Claim {
            name: "target_ratio_medians_f1_f2_in_student_ratio_ci",
            f: ClaimRunnable::F1(
                |out: &BenchDiffOut, value: f64| {
                    let ci = out.student_ratio_ci(ALPHA);

                    if PositionWrtCi::position_of_value(value, ci.0, ci.1) == PositionWrtCi::In {
                        None
                    } else {
                        Some(format!(
                            "ratio_medians_f1_f2={value}, student_ratio_ci={ci:?}"
                        ))
                    }
                },
                target,
            ),
        }
    }

    pub fn wilcoxon_rank_sum_f1_lt_f2() -> Claim {
        Claim {
            name: "wilcoxon_rank_sum_f1_lt_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Lt;
                let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn wilcoxon_rank_sum_f1_eq_f2() -> Claim {
        Claim {
            name: "wilcoxon_rank_sum_f1_eq_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Ne;
                let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Null)
            }),
        }
    }

    pub fn wilcoxon_rank_sum_f1_gt_f2() -> Claim {
        Claim {
            name: "wilcoxon_rank_sum_f1_gt_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.wilcoxon_rank_sum_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn bernoulli_f1_gt_f2() -> Claim {
        Claim {
            name: "bernoulli_f1_gt_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Gt;
                let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }

    pub fn bernoulli_f1_eq_f2() -> Claim {
        Claim {
            name: "bernoulli_f1_eq_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Ne;
                let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Null)
            }),
        }
    }

    pub fn bernoulli_f1_lt_f2() -> Claim {
        Claim {
            name: "bernoulli_f1_lt_f2",
            f: ClaimRunnable::F0(|out: &BenchDiffOut| {
                let alt_hyp = AltHyp::Lt;
                let res = out.bernoulli_eq_half_test(alt_hyp, ALPHA);
                check_hyp_test_result(res, Hyp::Alt(alt_hyp))
            }),
        }
    }
}

pub struct Scenario {
    pub name1: &'static str,
    pub name2: &'static str,
    pub claims: Vec<(Claim, bool)>,
}

impl Scenario {
    pub const fn new(name1: &'static str, name2: &'static str, claims: Vec<(Claim, bool)>) -> Self {
        Self {
            name1,
            name2,
            claims,
        }
    }

    pub fn run(&self, diff_out: &BenchDiffOut) -> Vec<ClaimResult> {
        self.claims
            .iter()
            .map(|(claim, must_pass)| ClaimResult {
                scenario_name: format!("fn1={}, fn2={}", self.name1, self.name2),
                claim_name: claim.name,
                result: claim.f.invoke(diff_out),
                must_pass: *must_pass,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ClaimResult {
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

pub struct ClaimResults {
    pub scenarios_claims: BTreeSet<(String, &'static str)>,
    pub failures: Vec<ClaimResult>,
}

impl ClaimResults {
    pub fn new() -> Self {
        Self {
            scenarios_claims: BTreeSet::new(),
            failures: Vec::new(),
        }
    }

    pub fn push_result(&mut self, result: ClaimResult) {
        self.scenarios_claims
            .insert((result.scenario_name.clone(), result.claim_name));
        if result.result.is_some() {
            self.failures.push(result)
        };
    }

    pub fn run_scenario(&mut self, scenario: &Scenario, diff_out: &BenchDiffOut) {
        let results = scenario.run(diff_out);
        for result in results {
            self.push_result(result);
        }
    }

    pub fn failed_must_pass(&self) -> Vec<&ClaimResult> {
        self.failures
            .iter()
            .filter(|cr| !cr.passed() && cr.must_pass)
            .collect()
    }

    pub fn failure_summary(&self) -> BTreeMap<(String, &'static str), u32> {
        let mut summary = BTreeMap::<(String, &'static str), u32>::new();
        for result in self.failures.iter() {
            let count = summary
                .entry((result.scenario_name.clone(), result.claim_name))
                .or_insert(0);
            *count += 1;
        }
        summary
    }

    pub fn success_summary(&self) -> BTreeSet<(String, &'static str)> {
        let failure_keys: BTreeSet<(String, &'static str)> = self
            .failure_summary()
            .keys()
            .map(|(s, c)| (s.clone(), *c))
            .collect();
        self.scenarios_claims
            .difference(&failure_keys)
            .cloned()
            .collect()
    }
}

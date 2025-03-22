//! Definition of [`Scenario`] and [`Claim`] types to support
//! implementaton of main benchmarking logic to verify [`bench_diff`].

use crate::{
    BenchDiffOut,
    bench_support::params_args::ALPHA,
    dev_utils::ApproxEq,
    statistics::{Hyp, HypTestResult, PositionWrtCi},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Clone)]
enum ClaimFn {
    Nullary(fn(&BenchDiffOut) -> Option<String>),
    Hyp(fn(&BenchDiffOut, Hyp) -> Option<String>, Hyp),
    Target(fn(&BenchDiffOut, f64) -> Option<String>, f64),
}

impl ClaimFn {
    fn invoke(&self, out: &BenchDiffOut) -> Option<String> {
        match self {
            Self::Nullary(f) => f(out),
            Self::Hyp(f, accept_hyp) => f(out, *accept_hyp),
            Self::Target(f, arg) => f(out, *arg),
        }
    }
}

#[derive(Clone)]
pub struct Claim {
    name: &'static str,
    f: ClaimFn,
}

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

impl Claim {
    pub fn invoke(&self, out: &BenchDiffOut) -> Option<String> {
        self.f.invoke(out)
    }

    pub fn welch_ratio_test(accept_hyp: Hyp) -> Claim {
        Claim {
            name: "welch_ratio_test",
            f: ClaimFn::Hyp(
                |out: &BenchDiffOut, accept_hyp: Hyp| {
                    let res = out.welch_ln_test(accept_hyp.alt_hyp(), ALPHA);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
            ),
        }
    }

    pub fn student_diff_test(accept_hyp: Hyp) -> Claim {
        Claim {
            name: "student_diff_test",
            f: ClaimFn::Hyp(
                |out: &BenchDiffOut, accept_hyp: Hyp| {
                    let res = out.student_diff_test(accept_hyp.alt_hyp(), ALPHA);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
            ),
        }
    }

    pub fn student_ratio_test(accept_hyp: Hyp) -> Claim {
        Claim {
            name: "student_ratio_test",
            f: ClaimFn::Hyp(
                |out: &BenchDiffOut, accept_hyp: Hyp| {
                    let res = out.student_diff_ln_test(accept_hyp.alt_hyp(), ALPHA);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
            ),
        }
    }

    pub fn ratio_medians_f1_f2_near_ratio_from_lns() -> Claim {
        Claim {
            name: "ratio_medians_f1_f2_near_ratio_from_lns",
            f: ClaimFn::Nullary(|out: &BenchDiffOut| {
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
            f: ClaimFn::Target(
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
            f: ClaimFn::Target(
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
            f: ClaimFn::Target(
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

    pub fn wilcoxon_rank_sum_test(accept_hyp: Hyp) -> Claim {
        Claim {
            name: "wilcoxon_rank_sum_test",
            f: ClaimFn::Hyp(
                |out: &BenchDiffOut, accept_hyp: Hyp| {
                    let res = out.wilcoxon_rank_sum_test(accept_hyp.alt_hyp(), ALPHA);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
            ),
        }
    }

    pub fn bernoulli_test(accept_hyp: Hyp) -> Claim {
        Claim {
            name: "bernoulli_test",
            f: ClaimFn::Hyp(
                |out: &BenchDiffOut, accept_hyp: Hyp| {
                    let res = out.bernoulli_eq_half_test(accept_hyp.alt_hyp(), ALPHA);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
            ),
        }
    }
}

pub struct Scenario {
    pub name1: &'static str,
    pub name2: &'static str,
    pub claims: Vec<Claim>,
}

impl Scenario {
    pub const fn new(name1: &'static str, name2: &'static str, claims: Vec<Claim>) -> Self {
        Self {
            name1,
            name2,
            claims,
        }
    }

    pub fn run(&self, diff_out: &BenchDiffOut) -> Vec<ClaimResult> {
        self.claims
            .iter()
            .map(|claim| ClaimResult {
                scenario_name: format!("fn1={}, fn2={}", self.name1, self.name2),
                claim_name: claim.name,
                result: claim.invoke(diff_out),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ClaimResult {
    scenario_name: String,
    claim_name: &'static str,
    result: Option<String>,
}

pub struct ClaimResults {
    failures: Vec<ClaimResult>,
    summary: BTreeMap<(String, &'static str), u32>,
}

impl ClaimResults {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            summary: BTreeMap::new(),
        }
    }

    pub fn push_result(&mut self, result: ClaimResult, verbose: bool) {
        let value = self
            .summary
            .entry((result.scenario_name.clone(), result.claim_name))
            .or_insert(0);
        if result.result.is_some() {
            *value += 1;
            if verbose {
                self.failures.push(result);
            }
        };
    }

    pub fn run_scenario(&mut self, scenario: &Scenario, diff_out: &BenchDiffOut, verbose: bool) {
        let results = scenario.run(diff_out);
        for result in results {
            self.push_result(result, verbose);
        }
    }

    pub fn summary(&self) -> &BTreeMap<(String, &'static str), u32> {
        &self.summary
    }

    pub fn failures(&self) -> &Vec<ClaimResult> {
        &self.failures
    }

    pub fn failure_summary(&self) -> BTreeMap<(String, &'static str), u32> {
        self.summary
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    pub fn success_summary(&self) -> BTreeSet<(String, &'static str)> {
        self.summary
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k.clone())
            .collect()
    }
}

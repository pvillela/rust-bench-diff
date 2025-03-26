use crate::{
    DiffOut,
    dev_utils::ApproxEq,
    statistics::{Hyp, HypTestResult, PositionWrtCi},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Clone)]
enum ClaimFn {
    Nullary(fn(&DiffOut) -> Option<String>),
    Hyp(fn(&DiffOut, Hyp, f64) -> Option<String>, Hyp, f64),
    Arity1(fn(&DiffOut, f64) -> Option<String>, f64),
    Arity2(fn(&DiffOut, f64, f64) -> Option<String>, f64, f64),
}

impl ClaimFn {
    fn invoke(&self, out: &DiffOut) -> Option<String> {
        match self {
            Self::Nullary(f) => f(out),
            Self::Hyp(f, accept_hyp, alpha) => f(out, *accept_hyp, *alpha),
            Self::Arity1(f, arg) => f(out, *arg),
            Self::Arity2(f, arg1, arg2) => f(out, *arg1, *arg2),
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
    pub fn invoke(&self, out: &DiffOut) -> Option<String> {
        self.f.invoke(out)
    }

    pub fn welch_ratio_test(accept_hyp: Hyp, alpha: f64) -> Claim {
        Claim {
            name: "welch_ratio_test",
            f: ClaimFn::Hyp(
                |out: &DiffOut, accept_hyp: Hyp, alpha: f64| {
                    let res = out.welch_ln_test(accept_hyp.alt_hyp(), alpha);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
                alpha,
            ),
        }
    }

    pub fn student_diff_test(accept_hyp: Hyp, alpha: f64) -> Claim {
        Claim {
            name: "student_diff_test",
            f: ClaimFn::Hyp(
                |out: &DiffOut, accept_hyp: Hyp, alpha: f64| {
                    let res = out.student_diff_test(accept_hyp.alt_hyp(), alpha);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
                alpha,
            ),
        }
    }

    pub fn student_ratio_test(accept_hyp: Hyp, alpha: f64) -> Claim {
        Claim {
            name: "student_ratio_test",
            f: ClaimFn::Hyp(
                |out: &DiffOut, accept_hyp: Hyp, alpha: f64| {
                    let res = out.student_diff_ln_test(accept_hyp.alt_hyp(), alpha);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
                alpha,
            ),
        }
    }

    pub fn ratio_medians_f1_f2_near_ratio_from_lns() -> Claim {
        Claim {
            name: "ratio_medians_f1_f2_near_ratio_from_lns",
            f: ClaimFn::Nullary(|out: &DiffOut| {
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
            f: ClaimFn::Arity1(
                |out: &DiffOut, value: f64| {
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

    pub fn target_ratio_medians_f1_f2_in_welch_ratio_ci(target: f64, alpha: f64) -> Claim {
        Claim {
            name: "target_ratio_medians_f1_f2_in_welch_ratio_ci",
            f: ClaimFn::Arity2(
                |out: &DiffOut, value: f64, alpha: f64| {
                    let ci = out.welch_ratio_ci(alpha);

                    if ci.position_of(value) == PositionWrtCi::In {
                        None
                    } else {
                        Some(format!(
                            "ratio_medians_f1_f2={value}, welch_ratio_ci={ci:?}"
                        ))
                    }
                },
                target,
                alpha,
            ),
        }
    }

    pub fn target_ratio_medians_f1_f2_in_student_ratio_ci(target: f64, alpha: f64) -> Claim {
        Claim {
            name: "target_ratio_medians_f1_f2_in_student_ratio_ci",
            f: ClaimFn::Arity2(
                |out: &DiffOut, value: f64, alpha: f64| {
                    let ci = out.student_ratio_ci(alpha);

                    if ci.position_of(value) == PositionWrtCi::In {
                        None
                    } else {
                        Some(format!(
                            "ratio_medians_f1_f2={value}, student_ratio_ci={ci:?}"
                        ))
                    }
                },
                target,
                alpha,
            ),
        }
    }

    pub fn wilcoxon_rank_sum_test(accept_hyp: Hyp, alpha: f64) -> Claim {
        Claim {
            name: "wilcoxon_rank_sum_test",
            f: ClaimFn::Hyp(
                |out: &DiffOut, accept_hyp: Hyp, alpha: f64| {
                    let res = out.wilcoxon_rank_sum_test(accept_hyp.alt_hyp(), alpha);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
                alpha,
            ),
        }
    }

    pub fn bernoulli_test(accept_hyp: Hyp, alpha: f64) -> Claim {
        Claim {
            name: "bernoulli_test",
            f: ClaimFn::Hyp(
                |out: &DiffOut, accept_hyp: Hyp, alpha: f64| {
                    let res = out.bernoulli_eq_half_test(accept_hyp.alt_hyp(), alpha);
                    check_hyp_test_result(res, accept_hyp)
                },
                accept_hyp,
                alpha,
            ),
        }
    }

    pub fn claims(accept_hyp: Hyp, target: f64, alpha: f64) -> Vec<Claim> {
        vec![
            Claim::welch_ratio_test(accept_hyp, alpha),
            Claim::student_diff_test(accept_hyp, alpha),
            Claim::student_ratio_test(accept_hyp, alpha),
            Claim::wilcoxon_rank_sum_test(accept_hyp, alpha),
            Claim::bernoulli_test(accept_hyp, alpha),
            //
            Claim::ratio_medians_f1_f2_near_ratio_from_lns(),
            Claim::ratio_medians_f1_f2_near_target(target),
            Claim::target_ratio_medians_f1_f2_in_welch_ratio_ci(target, alpha),
            Claim::target_ratio_medians_f1_f2_in_student_ratio_ci(target, alpha),
        ]
    }

    pub const CRITICAL_NAMES: [&'static str; 4] = [
        "welch_ratio_test",
        // "student_diff_test",
        "student_ratio_test",
        // "wilcoxon_rank_sum_test",
        // "bernoulli_test",
        "target_ratio_medians_f1_f2_in_welch_ratio_ci",
        "target_ratio_medians_f1_f2_in_student_ratio_ci",
    ];
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClaimResult {
    name1: &'static str,
    name2: &'static str,
    claim_name: &'static str,
    result: Option<String>,
}

pub struct ClaimResults {
    failures: Vec<ClaimResult>,
    summary: BTreeMap<((&'static str, &'static str), &'static str), u32>,
}

impl ClaimResults {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            summary: BTreeMap::new(),
        }
    }

    pub fn push_claim(
        &mut self,
        name1: &'static str,
        name2: &'static str,
        claim: &Claim,
        diff_out: &DiffOut,
        verbose: bool,
    ) {
        let value = self
            .summary
            .entry(((name1, name2), claim.name))
            .or_insert(0);
        let result = claim.invoke(diff_out);
        if result.is_some() {
            *value += 1;
            if verbose {
                self.failures.push(ClaimResult {
                    name1,
                    name2,
                    claim_name: claim.name,
                    result,
                });
            }
        };
    }

    pub fn summary(&self) -> &BTreeMap<((&'static str, &'static str), &'static str), u32> {
        &self.summary
    }

    pub fn failures(&self) -> &Vec<ClaimResult> {
        &self.failures
    }

    pub fn failure_summary(&self) -> BTreeMap<((&'static str, &'static str), &'static str), u32> {
        self.summary
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    pub fn success_summary(&self) -> BTreeSet<((&'static str, &'static str), &'static str)> {
        self.summary
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn type_i_and_ii_errors(
        &self,
        alpha: f64,
        beta: f64,
        claim_names: &[&'static str],
        nrepeats: usize,
    ) -> BTreeMap<((&'static str, &'static str), &'static str), u32> {
        const ALPHA_COUNT_ALLOWANCE: f64 = 0.30;
        const BETA_COUNT_ALLOWANCE: f64 = 0.20;
        let alpha_count = (nrepeats as f64 * alpha * (1.0 + ALPHA_COUNT_ALLOWANCE)).ceil() as u32;
        let beta_count = (nrepeats as f64 * beta * (1.0 + BETA_COUNT_ALLOWANCE)).ceil() as u32;

        let predicate = |name1: &'static str,
                         name2: &'static str,
                         claim_name: &'static str,
                         count: u32|
         -> bool {
            match (name1, name2, claim_name, count) {
                _ if name1[..5] == name2[..5]
                    && claim_names.contains(&claim_name)
                    && count > alpha_count =>
                {
                    true
                }

                _ if name1[..5] != name2[..5]
                    && claim_names.contains(&claim_name)
                    && count > beta_count =>
                {
                    true
                }

                _ => false,
            }
        };

        self.summary
            .iter()
            .filter(|(((name1, name2), claim_name), count)| {
                predicate(name1, name2, claim_name, **count)
            })
            .map(|(k, v)| (k.clone(), *v))
            .collect::<BTreeMap<_, _>>()
    }
}

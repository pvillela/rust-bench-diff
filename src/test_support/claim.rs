use super::binomial_inv_cdf;
use crate::{
    DiffOut,
    dev_utils::ApproxEq,
    stats_types::{AcceptedHyp, AltHyp, HypTestResult, PositionWrtCi},
    test_support::FnSpec,
};
use bench_utils::Comp;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

type Hyp = Option<AltHyp>;

fn alt_hyp(hyp: Option<AltHyp>) -> AltHyp {
    match hyp {
        Some(alt) => alt,
        None => AltHyp::Ne,
    }
}

fn accepted_hyp(hyp: Option<AltHyp>) -> AcceptedHyp {
    match hyp {
        Some(_) => AcceptedHyp::Alt,
        None => AcceptedHyp::Null,
    }
}

fn check_hyp_test_result(res: HypTestResult, hyp: Hyp) -> Option<String> {
    let exp_accepted = accepted_hyp(hyp);
    if res.accepted() == exp_accepted {
        None
    } else {
        Some(format!(
            "expected to accept {:?} but accepted {:?}: p={:?}, alpha={:?}, alt_hyp:{:?}",
            exp_accepted,
            res.accepted(),
            res.p(),
            res.alpha(),
            res.alt_hyp()
        ))
    }
}

#[inline(always)]
fn cmp_hyp(median_ratio: f64) -> Hyp {
    match median_ratio.partial_cmp(&1.0) {
        Some(Ordering::Less) => Some(AltHyp::Lt),
        Some(Ordering::Equal) => None,
        Some(Ordering::Greater) => Some(AltHyp::Gt),
        None => panic!("invalid spec_f1 or spec_f2"),
    }
}

#[derive(Debug)]
pub struct ClaimResult {
    spec_f1: FnSpec,
    spec_f2: FnSpec,
    claim_name: String,
    result: Option<String>,
}

impl ClaimResult {
    fn welch_ratio_test(spec_f1: FnSpec, spec_f2: FnSpec, out: &Comp, alpha: f64) -> ClaimResult {
        let claim_name = "welch_ratio_test";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio);
        let result = {
            let res = out.welch_ln_test(0., alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn student_diff_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_name = "student_diff_test";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio);
        let result = {
            let res = out.student_diff_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn student_ratio_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_name = "student_ratio_test";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio);
        let result = {
            let res = out.student_diff_ln_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn ratio_medians_f1_f2_near_target(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &Comp,
    ) -> ClaimResult {
        let claim_name = "ratio_medians_f1_f2_near_target";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let result = {
            let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();

            if ratio_medians_f1_f2.approx_eq(ratio, 0.005) {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={ratio_medians_f1_f2}, target={ratio}"
                ))
            }
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn ratio_medians_f1_f2_near_ratio_from_lns(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
    ) -> ClaimResult {
        let claim_name = "ratio_medians_f1_f2_near_ratio_from_lns";
        let result = {
            let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();
            let ratio_medians_f1_f2_from_lns = out.ratio_medians_f1_f2_from_lns();

            if ratio_medians_f1_f2.approx_eq(ratio_medians_f1_f2_from_lns, 0.005) {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={ratio_medians_f1_f2}, ratio_medians_f1_f2_from_lns={ratio_medians_f1_f2_from_lns}"
                ))
            }
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn target_ratio_medians_f1_f2_in_welch_ratio_ci(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &Comp,
        alpha: f64,
    ) -> ClaimResult {
        let claim_name = "target_ratio_medians_f1_f2_in_welch_ratio_ci";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let result = {
            let ci = out.welch_ratio_ci(alpha);

            if ci.position_of(ratio) == PositionWrtCi::In {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={ratio}, welch_ratio_ci={ci:?}"
                ))
            }
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn target_ratio_medians_f1_f2_in_student_ratio_ci(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_name = "target_ratio_medians_f1_f2_in_student_ratio_ci";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let result = {
            let ci = out.student_ratio_ci(alpha);

            if ci.position_of(ratio) == PositionWrtCi::In {
                None
            } else {
                Some(format!(
                    "ratio_medians_f1_f2={ratio}, student_ratio_ci={ci:?}"
                ))
            }
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn wilcoxon_rank_sum_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_name = "wilcoxon_rank_sum_test";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio);
        let result = {
            let res = out.wilcoxon_rank_sum_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }

    fn binomial_test(spec_f1: FnSpec, spec_f2: FnSpec, out: &DiffOut, alpha: f64) -> ClaimResult {
        let claim_name = "binomial_test";
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio);
        let result = {
            let res = out.exact_binomial_f1_gt_f2_eq_half_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_name: claim_name.into(),
            result,
        }
    }
}

pub struct ClaimResults {
    failures: Vec<ClaimResult>,
    summary: BTreeMap<((FnSpec, FnSpec), String), u32>,
}

impl ClaimResults {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            summary: BTreeMap::new(),
        }
    }

    fn push_claim_result(&mut self, claim_result: ClaimResult, verbose: bool) {
        let ClaimResult {
            spec_f1,
            spec_f2,
            claim_name,
            result,
        } = claim_result;

        let cond_name = if verbose {
            Some(claim_name.clone())
        } else {
            None
        };

        let value = self
            .summary
            .entry(((spec_f1, spec_f2), claim_name))
            .or_insert(0);

        if result.is_some() {
            *value += 1;
            if let Some(claim_name) = cond_name {
                self.failures.push(ClaimResult {
                    spec_f1,
                    spec_f2,
                    claim_name,
                    result,
                });
            }
        };
    }

    pub fn check_claims_comp(
        &mut self,
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        alpha: f64,
        out: &Comp,
        verbose: bool,
    ) {
        self.push_claim_result(
            ClaimResult::welch_ratio_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::ratio_medians_f1_f2_near_target(spec_f1, spec_f2, out),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::target_ratio_medians_f1_f2_in_welch_ratio_ci(spec_f1, spec_f2, out, alpha),
            verbose,
        );
    }

    pub fn check_claims_diff(
        &mut self,
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        alpha: f64,
        out: &DiffOut,
        verbose: bool,
    ) {
        let comp = out.comp();
        self.push_claim_result(
            ClaimResult::welch_ratio_test(spec_f1, spec_f2, &comp, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::student_diff_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::student_ratio_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::ratio_medians_f1_f2_near_target(spec_f1, spec_f2, &comp),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::ratio_medians_f1_f2_near_ratio_from_lns(spec_f1, spec_f2, out),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::target_ratio_medians_f1_f2_in_welch_ratio_ci(
                spec_f1, spec_f2, &comp, alpha,
            ),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::target_ratio_medians_f1_f2_in_student_ratio_ci(
                spec_f1, spec_f2, out, alpha,
            ),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::wilcoxon_rank_sum_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::binomial_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );
    }

    pub fn summary(&self) -> &BTreeMap<((FnSpec, FnSpec), String), u32> {
        &self.summary
    }

    pub fn failures(&self) -> &Vec<ClaimResult> {
        &self.failures
    }

    pub fn failure_summary(&self) -> BTreeMap<((FnSpec, FnSpec), String), u32> {
        self.summary
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    pub fn success_summary(&self) -> BTreeSet<((FnSpec, FnSpec), String)> {
        self.summary
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Counts of claims that exceed their Type I errors, with tolerance `τ`. The higher the value of `τ`,
    /// the more tolerant we are about the acceptable number of errors in `nrepeat` trials.
    ///
    /// Calculation for alpha when median(latency(f1)) == median(latency(f2)).
    /// - Hyp0: Prob(latency(f1) > latency(f2) == 0.5), for example. It could be any null hypothesis that should be accepted.
    /// - Type I error = Prob(Hyp0 rejected) should be <= α.
    /// - Thus, given the Type I error hypothesis above, let critical_value = binomial_inv_cdf(nrepeats, α, τ):
    ///   - Prob(number of Hyp0 rejections in nrepeat trials <= critical_value) >= τ.
    ///   - Equivalently, Prob(number of Hyp0 rejections in nrepeat trials > critical_value) < 1-τ.
    ///
    /// Returns a map from claim keys to the excessive number of errors associated with the key.
    pub fn excess_type_i_errors(
        &self,
        alpha: f64,
        claim_names: &[&'static str],
        nrepeats: usize,
        tau: f64,
    ) -> BTreeMap<((FnSpec, FnSpec), String), u32> {
        let max_alpha_count = binomial_inv_cdf(nrepeats as u64, alpha, tau).unwrap();

        let predicate =
            |spec_f1: &FnSpec, spec_f2: &FnSpec, claim_name: &str, count: u64| -> bool {
                let eq_base_median = spec_f1.base_median_factor == spec_f2.base_median_factor;

                if eq_base_median && claim_names.contains(&claim_name) && count > max_alpha_count {
                    true
                } else {
                    false
                }
            };

        self.summary
            .iter()
            .filter(|(((spec_f1, spec_f2), claim_name), count)| {
                predicate(spec_f1, spec_f2, claim_name, **count as u64)
            })
            .map(|(k, v)| (k.clone(), *v))
            .collect::<BTreeMap<_, _>>()
    }

    /// Counts of claims that exceed their Type I or Type II errors, with tolerance `τ`. The higher the value of `τ`,
    /// the more tolerant we are about the acceptable number of errors in `nrepeat` trials.
    ///
    /// Calculation for beta when median(latency(f1)) < median(latency(f2)).
    /// - Hyp0: Prob(latency(f1) > latency(f2) == 0.5), for example. It could be any null hypothesis that should be rejected.
    /// - Type II error = Prob(Hyp0 accepted) should be <= β.
    /// - Thus, given the Type II error hypothesis above, let critical_value = binomial_inv_cdf(nrepeats, α, τ):
    ///   - Prob(number of Hyp0 acceptances in nrepeat trials <= critical_value) >= τ.
    ///   - Equivalently, Prob(number of Hyp0 acceptances in nrepeat trials > critical_value) < 1-τ.
    ///
    /// Returns a map from claim keys to the excessive number of errors associated with the key.
    pub fn excess_type_ii_errors(
        &self,
        beta: f64,
        claim_names: &[&'static str],
        nrepeats: usize,
        tau: f64,
    ) -> BTreeMap<((FnSpec, FnSpec), String), u32> {
        let max_beta_count = binomial_inv_cdf(nrepeats as u64, beta, tau).unwrap();

        let predicate =
            |spec_f1: &FnSpec, spec_f2: &FnSpec, claim_name: &str, count: u64| -> bool {
                let eq_base_median = spec_f1.base_median_factor == spec_f2.base_median_factor;

                if !eq_base_median && claim_names.contains(&claim_name) && count > max_beta_count {
                    true
                } else {
                    false
                }
            };

        self.summary
            .iter()
            .filter(|(((spec_f1, spec_f2), claim_name), count)| {
                predicate(spec_f1, spec_f2, claim_name, **count as u64)
            })
            .map(|(k, v)| (k.clone(), *v))
            .collect::<BTreeMap<_, _>>()
    }

    pub const CRITICAL_CLAIM_NAMES: [&'static str; 4] = [
        "welch_ratio_test",
        // "student_diff_test",
        "student_ratio_test",
        // "ratio_medians_f1_f2_near_target",
        // "ratio_medians_f1_f2_near_ratio_from_lns",
        "target_ratio_medians_f1_f2_in_welch_ratio_ci",
        "target_ratio_medians_f1_f2_in_student_ratio_ci",
        // "wilcoxon_rank_sum_test",
        // "binomial_test",
    ];
}

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
    fmt::{Debug, Display},
    sync::LazyLock,
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
fn cmp_hyp(median_ratio: f64, ref_ratio: f64) -> Hyp {
    match median_ratio.partial_cmp(&ref_ratio) {
        Some(Ordering::Less) => Some(AltHyp::Lt),
        Some(Ordering::Equal) => None,
        Some(Ordering::Greater) => Some(AltHyp::Gt),
        None => panic!("invalid spec_f1 or spec_f2"),
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClaimId {
    claim_type: &'static str,
    qualifier: Option<String>,
}

impl ClaimId {
    pub fn new(claim_type: &'static str, ref_ratio: Option<f64>) -> Self {
        Self {
            claim_type,
            qualifier: ref_ratio.map(|v| v.to_string()),
        }
    }

    pub fn has_ref_ratio(&self, ref_ratio: f64) -> Option<bool> {
        if let Some(v) = &self.qualifier {
            Some(v == &ref_ratio.to_string())
        } else {
            None
        }
    }
}

impl Display for ClaimId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.claim_type.to_string();
        if let Some(q) = &self.qualifier {
            s += &format!("[{q}]");
        }
        f.write_str(&s)
    }
}

impl Debug for ClaimId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Debug)]
pub struct ClaimResult {
    spec_f1: FnSpec,
    spec_f2: FnSpec,
    claim_id: ClaimId,
    result: Option<String>,
}

impl ClaimResult {
    fn welch_ratio_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        ref_ratio: f64,
        out: &Comp,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("welch_ratio_test");
        let claim_id = ClaimId::new(claim_type, Some(ref_ratio));
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio, ref_ratio);
        let result = {
            let res = out.welch_ln_test(ref_ratio.ln(), alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    /// Not very useful; consider deleting. Can't fit it into the pattern of the other tests, with a `ref_ratio` argument.
    fn student_diff_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("student_diff_test");
        let claim_id = ClaimId::new(claim_type, None);
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio, 1.0);
        let result = {
            let res = out.student_diff_test(0., alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    fn student_ratio_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        ref_ratio: f64,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("student_ratio_test");
        let claim_id = ClaimId::new(claim_type, Some(ref_ratio));

        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio, ref_ratio);
        let result = {
            let res = out.student_diff_ln_test(ref_ratio.ln(), alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    fn ratio_medians_f1_f2_near_target(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &Comp,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("ratio_medians_f1_f2_near_target");
        let claim_id = ClaimId::new(claim_type, None);

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
            claim_id,
            result,
        }
    }

    fn ratio_medians_f1_f2_near_ratio_from_lns(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("ratio_medians_f1_f2_near_ratio_from_lns");
        let claim_id = ClaimId::new(claim_type, None);
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
            claim_id,
            result,
        }
    }

    fn target_ratio_medians_f1_f2_in_welch_ratio_ci(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &Comp,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type = Self::validated_claim_type("target_ratio_medians_f1_f2_in_welch_ratio_ci");
        let claim_id = ClaimId::new(claim_type, None);
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
            claim_id,
            result,
        }
    }

    fn target_ratio_medians_f1_f2_in_student_ratio_ci(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type =
            Self::validated_claim_type("target_ratio_medians_f1_f2_in_student_ratio_ci");
        let claim_id = ClaimId::new(claim_type, None);
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
            claim_id,
            result,
        }
    }

    fn reversed_ratio_medians(spec_f1: FnSpec, spec_f2: FnSpec, comp: &Comp) -> ClaimResult {
        let claim_type = Self::validated_claim_type("reversed_ratio_medians");
        let claim_id = ClaimId::new(claim_type, None);
        let target_ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let measured_ratio = comp.ratio_medians_f1_f2();
        let result = if target_ratio.ln() * measured_ratio.ln() >= 0. {
            None
        } else {
            Some(format!(
                "target_ratio={target_ratio}, measured_ratio={measured_ratio}"
            ))
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    fn wilcoxon_rank_sum_test(
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        out: &DiffOut,
        alpha: f64,
    ) -> ClaimResult {
        let claim_type = "wilcoxon_rank_sum_test";
        let claim_id = ClaimId::new(claim_type, None);
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio, 1.);
        let result = {
            let res = out.wilcoxon_rank_sum_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    fn binomial_test(spec_f1: FnSpec, spec_f2: FnSpec, out: &DiffOut, alpha: f64) -> ClaimResult {
        let claim_type = "binomial_test";
        let claim_id = ClaimId::new(claim_type, None);
        let ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
        let hyp = cmp_hyp(ratio, 1.);
        let result = {
            let res = out.exact_binomial_f1_gt_f2_eq_half_test(alt_hyp(hyp), alpha);
            check_hyp_test_result(res, hyp)
        };
        ClaimResult {
            spec_f1,
            spec_f2,
            claim_id,
            result,
        }
    }

    /// Claim types with indication of whether they are critical.
    const CLAIM_TYPES_WITH_CRITICALITY: [(&'static str, bool); 10] = [
        ("welch_ratio_test", true),
        ("student_diff_test", false),
        ("student_ratio_test", true),
        ("ratio_medians_f1_f2_near_target", false),
        ("ratio_medians_f1_f2_near_ratio_from_lns", false),
        ("target_ratio_medians_f1_f2_in_welch_ratio_ci", true),
        ("target_ratio_medians_f1_f2_in_student_ratio_ci", true),
        ("wilcoxon_rank_sum_test", false),
        ("binomial_test", false),
        ("reversed_ratio_medians", true),
    ];

    fn is_valid_claim_type(claim_type: &str) -> bool {
        static CLAIM_TYPES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
            ClaimResult::CLAIM_TYPES_WITH_CRITICALITY
                .iter()
                .map(|x| x.0)
                .collect::<Vec<_>>()
        });
        CLAIM_TYPES.contains(&claim_type)
    }

    fn validated_claim_type(claim_type: &'static str) -> &'static str {
        assert!(
            Self::is_valid_claim_type(claim_type),
            "invalid claim type {claim_type}"
        );
        claim_type
    }

    pub fn is_critical_claim_type(claim_type: &str) -> bool {
        assert!(
            Self::is_valid_claim_type(claim_type),
            "invalid claim type {claim_type}"
        );
        Self::CLAIM_TYPES_WITH_CRITICALITY
            .iter()
            .filter(|p| **p == (claim_type, true))
            .next()
            .is_some()
    }
}

pub struct ClaimResults {
    failures: Vec<ClaimResult>,
    summary: BTreeMap<((FnSpec, FnSpec), ClaimId), u32>,
}

impl ClaimResults {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            summary: BTreeMap::new(),
        }
    }

    fn push_claim_result(&mut self, cr: ClaimResult, verbose: bool) {
        let value = self
            .summary
            .entry(((cr.spec_f1, cr.spec_f2), cr.claim_id.clone()))
            .or_insert(0);

        if cr.result.is_some() {
            *value += 1;
            if verbose {
                self.failures.push(cr);
            }
        };
    }

    fn check_welch_test(
        &mut self,
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        alpha: f64,
        comp: &Comp,
        verbose: bool,
    ) {
        self.push_claim_result(
            ClaimResult::welch_ratio_test(spec_f1, spec_f2, 1., comp, alpha),
            verbose,
        );

        let full_ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;

        if full_ratio == 1. {
            return;
        }

        let mid_ratio = (full_ratio + 1.) / 2.;

        self.push_claim_result(
            ClaimResult::welch_ratio_test(spec_f1, spec_f2, mid_ratio, comp, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::welch_ratio_test(spec_f1, spec_f2, full_ratio, comp, alpha),
            verbose,
        );
    }

    fn check_student_test(
        &mut self,
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        alpha: f64,
        out: &DiffOut,
        verbose: bool,
    ) {
        self.push_claim_result(
            ClaimResult::student_ratio_test(spec_f1, spec_f2, 1., out, alpha),
            verbose,
        );

        let full_ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;

        if full_ratio == 1. {
            return;
        }

        let mid_ratio = (full_ratio + 1.) / 2.;

        self.push_claim_result(
            ClaimResult::student_ratio_test(spec_f1, spec_f2, mid_ratio, out, alpha),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::student_ratio_test(spec_f1, spec_f2, full_ratio, out, alpha),
            verbose,
        );
    }

    pub fn check_claims_comp(
        &mut self,
        spec_f1: FnSpec,
        spec_f2: FnSpec,
        alpha: f64,
        comp: &Comp,
        verbose: bool,
    ) {
        self.check_welch_test(spec_f1, spec_f2, alpha, comp, verbose);

        self.push_claim_result(
            ClaimResult::ratio_medians_f1_f2_near_target(spec_f1, spec_f2, comp),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::target_ratio_medians_f1_f2_in_welch_ratio_ci(
                spec_f1, spec_f2, comp, alpha,
            ),
            verbose,
        );
        self.push_claim_result(
            ClaimResult::reversed_ratio_medians(spec_f1, spec_f2, comp),
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

        self.check_claims_comp(spec_f1, spec_f2, alpha, &comp, verbose);

        self.push_claim_result(
            ClaimResult::student_diff_test(spec_f1, spec_f2, out, alpha),
            verbose,
        );

        self.check_student_test(spec_f1, spec_f2, alpha, out, verbose);

        self.push_claim_result(
            ClaimResult::ratio_medians_f1_f2_near_ratio_from_lns(spec_f1, spec_f2, out),
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

    pub fn summary(&self) -> &BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        &self.summary
    }

    pub fn failures(&self) -> &Vec<ClaimResult> {
        &self.failures
    }

    pub fn failure_summary(&self) -> BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        self.summary
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    pub fn success_summary(&self) -> BTreeSet<((FnSpec, FnSpec), ClaimId)> {
        self.summary
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k.clone())
            .collect()
    }

    fn filter(
        &self,
        pred: impl Fn(&FnSpec, &FnSpec, &ClaimId, u64) -> bool,
    ) -> BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        self.summary
            .iter()
            .filter(|(((spec_f1, spec_f2), claim_id), count)| {
                pred(spec_f1, spec_f2, claim_id, **count as u64)
            })
            .map(|(k, v)| (k.clone(), *v))
            .collect::<BTreeMap<_, _>>()
    }

    /// Counts of hypothesis test claims that exceed their Type I errors, with tolerance `τ`. The higher the value of `τ`,
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
        nrepeats: usize,
        tau: f64,
    ) -> BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        let max_alpha_count = binomial_inv_cdf(nrepeats as u64, alpha, tau).unwrap();

        let predicate =
            |spec_f1: &FnSpec, spec_f2: &FnSpec, claim_id: &ClaimId, count: u64| -> bool {
                let target_ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
                let claim_type = claim_id.claim_type;
                if ClaimResult::is_critical_claim_type(claim_type)
                    && claim_id.has_ref_ratio(target_ratio) == Some(true)
                    && count > max_alpha_count
                {
                    true
                } else {
                    false
                }
            };

        self.filter(predicate)
    }

    /// Counts of hypothesis test claims that exceed their Type II errors, with tolerance `τ`. The higher the value of `τ`,
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
        nrepeats: usize,
        tau: f64,
    ) -> BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        let max_beta_count = binomial_inv_cdf(nrepeats as u64, beta, tau).unwrap();

        let predicate =
            |spec_f1: &FnSpec, spec_f2: &FnSpec, claim_id: &ClaimId, count: u64| -> bool {
                let target_ratio = spec_f1.base_median_factor / spec_f2.base_median_factor;
                let claim_type = claim_id.claim_type;
                if ClaimResult::is_critical_claim_type(claim_type)
                    && claim_id.has_ref_ratio(target_ratio) == Some(false)
                    && count > max_beta_count
                {
                    true
                } else {
                    false
                }
            };

        self.filter(predicate)
    }

    /// Counts of number of reversals of the ratio of the medians.
    ///
    /// Returns a map from claim keys to the number of reversals associated with the key.
    pub fn reversed_ratio_medians(&self) -> BTreeMap<((FnSpec, FnSpec), ClaimId), u32> {
        let predicate =
            |_spec_f1: &FnSpec, _spec_f2: &FnSpec, claim_id: &ClaimId, count: u64| -> bool {
                let claim_type = claim_id.claim_type;
                if ClaimResult::is_critical_claim_type(claim_type)
                    && claim_type == "reversed_ratio_medians"
                    && count > 0
                {
                    true
                } else {
                    false
                }
            };

        self.filter(predicate)
    }
}

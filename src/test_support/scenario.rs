//! Definition of [`Scenario`] and [`Claim`] types to support
//! implementaton of main benchmarking logic to verify [`bench_diff`].

use super::{ALPHA, Claim, ClaimResults};
use crate::{
    DiffOut,
    basic_stats::core::{AltHyp, Hyp},
};
use std::sync::LazyLock;

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

    pub fn check_claims(&self, results: &mut ClaimResults, diff_out: &DiffOut, verbose: bool) {
        for claim in &self.claims {
            results.push_claim(self.name1, self.name2, claim, diff_out, verbose);
        }
    }
}

pub static SCENARIO_SPECS: LazyLock<[Scenario; 14]> = LazyLock::new(|| {
    [
        Scenario::new(
            "base_median_no_var",
            "base_median_no_var",
            Claim::claims(Hyp::Null, 1., ALPHA),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_1pct_median_no_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.01, ALPHA),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_10pct_median_no_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.1, ALPHA),
        ),
        Scenario::new(
            "base_median_no_var",
            "hi_25pct_median_no_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.25, ALPHA),
        ),
        Scenario::new(
            "hi_1pct_median_no_var",
            "base_median_no_var",
            Claim::claims(Hyp::Alt(AltHyp::Gt), 1.01, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "base_median_lo_var",
            Claim::claims(Hyp::Null, 1., ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "base_median_hi_var",
            Claim::claims(Hyp::Null, 1., ALPHA),
        ),
        Scenario::new(
            "base_median_hi_var",
            "base_median_lo_var",
            Claim::claims(Hyp::Null, 1., ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_1pct_median_lo_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.01, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_10pct_median_lo_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.1, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_25pct_median_lo_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.25, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_1pct_median_hi_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.01, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_10pct_median_hi_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.1, ALPHA),
        ),
        Scenario::new(
            "base_median_lo_var",
            "hi_25pct_median_hi_var",
            Claim::claims(Hyp::Alt(AltHyp::Lt), 1. / 1.25, ALPHA),
        ),
    ]
});

pub static FN_NAME_PAIRS: LazyLock<Vec<(&'static str, &'static str)>> = LazyLock::new(|| {
    SCENARIO_SPECS
        .iter()
        .map(|s| (s.name1, s.name2))
        .collect::<Vec<_>>()
});

pub fn get_scenario(name1: &str, name2: &str) -> &'static Scenario {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.name1 == name1 && spec.name2 == name2)
        .unwrap_or_else(|| {
            panic!(
                "invalid fn name pair: ({name1}, {name2}); valid name pairs are: {FN_NAME_PAIRS:?}"
            )
        })
}

use super::{AltHyp, Ci, HypTestResult};
use statrs::distribution::{ContinuousCDF, Normal, StudentsT};

/// Returns the p-value for a z-value from the standard normal.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn z_to_p(z: f64, alt_hyp: AltHyp) -> f64 {
    let normal = Normal::standard();

    match alt_hyp {
        AltHyp::Lt => normal.cdf(z),
        AltHyp::Gt => normal.cdf(-z),
        AltHyp::Ne => normal.cdf(-z.abs()) * 2.,
    }
}

/// Returns the p-value for a t-value from the Student distribution with location 0, scale 1, and `deg_freedom` degrees of freedom.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn t_to_p(t: f64, deg_freedom: f64, alt_hyp: AltHyp) -> f64 {
    let stud = StudentsT::new(0., 1., deg_freedom).expect("degrees of freedom must be > 0");

    match alt_hyp {
        AltHyp::Lt => stud.cdf(t),
        AltHyp::Gt => stud.cdf(-t),
        AltHyp::Ne => stud.cdf(-t.abs()) * 2.,
    }
}

/// Returns the probability that the standard normal distribution produces a value greater than `alpha`.
pub fn z_alpha(alpha: f64) -> f64 {
    let normal = Normal::standard();
    normal.cdf(-alpha)
}

/// Returns the probability that the Student distribution with location 0, scale 1, and `deg_freedom` degrees of freedom
/// produces a value greater than `alpha`.
pub fn t_alpha(deg_freedom: f64, alpha: f64) -> f64 {
    let stud = StudentsT::new(0., 1., deg_freedom).expect("degrees of freedom must be > 0");
    stud.cdf(-alpha)
}

#[inline(always)]
pub fn sample_mean(n: f64, sum: f64) -> f64 {
    sum / n
}

#[inline(always)]
pub fn sample_sum2_deviations(n: f64, sum: f64, sum2: f64) -> f64 {
    sum2 - sum.powi(2) / n
}

#[inline(always)]
pub fn sample_var(n: f64, sum: f64, sum2: f64) -> f64 {
    sample_sum2_deviations(n, sum, sum2) / (n - 1.)
}

#[inline(always)]
pub fn sample_stdev(n: f64, sum: f64, sum2: f64) -> f64 {
    sample_var(n, sum, sum2).sqrt()
}

#[cfg(feature = "dev_support")]
/// Estimator of mean of Bernoulli distribution.
///
/// Arguments:
/// - `n`: number of trials.
/// - `successes`: number of successes (`1`s) observed.
pub fn bernoulli_p_hat(n: f64, successes: f64) -> f64 {
    successes / n
}

#[cfg(feature = "dev_support")]
/// Confidence interval for the probability of success of a Bernoulli distribution computed from a
/// sample of trials (Wilson score interval).
///
/// Arguments:
/// - `n`: number of trials.
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `alpha`: confidence level.
pub fn bernoulli_psucc_ci(n: f64, p_hat: f64, alpha: f64) -> Ci {
    let p = p_hat;
    let z_alpha_2 = z_alpha(alpha / 2.);
    let mid = p + z_alpha_2.powi(2) / (2. * n);
    let delta = z_alpha_2 * (p * (1. - p) / n + z_alpha_2.powi(2) / (4. * n.powi(2))).sqrt();
    let denom = 1. + z_alpha_2.powi(2) / n;
    Ci((mid - delta) / denom, (mid + delta) / denom)
}

#[cfg(feature = "dev_support")]
/// Normal approximation z-value for standardized sample mean of Bernoulli distribution under the hypothesis that
/// the probability of success is `p0`.
///
/// Arguments:
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `p0`: probability of success under null hypothesis.
pub fn bernoulli_normal_approx_z(n: f64, p_hat: f64, p0: f64) -> f64 {
    (p_hat - p0) / (p0 * (1. - p0) / n).sqrt()
}

#[cfg(feature = "dev_support")]
/// Normal approximation p-value for standardized sample mean of Bernoulli distribution under the hypothesis that
/// the probability of success is `p0`.
///
/// Arguments:
/// - `n`: number of trials.
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `p0`: probability of success under null hypothesis.
/// - `alt_hyp`: alternative hypothesis.
pub fn bernoulli_normal_approx_p(n: f64, p_hat: f64, p0: f64, alt_hyp: AltHyp) -> f64 {
    let z = bernoulli_normal_approx_z(n, p_hat, p0);
    z_to_p(z, alt_hyp)
}

#[cfg(feature = "dev_support")]
pub fn bernoulli_test(n: f64, p_hat: f64, p0: f64, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
    let p = bernoulli_normal_approx_p(n, p_hat, p0, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

pub struct SampleMoments {
    count: u64,
    sum: f64,
    sum2: f64,
    min: f64,
    max: f64,
}

impl SampleMoments {
    pub fn new(count: u64, sum: f64, sum2: f64) -> Self {
        Self {
            count,
            sum,
            sum2,
            min: f64::NAN,
            max: f64::NAN,
        }
    }

    pub fn new_with_min_max(count: u64, sum: f64, sum2: f64, min: f64, max: f64) -> Self {
        Self {
            count,
            sum,
            sum2,
            min,
            max,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0., 0.)
    }

    pub fn collect_value(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.sum2 += value * value;
        self.min = value.min(self.min);
        self.max = value.max(self.max);
    }

    pub fn from_slice(dataset: &[f64]) -> Self {
        let mut moments = SampleMoments::empty();
        for value in dataset {
            moments.collect_value(*value);
        }
        moments
    }

    pub fn n(&self) -> f64 {
        self.count as f64
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn mean(&self) -> f64 {
        self.sum / self.n()
    }

    pub fn sum2(&self) -> f64 {
        self.sum2
    }

    pub fn sum2_deviations(&self) -> f64 {
        sample_sum2_deviations(self.n(), self.sum, self.sum2)
    }

    /// Returns `Nan` if  `self.n()` == 1
    pub fn var(&self) -> f64 {
        sample_var(self.n(), self.sum, self.sum2)
    }

    /// Returns `Nan` if  `self.n()` == 1
    pub fn stdev(&self) -> f64 {
        sample_stdev(self.n(), self.sum, self.sum2)
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Default for SampleMoments {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.,
            sum2: 0.,
            min: f64::NAN,
            max: f64::NAN,
        }
    }
}

pub fn welch_t(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let s_dx = (s2_mean_a + s2_mean_b).sqrt();
    dx / s_dx
}

pub fn welch_deg_freedom(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let numerator = (s2_mean_a + s2_mean_b).powi(2);
    let denominator = s2_mean_a.powi(2) / (n_a - 1.) + s2_mean_b.powi(2) / (n_b - 1.);
    numerator / denominator
}

pub fn welch_p(moments_a: &SampleMoments, moments_b: &SampleMoments, alt_hyp: AltHyp) -> f64 {
    let t = welch_t(moments_a, moments_b);
    let deg_freedom = welch_deg_freedom(moments_a, moments_b);
    t_to_p(t, deg_freedom, alt_hyp)
}

pub fn welch_alt_hyp_ci(
    moments_a: &SampleMoments,
    moments_b: &SampleMoments,
    alt_hyp: AltHyp,
    alpha: f64,
) -> Ci {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let df = welch_deg_freedom(moments_a, moments_b);

    let stud = StudentsT::new(0., 1., df).expect("Welch degrees of freedom must be > 0");
    let t0 = match alt_hyp {
        AltHyp::Ne => -stud.inverse_cdf(alpha / 2.),
        _ => -stud.inverse_cdf(alpha),
    };

    let mid = dx;
    let delta = (s2_mean_a + s2_mean_b).sqrt() * t0;

    match alt_hyp {
        AltHyp::Lt => Ci(-f64::INFINITY, mid + delta),
        AltHyp::Ne => Ci(mid - delta, mid + delta),
        AltHyp::Gt => Ci(mid - delta, f64::INFINITY),
    }
}

pub fn welch_ci(moments_a: &SampleMoments, moments_b: &SampleMoments, alpha: f64) -> Ci {
    welch_alt_hyp_ci(moments_a, moments_b, AltHyp::Ne, alpha)
}

pub fn welch_test(
    moments_a: &SampleMoments,
    moments_b: &SampleMoments,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = welch_p(&moments_a, &moments_b, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

pub fn student_one_sample_t(moments: &SampleMoments, mu0: f64) -> f64 {
    let n = moments.n();
    let mean = moments.mean();
    let s = moments.stdev();
    (mean - mu0) / s * n.sqrt()
}

pub fn student_one_sample_deg_freedom(moments: &SampleMoments) -> f64 {
    moments.n() - 1.
}

pub fn student_one_sample_p(moments: &SampleMoments, mu0: f64, alt_hyp: AltHyp) -> f64 {
    let t = student_one_sample_t(moments, mu0);
    let deg_freedom = student_one_sample_deg_freedom(moments);
    t_to_p(t, deg_freedom, alt_hyp)
}

pub fn student_one_sample_alt_hyp_ci(moments: &SampleMoments, alt_hyp: AltHyp, alpha: f64) -> Ci {
    let df = student_one_sample_deg_freedom(moments);

    let stud = StudentsT::new(0., 1., df)
        .expect("can't happen: degrees of freedom is always >= 3 by construction");
    let t0 = match alt_hyp {
        AltHyp::Ne => -stud.inverse_cdf(alpha / 2.),
        _ => -stud.inverse_cdf(alpha),
    };

    let mid = moments.mean();
    let delta = (moments.stdev() / moments.n().sqrt()) * t0;

    match alt_hyp {
        AltHyp::Lt => Ci(-f64::INFINITY, mid + delta),
        AltHyp::Ne => Ci(mid - delta, mid + delta),
        AltHyp::Gt => Ci(mid - delta, f64::INFINITY),
    }
}

pub fn student_one_sample_ci(moments: &SampleMoments, alpha: f64) -> Ci {
    student_one_sample_alt_hyp_ci(moments, AltHyp::Ne, alpha)
}

pub fn student_one_sample_test(
    moments: &SampleMoments,
    mu0: f64,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = student_one_sample_p(&moments, mu0, alt_hyp);
    HypTestResult::new(p, alpha, alt_hyp)
}

#[cfg(test)]
mod test_welch {
    use super::*;
    use crate::{dev_utils::ApproxEq, statistics::AltHyp};

    const ALPHA: f64 = 0.05;

    fn compare_welch(
        dataset_a: &[f64],
        dataset_b: &[f64],
        alt_hyp: AltHyp,
        exp_t: f64,
        exp_df: f64,
        exp_p: f64,
        exp_ci: Ci,
    ) {
        println!("*** alternative hypothesis: {alt_hyp:?}");

        let moments_a = SampleMoments::from_slice(dataset_a);
        let moments_b = SampleMoments::from_slice(dataset_b);

        let t = welch_t(&moments_a, &moments_b);
        let df = welch_deg_freedom(&moments_a, &moments_b);
        let p = t_to_p(t, df, alt_hyp);
        let ci = welch_alt_hyp_ci(&moments_a, &moments_b, alt_hyp, ALPHA);

        let epsilon = 0.001;
        assert!(exp_t.approx_eq(t, epsilon), "exp_t={exp_t}, t={t}");
        assert!(exp_df.approx_eq(df, epsilon), "exp_df={exp_df}, df={df}");
        assert!(exp_p.approx_eq(p, epsilon), "exp_p={exp_p}, p={p}");
        assert!(
            exp_ci.0.approx_eq(ci.0, epsilon) || exp_ci.0.is_infinite() && ci.0.is_infinite(),
            "exp_ci.0={}, ci.0={}",
            exp_ci.0,
            ci.0
        );
        assert!(
            exp_ci.1.approx_eq(ci.1, epsilon) || exp_ci.1.is_infinite() && ci.1.is_infinite(),
            "exp_ci.1={}, ci.1={}",
            exp_ci.1,
            ci.1
        );
    }

    #[test]
    fn test_welch_eq() {
        let a = [14., 15., 15., 15., 16., 18., 22., 23., 24., 25., 25.];
        let b = [
            10., 12., 14., 15., 18., 22., 24., 27., 31., 33., 34., 34., 34.,
        ];

        let exp_t = -1.5379;
        let exp_df = 18.137;

        {
            let alt_hyp = AltHyp::Ne;
            let exp_p = 0.1413;
            let exp_ci = Ci(-10.453875, 1.614714);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_p = 0.9293;
            let exp_ci = Ci(-9.40084, f64::INFINITY);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }

        {
            let alt_hyp = AltHyp::Lt;
            let exp_p = 0.07067;
            let exp_ci = Ci(-f64::INFINITY, 0.5616789);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }
    }

    #[test]
    fn test_welch_gt() {
        let a = [24., 28., 32., 29., 35., 36., 30., 32., 25., 31.];
        let b = [5., 10., 25., 15., 16., 20.];

        let exp_t = 4.7857;
        let exp_df = 6.8409;

        {
            let alt_hyp = AltHyp::Ne;
            let exp_p = 0.00213;
            let exp_ci = Ci(7.57018, 22.49649);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }

        {
            let alt_hyp = AltHyp::Gt;
            let exp_p = 0.001065;
            let exp_ci = Ci(9.061005, f64::INFINITY);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }

        {
            let alt_hyp = AltHyp::Lt;
            let exp_p = 0.9989;
            let exp_ci = Ci(-f64::INFINITY, 21.00566);
            compare_welch(&a, &b, alt_hyp, exp_t, exp_df, exp_p, exp_ci);
        }
    }
}

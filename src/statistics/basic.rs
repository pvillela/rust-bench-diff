use core::f64;

use statrs::distribution::{ContinuousCDF, Normal, StudentsT};

/// Altnernative statistical hypothesis to the null hypothesis that there is no difference between the two distributions.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AltHyp {
    Lt,
    Gt,
    Ne,
}

/// Statistical hypothesis test result
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HypTestResult {
    /// Accept the null hypothesis
    Accept { p: f64, alpha: f64 },
    /// Reject the null hypothesis
    Reject { p: f64, alpha: f64 },
}

impl HypTestResult {
    pub fn from_p_and_alpha(p: f64, alpha: f64) -> HypTestResult {
        if p < alpha {
            HypTestResult::Reject { p, alpha }
        } else {
            HypTestResult::Accept { p, alpha }
        }
    }

    pub fn p(&self) -> f64 {
        match self {
            Self::Accept { p, .. } => *p,
            Self::Reject { p, .. } => *p,
        }
    }

    pub fn alpha(&self) -> f64 {
        match self {
            Self::Accept { alpha, .. } => *alpha,
            Self::Reject { alpha, .. } => *alpha,
        }
    }

    pub fn is_accept(&self) -> bool {
        match self {
            Self::Accept { .. } => true,
            _ => false,
        }
    }

    pub fn is_reject(&self) -> bool {
        !self.is_accept()
    }
}

/// Returns the p-value for a z-value from the standard normal.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn z_to_p(z: f64, alt_hyp: AltHyp) -> f64 {
    let normal = Normal::standard();

    match alt_hyp {
        AltHyp::Lt => normal.cdf(-z),
        AltHyp::Gt => normal.cdf(z),
        AltHyp::Ne => normal.cdf(-z.abs()) * 2.0,
    }
}

/// Returns the p-value for a t-value from the Student distribution with location 0, scale 1, and `deg_freedom` degrees of freedom.
/// `alt_hyp` is the alternative hypothesis. The null hypothesis is that the sample distribution's mean is 0.
pub fn t_to_p(t: f64, deg_freedom: f64, alt_hyp: AltHyp) -> f64 {
    let stud = StudentsT::new(0.0, 1.0, deg_freedom).expect("degrees of freedom must be > 0");

    match alt_hyp {
        AltHyp::Lt => stud.cdf(-t),
        AltHyp::Gt => stud.cdf(t),
        AltHyp::Ne => stud.cdf(-t.abs()) * 2.0,
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
    let stud = StudentsT::new(0.0, 1.0, deg_freedom).expect("degrees of freedom must be > 0");
    stud.cdf(-alpha)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionWrtCi {
    Below,
    In,
    Above,
}

impl PositionWrtCi {
    pub fn position_of_value(value: f64, low: f64, high: f64) -> Self {
        match value {
            _ if value <= low => PositionWrtCi::Below,
            _ if low < value && value < high => PositionWrtCi::In,
            _ => PositionWrtCi::Above,
        }
    }
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
    sample_sum2_deviations(n, sum, sum2) / (n - 1.0)
}

#[inline(always)]
pub fn sample_stdev(n: f64, sum: f64, sum2: f64) -> f64 {
    sample_var(n, sum, sum2).sqrt()
}

/// Estimator of mean of Bernoulli distribution.
///
/// Arguments:
/// - `n`: number of trials.
/// - `successes`: number of successes (`1`s) observed.
pub fn bernoulli_p_hat(n: f64, successes: f64) -> f64 {
    successes / n
}

/// Confidence interval for the probability of success of a Bernoulli distribution computed from a
/// sample of trials (Wilson score interval).
///
/// Arguments:
/// - `n`: number of trials.
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `alpha`: confidence level.
pub fn bernoulli_psucc_ci(n: f64, p_hat: f64, alpha: f64) -> (f64, f64) {
    let p = p_hat;
    let z_alpha_2 = z_alpha(alpha / 2.0);
    let mid = p + z_alpha_2.powi(2) / (2.0 * n);
    let delta = z_alpha_2 * (p * (1.0 - p) / n + z_alpha_2.powi(2) / (4.0 * n.powi(2))).sqrt();
    let denom = 1.0 + z_alpha_2.powi(2) / n;
    ((mid - delta) / denom, (mid + delta) / denom)
}

/// Normal approximation z-value for standardized sample mean of Bernoulli distribution under the hypothesis that
/// the probability of success is `p0`.
///
/// Arguments:
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `p0`: probability of success under null hypothesis.
pub fn bernoulli_normal_approx_z(p_hat: f64, p0: f64) -> f64 {
    (p_hat - p0) / (p0 * (1.0 - p0)).sqrt()
}

/// Normal approximation p-value for standardized sample mean of Bernoulli distribution under the hypothesis that
/// the probability of success is `p0`.
///
/// Arguments:
/// - `n`: number of trials.
/// - `p_hat`: estimate of mean of the Bernoulli distribution. See [`bernoulli_p_hat`].
/// - `p0`: probability of success under null hypothesis.
/// - `alt_hyp`: alternative hypothesis.
pub fn bernoulli_normal_approx_p(p_hat: f64, p0: f64, alt_hyp: AltHyp) -> f64 {
    let z = bernoulli_normal_approx_z(p_hat, p0);
    z_to_p(z, alt_hyp)
}

pub fn bernoulli_test(p_hat: f64, p0: f64, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
    let p = bernoulli_normal_approx_p(p_hat, p0, alt_hyp);
    HypTestResult::from_p_and_alpha(p, alpha)
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
            sum: 0.0,
            sum2: 0.0,
            min: f64::NAN,
            max: f64::NAN,
        }
    }
}

pub fn collect_moments(moments: &mut SampleMoments, value: f64) {
    moments.count += 1;
    moments.sum += value;
    moments.sum2 += value * value;
    moments.min = value.min(moments.min);
    moments.max = value.max(moments.max);
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
    let denominator = s2_mean_a.powi(2) / (n_a - 1.0) + s2_mean_b.powi(2) / (n_b - 1.0);
    numerator / denominator
}

pub fn welch_p(moments_a: &SampleMoments, moments_b: &SampleMoments, alt_hyp: AltHyp) -> f64 {
    let t = welch_t(moments_a, moments_b);
    let deg_freedom = welch_deg_freedom(moments_a, moments_b);
    t_to_p(t, deg_freedom, alt_hyp)
}

pub fn welch_ci(moments_a: &SampleMoments, moments_b: &SampleMoments, alpha: f64) -> (f64, f64) {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let nu = welch_deg_freedom(moments_a, moments_b);

    let stud = StudentsT::new(0.0, 1.0, nu).expect("Welch degrees of freedom must be > 0");
    let t = -stud.inverse_cdf(alpha / 2.0);

    let mid = dx;
    let radius = (s2_mean_a + s2_mean_b).sqrt() * t;

    (mid - radius, mid + radius)
}

pub fn welch_test(
    moments_a: &SampleMoments,
    moments_b: &SampleMoments,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = welch_p(&moments_a, &moments_b, alt_hyp);
    HypTestResult::from_p_and_alpha(p, alpha)
}

pub fn student_one_sample_t(moments: &SampleMoments, mu0: f64) -> f64 {
    let n = moments.n();
    let mean = moments.mean();
    let s = moments.stdev();
    (mean - mu0) / s * n.sqrt()
}

pub fn student_one_sample_deg_freedom(moments: &SampleMoments) -> f64 {
    moments.n() - 1.0
}

pub fn student_one_sample_p(moments: &SampleMoments, mu0: f64, alt_hyp: AltHyp) -> f64 {
    let t = student_one_sample_t(moments, mu0);
    let deg_freedom = student_one_sample_deg_freedom(moments);
    t_to_p(t, deg_freedom, alt_hyp)
}

pub fn student_one_sample_ci(moments: &SampleMoments, alpha: f64) -> (f64, f64) {
    let nu = student_one_sample_deg_freedom(moments);
    let stud = StudentsT::new(0.0, 1.0, nu)
        .expect("can't happen: degrees of freedom is always >= 3 by construction");
    let t = -stud.inverse_cdf(alpha / 2.0);

    let mid = moments.mean();
    let radius = (moments.stdev() / moments.n().sqrt()) * t;

    (mid - radius, mid + radius)
}

pub fn student_one_sample_test(
    moments: &SampleMoments,
    mu0: f64,
    alt_hyp: AltHyp,
    alpha: f64,
) -> HypTestResult {
    let p = student_one_sample_p(&moments, mu0, alt_hyp);
    HypTestResult::from_p_and_alpha(p, alpha)
}

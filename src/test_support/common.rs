use statrs::distribution::{Binomial, ContinuousCDF, DiscreteCDF, Normal};

pub const ALPHA: f64 = 0.05;
pub const BETA: f64 = 0.05;
pub const BETA_01: f64 = 0.01;

pub const HI_1PCT_FACTOR: f64 = 1.01;
pub const HI_10PCT_FACTOR: f64 = 1.1;
pub const HI_25PCT_FACTOR: f64 = 1.25;

pub fn default_lo_stdev_ln() -> f64 {
    1.2_f64.ln() / 2.
}

pub fn default_hi_stdev_ln() -> f64 {
    2.4_f64.ln() / 2.
}

/// Returns the highest value `n_c` for which `Prob(Binomial(n, p0) <= n_c) <= theta`.
///
/// This is the exact inverse CDF of the binomial distribution.
pub fn binomial_exact_gt_critical_value(n: u64, p0: f64, theta: f64) -> u64 {
    let binomial = Binomial::new(p0, n).expect("invalid arguments to binomial distribution");
    binomial.inverse_cdf(theta)
}

/// Returns an approximation of the highest value `n_c` for which `Prob(Binomial(n, p0) <= n_c) <= theta`
/// using the normal approximation.
///
/// The normal approximations of the binomial distribution is acceptable for `p0 * nrepeats > 5`.
pub fn binomial_nappr_gt_critical_value(n: u64, p0: f64, theta: f64) -> u64 {
    let mean = n as f64 * p0;
    let stdev: f64 = (n as f64 * p0 * (1. - p0)).sqrt();
    let normal = Normal::new(mean, stdev).expect("invalid arguments to normal distribution");
    normal.inverse_cdf(theta).ceil() as u64
}

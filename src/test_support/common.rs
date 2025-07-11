use statrs::distribution::{Binomial, DiscreteCDF};

pub const ALPHA: f64 = 0.05;
pub const BETA: f64 = 0.05;
#[allow(unused)]
pub const BETA_01: f64 = 0.01;

/// Returns the highest value `n_s` for which `Prob(Binomial(n, p0) <= n_s) <= tau`.
///
/// This is the exact inverse CDF of the binomial distribution.
///
/// # Panics
/// - When `n` or `tau` are sufficently small (e.g., `n <= 7 && tau <= 0.67`), due to implementation in [`statrs`] crate.
pub fn binomial_inv_cdf(n: u64, p0: f64, tau: f64) -> u64 {
    let binomial = Binomial::new(p0, n).expect("invalid arguments to binomial distribution");
    binomial.inverse_cdf(tau)
}

#[allow(unused)]
/// Returns the value that is `nsigmas` standard deviations higher than the median of `Binomial(n, p0)`.
pub fn binomial_nsigmas_gt_critical_value(n: u64, p0: f64, nsigmas: f64) -> u64 {
    let mean = n as f64 * p0;
    let stdev: f64 = (n as f64 * p0 * (1. - p0)).sqrt();
    (mean + nsigmas * stdev).ceil() as u64
}

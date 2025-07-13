use basic_stats::core::StatsError;
use statrs::distribution::{Binomial, DiscreteCDF};
use std::panic::catch_unwind;

pub const ALPHA: f64 = 0.05;
pub const BETA: f64 = 0.05;
#[allow(unused)]
pub const BETA_01: f64 = 0.01;

/// Returns the highest value `n_s` for which `Prob(Binomial(n, p0) <= n_s) <= tau`.
///
/// This is the exact inverse CDF of the binomial distribution.
///
/// # Errors
///
/// Returns an error in any of these conditions:
/// - `n == 0`
/// - `p0` is not in the open interval (0, 1).
/// - The combination of `n`, `p0`, and `tau` is too small (e.g., `n <= 7 && p0 == 0.05 && tau <= 0.67`),
///   due to implementation in [`statrs`] crate.
pub fn binomial_inv_cdf(n: u64, p0: f64, tau: f64) -> Result<u64, StatsError> {
    let binomial = Binomial::new(p0, n)
        .map_err(|_| StatsError::new("`n == 0` or `p0` is not in the open interval (0, 1)"))?;

    catch_unwind(|| binomial.inverse_cdf(tau)).or_else(|_| {
        eprintln!("caught and handled dependency panic");
        Err(StatsError::new(format!(
            "binomial_inv_cdf: combination of n={n}, p0={p0}, and tau={tau} is too small"
        )))
    })
}

#[allow(unused)]
/// Returns the value that is `nsigmas` standard deviations higher than the median of `Binomial(n, p0)`.
pub fn binomial_nsigmas_gt_critical_value(n: u64, p0: f64, nsigmas: f64) -> u64 {
    let mean = n as f64 * p0;
    let stdev: f64 = (n as f64 * p0 * (1. - p0)).sqrt();
    (mean + nsigmas * stdev).ceil() as u64
}

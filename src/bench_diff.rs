//! Module to compare the difference in latency between two closures.

use crate::{
    SummaryStats, Timing, new_timing,
    statistics::{
        AltHyp, Ci, HypTestResult, PositionWrtCi, SampleMoments, sample_mean, sample_stdev,
        student_one_sample_ci, student_one_sample_t, student_one_sample_test, welch_ci,
        welch_deg_freedom, welch_t, welch_test,
    },
    summary_stats,
};
use core::f64;
use hdrhistogram::Histogram;
use std::{
    error::Error,
    io::{Write, stderr, stdout},
    time::{Duration, Instant},
};

#[cfg(feature = "dev_utils")]
use crate::statistics::{self, bernoulli_psucc_ci, bernoulli_test};

const WARMUP_MILLIS: u64 = 3_000;
const WARMUP_INCREMENT_COUNT: usize = 20;

#[derive(Clone, Copy, Debug)]
pub enum LatencyUnit {
    Milli,
    Micro,
    Nano,
}

impl LatencyUnit {
    #[inline(always)]
    pub fn latency_as_u64(&self, latency: Duration) -> u64 {
        match self {
            Self::Nano => latency.as_nanos() as u64,
            Self::Micro => latency.as_micros() as u64,
            Self::Milli => latency.as_millis() as u64,
        }
    }

    #[inline(always)]
    pub fn latency_from_u64(&self, elapsed: u64) -> Duration {
        match self {
            Self::Nano => Duration::from_nanos(elapsed),
            Self::Micro => Duration::from_micros(elapsed),
            Self::Milli => Duration::from_millis(elapsed),
        }
    }

    #[inline(always)]
    pub fn latency_as_f64(&self, latency: Duration) -> f64 {
        self.latency_as_u64(latency) as f64
    }

    #[inline(always)]
    pub fn latency_from_f64(&self, elapsed: f64) -> Duration {
        self.latency_from_u64(elapsed as u64)
    }
}

#[inline(always)]
pub fn latency(mut f: impl FnMut()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

#[inline(always)]
fn duo_exec(mut f1: impl FnMut(), mut f2: impl FnMut()) -> [(Duration, Duration); 2] {
    let l01 = latency(&mut f1);
    let l02 = latency(&mut f2);

    let l11 = latency(&mut f2);
    let l12 = latency(&mut f1);

    [(l01, l02), (l11, l12)]
}

pub struct DiffOut {
    hist_f1: Timing,
    hist_f2: Timing,
    hist_f1_lt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    count_f1_eq_f2: u64,
    hist_f1_gt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    sum_ln_f1: f64,
    sum2_ln_f1: f64,
    sum_ln_f2: f64,
    sum2_ln_f2: f64,
    sum_diff_f1_f2: f64,
    sum2_diff_f1_f2: f64,
    sum_diff_ln_f1_f2: f64,
    sum2_diff_ln_f1_f2: f64,
}

impl DiffOut {
    pub(crate) fn new() -> Self {
        let hist_f1 = new_timing(20 * 1000 * 1000, 5);
        let hist_f2 = Histogram::<u64>::new_from(&hist_f1);
        let hist_f1_lt_f2 = Histogram::<u64>::new_from(&hist_f1);
        let count_f1_eq_f2 = 0;
        let hist_f1_gt_f2 = Histogram::<u64>::new_from(&hist_f1);
        let sum_ln_f1 = 0.0_f64;
        let sum2_ln_f1 = 0.0_f64;
        let sum_ln_f2 = 0.0_f64;
        let sum2_ln_f2 = 0.0_f64;
        let sum_diff_f1_f2 = 0.0_f64;
        let sum2_diff_f1_f2 = 0.0_f64;
        let sum_diff_ln_f1_f2 = 0.0_f64;
        let sum2_diff_ln_f1_f2 = 0.0_f64;

        Self {
            hist_f1,
            hist_f2,
            hist_f1_lt_f2,
            count_f1_eq_f2,
            hist_f1_gt_f2,
            sum_ln_f1,
            sum2_ln_f1,
            sum_ln_f2,
            sum2_ln_f2,
            sum_diff_f1_f2,
            sum2_diff_f1_f2,
            sum_diff_ln_f1_f2,
            sum2_diff_ln_f1_f2,
        }
    }

    #[inline(always)]
    pub fn n(&self) -> f64 {
        self.hist_f1.len() as f64
    }

    pub fn summary_f1(&self) -> SummaryStats {
        summary_stats(&self.hist_f1)
    }

    pub fn summary_f2(&self) -> SummaryStats {
        summary_stats(&self.hist_f2)
    }

    pub fn count_f1_lt_f2(&self) -> u64 {
        self.hist_f1_lt_f2.len()
    }

    pub fn count_f1_eq_f2(&self) -> u64 {
        self.count_f1_eq_f2
    }

    pub fn count_f1_gt_f2(&self) -> u64 {
        self.hist_f1_gt_f2.len()
    }

    pub fn mean_ln_f1(&self) -> f64 {
        sample_mean(self.n(), self.sum_ln_f1)
    }

    pub fn stdev_ln_f1(&self) -> f64 {
        sample_stdev(self.n(), self.sum_ln_f1, self.sum2_ln_f1)
    }

    pub fn mean_ln_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_ln_f2)
    }

    pub fn stdev_ln_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_ln_f2, self.sum2_ln_f2)
    }

    pub fn mean_diff_f1_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_diff_f1_f2)
    }

    pub fn stdev_diff_f1_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_diff_f1_f2, self.sum2_diff_f1_f2)
    }

    pub fn mean_diff_ln_f1_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_diff_ln_f1_f2)
    }

    pub fn stdev_diff_ln_f1_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_diff_ln_f1_f2, self.sum2_diff_ln_f1_f2)
    }

    pub fn mean_f1(&self) -> f64 {
        self.summary_f1().mean
    }

    pub fn mean_f2(&self) -> f64 {
        self.summary_f2().mean
    }

    pub fn median_f1(&self) -> f64 {
        self.summary_f1().median as f64
    }

    pub fn median_f2(&self) -> f64 {
        self.summary_f2().median as f64
    }

    pub fn diff_medians_f1_f2(&self) -> f64 {
        self.median_f1() - self.median_f2()
    }

    // Ratio of medians computed from histograms.
    pub fn ratio_medians_f1_f2(&self) -> f64 {
        self.median_f1() / self.median_f2()
    }

    // Alternative ratio of medians computed from the `exp` of the mean difference of `ln`s of latencies.
    pub fn ratio_medians_f1_f2_from_lns(&self) -> f64 {
        self.mean_diff_ln_f1_f2().exp()
    }

    #[cfg(feature = "dev_utils")]
    /// Estimator of mean of Bernoulli distribution.
    pub fn bernoulli_prob_f1_gt_f2(&self) -> f64 {
        (self.count_f1_gt_f2() as f64 + self.count_f1_eq_f2 as f64 / 2.0)
            / (self.count_f1_lt_f2() + self.count_f1_eq_f2 + self.count_f1_gt_f2()) as f64
    }

    #[cfg(feature = "dev_utils")]
    /// Confidence interval for Bernoulli distribution (Wilson score interval).
    pub fn bernoulli_ci(&self, alpha: f64) -> Ci {
        let p_hat = self.bernoulli_prob_f1_gt_f2();
        let n = self.n();
        bernoulli_psucc_ci(n, p_hat, alpha)
    }

    #[cfg(feature = "dev_utils")]
    pub fn bernoulli_value_position_wrt_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.bernoulli_ci(alpha);
        ci.position_of(value)
    }

    #[cfg(feature = "dev_utils")]
    pub fn bernoulli_eq_half_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let p_hat = self.bernoulli_prob_f1_gt_f2();
        bernoulli_test(self.n(), p_hat, 1.0 / 2.0, alt_hyp, alpha)
    }

    /// Welch's t statistic for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    ///
    /// See [Welch's t-test](https://en.wikipedia.org/wiki/Welch%27s_t-test)
    pub fn welch_ln_t(&self) -> f64 {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_t(&moments1, &moments2)
    }

    /// Degrees of freedom for Welch's t-test for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    ///
    /// See [Welch's t-test](https://en.wikipedia.org/wiki/Welch%27s_t-test)
    pub fn welch_ln_deg_freedom(&self) -> f64 {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_deg_freedom(&moments1, &moments2)
    }

    /// Confidence interval for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    /// with confidence level of `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are log-normal. This assumption is widely supported by
    /// performance analysis theory and empirical data.
    ///
    /// This is also the confidence interval for the difference of medians of logarithms under the above assumption.
    pub fn welch_ln_ci(&self, alpha: f64) -> Ci {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_ci(&moments1, &moments2, alpha)
    }

    /// Confidence interval for
    /// `median(latency(f1)) / median(latency(f2))`,
    /// with confidence level of `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are log-normal. This assumption is widely supported by
    /// performance analysis theory and empirical data.
    pub fn welch_ratio_ci(&self, alpha: f64) -> Ci {
        let Ci(log_low, log_high) = self.welch_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        Ci(low, high)
    }

    pub fn welch_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.welch_ratio_ci(alpha);
        ci.position_of(value)
    }

    pub fn welch_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_test(&moments1, &moments2, alt_hyp, alpha)
    }

    #[cfg(feature = "dev_utils")]
    pub fn student_diff_t(&self) -> f64 {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2,
            self.sum2_diff_f1_f2,
        );
        student_one_sample_t(&moments, 0.0)
    }

    #[cfg(feature = "dev_utils")]
    pub fn student_diff_deg_freedom(&self) -> f64 {
        self.n() - 1.0
    }

    #[cfg(feature = "dev_utils")]
    pub fn student_diff_ci(&self, alpha: f64) -> Ci {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2,
            self.sum2_diff_f1_f2,
        );
        student_one_sample_ci(&moments, alpha)
    }

    #[cfg(feature = "dev_utils")]
    pub fn student_value_position_wrt_diff_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.student_diff_ci(alpha);
        ci.position_of(value)
    }

    #[cfg(feature = "dev_utils")]
    pub fn student_diff_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2,
            self.sum2_diff_f1_f2,
        );
        student_one_sample_test(&moments, 0.0, alt_hyp, alpha)
    }

    pub fn student_diff_ln_t(&self) -> f64 {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2,
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_t(&moments, 0.0)
    }

    pub fn student_diff_ln_deg_freedom(&self) -> f64 {
        self.n() - 1.0
    }

    pub fn student_diff_ln_ci(&self, alpha: f64) -> Ci {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2,
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_ci(&moments, alpha)
    }
    pub fn student_ratio_ci(&self, alpha: f64) -> Ci {
        let Ci(log_low, log_high) = self.student_diff_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        Ci(low, high)
    }

    pub fn student_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.student_ratio_ci(alpha);
        ci.position_of(value)
    }

    pub fn student_diff_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2,
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_test(&moments, 0.0, alt_hyp, alpha)
    }

    #[cfg(feature = "dev_utils")]
    pub fn wilcoxon_rank_sum_z(&self) -> f64 {
        statistics::wilcoxon_rank_sum_z(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "dev_utils")]
    pub fn wilcoxon_rank_sum_z_no_ties_adjust(&self) -> f64 {
        statistics::wilcoxon_rank_sum_z_no_ties_adjust(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "dev_utils")]
    pub fn wilcoxon_rank_sum_p(&self, alt_hyp: AltHyp) -> f64 {
        statistics::wilcoxon_rank_sum_p(&self.hist_f1, &self.hist_f2, alt_hyp)
    }

    #[cfg(feature = "dev_utils")]
    pub fn wilcoxon_rank_sum_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        statistics::wilcoxon_rank_sum_test(&self.hist_f1, &self.hist_f2, alt_hyp, alpha)
    }
}

pub(crate) type DiffState = DiffOut;

impl DiffState {
    pub(crate) fn reset(&mut self) {
        self.hist_f1.reset();
        self.hist_f2.reset();
        self.hist_f1_lt_f2.reset();
        self.count_f1_eq_f2 = 0;
        self.hist_f1_gt_f2.reset();
        self.sum_ln_f1 = 0.0;
        self.sum2_ln_f1 = 0.0;
        self.sum_ln_f2 = 0.0;
        self.sum2_ln_f2 = 0.0;
        self.sum_diff_f1_f2 = 0.0;
        self.sum2_diff_f1_f2 = 0.0;
        self.sum_diff_ln_f1_f2 = 0.0;
        self.sum2_diff_ln_f1_f2 = 0.0;
    }

    #[inline(always)]
    pub(crate) fn capture_data(&mut self, elapsed1: u64, elapsed2: u64) {
        self.hist_f1
            .record(elapsed1)
            .expect("can't happen: histogram is auto-resizable");
        self.hist_f2
            .record(elapsed2)
            .expect("can't happen: histogram is auto-resizable");

        let diff = elapsed1 as i64 - elapsed2 as i64;

        if diff < 0 {
            self.hist_f1_lt_f2
                .record(diff as u64)
                .expect("can't happen: histogram is auto-resizable");
        } else if diff > 0 {
            self.hist_f1_gt_f2
                .record(-diff as u64)
                .expect("can't happen: histogram is auto-resizable");
        } else {
            self.count_f1_eq_f2 += 1;
        }

        assert!(elapsed1 > 0, "f1 latency must be > 0");
        let ln_f1 = (elapsed1 as f64).ln();
        self.sum_ln_f1 += ln_f1;
        self.sum2_ln_f1 += ln_f1.powi(2);

        assert!(elapsed2 > 0, "f2 latency must be > 0");
        let ln_f2 = (elapsed2 as f64).ln();
        self.sum_ln_f2 += ln_f2;
        self.sum2_ln_f2 += ln_f2.powi(2);

        let diff_f1_f2 = elapsed1 as f64 - elapsed2 as f64;
        self.sum_diff_f1_f2 += diff_f1_f2;
        self.sum2_diff_f1_f2 += diff_f1_f2.powi(2);

        let diff_ln_f1_f2 = ln_f1 - ln_f2;
        self.sum_diff_ln_f1_f2 += diff_ln_f1_f2;
        self.sum2_diff_ln_f1_f2 += diff_ln_f1_f2.powi(2);
    }

    fn execute(
        &mut self,
        unit: LatencyUnit,
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        exec_count: usize,
        pre_exec: impl Fn(),
        mut exec_status: impl FnMut(),
    ) {
        pre_exec();

        for _ in 1..=exec_count / 2 {
            let pairs = duo_exec(&mut f1, &mut f2);

            for (latency1, latency2) in pairs {
                let elapsed1 = unit.latency_as_u64(latency1);
                let elapsed2 = unit.latency_as_u64(latency2);
                self.capture_data(elapsed1, elapsed2);
            }

            exec_status();
        }
    }

    fn warm_up(
        &mut self,
        unit: LatencyUnit,
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        mut warm_up_status: impl FnMut(usize, u64, u64),
    ) {
        let start = Instant::now();
        for i in 1.. {
            self.execute(unit, &mut f1, &mut f2, WARMUP_INCREMENT_COUNT, || {}, || {});
            let elapsed = Instant::now().duration_since(start);
            warm_up_status(i, elapsed.as_millis() as u64, WARMUP_MILLIS);
            if elapsed.ge(&Duration::from_millis(WARMUP_MILLIS)) {
                break;
            }
        }
    }

    fn merge_reversed(&mut self, other: DiffState) -> Result<(), Box<dyn Error>> {
        self.hist_f1.add(other.hist_f2)?;
        self.hist_f2.add(other.hist_f1)?;
        self.hist_f1_lt_f2.add(other.hist_f1_gt_f2)?;
        self.count_f1_eq_f2 += other.count_f1_eq_f2;
        self.hist_f1_gt_f2.add(other.hist_f1_lt_f2)?;
        self.sum_ln_f1 += other.sum_ln_f2;
        self.sum2_ln_f1 += other.sum2_ln_f2;
        self.sum_ln_f2 += other.sum_ln_f1;
        self.sum2_ln_f2 += other.sum2_ln_f1;
        self.sum_diff_f1_f2 -= other.sum_diff_f1_f2;
        self.sum2_diff_f1_f2 += other.sum2_diff_f1_f2;
        self.sum_diff_ln_f1_f2 -= other.sum_diff_ln_f1_f2;
        self.sum2_diff_ln_f1_f2 += other.sum2_diff_ln_f1_f2;

        Ok(())
    }
}

/// Compares the difference of total latency for two closures `f1` and `f2` in ***microseconds***.
/// Differences (latency(f1) - latency(f2)) are collected in two [`Histogram`]s, one for positive differences and the
/// other for negative differences.
///
/// Arguments:
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of outer loop repetitions. For each iteration, the inner loop (see below) is executed for
///   each of the target closures.
/// - `inner_count` - number of inner loop repetitions. Within each outer loop iteration and for each of the target closures,
///   the target closure is executed `inner_count times`, the total latency for the inner loop is measured for the
///   target closure for the inner loop. The mean difference `(total_latency(f1) - total_latency(f2)) / inner_count` is
///   calculated. Depending on whether the mean difference is positive or negative, it is recorded on the histogram
///   `hist_f1_ge_f2` or `hist_f1_lt_f2`, respectively.
/// - `f_args_str` - string that documents relevant arguments enclosed by the closures `f1` and `f2` (e.g., using the
///   `format!` macro). It is printed together with `exec_count` and `inner_count` to provide context for the benchmark.
///
/// The benchmark is warmed-up with one additional initial outer loop iteration for which measurements are not collected.
pub fn bench_diff_x(
    unit: LatencyUnit,
    mut f1: impl FnMut(),
    mut f2: impl FnMut(),
    exec_count: usize,
    mut warm_up_status: impl FnMut(usize, u64, u64),
    pre_exec: impl Fn(),
    mut exec_status: impl FnMut(),
) -> DiffOut {
    let exec_count2 = exec_count / 2;

    let mut state = DiffState::new();
    state.warm_up(unit, &mut f1, &mut f2, &mut warm_up_status);
    state.reset();
    state.execute(
        unit,
        &mut f1,
        &mut f2,
        exec_count2,
        pre_exec,
        &mut exec_status,
    );

    let mut state_rev = DiffState::new();
    state_rev.execute(unit, &mut f2, &mut f1, exec_count2, || (), &mut exec_status);

    state
        .merge_reversed(state_rev)
        .expect("state merger cannot fail");

    state
}

pub fn bench_diff(
    unit: LatencyUnit,
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
) -> DiffOut {
    bench_diff_x(unit, f1, f2, exec_count, |_, _, _| {}, || (), || ())
}

pub fn bench_diff_print(
    unit: LatencyUnit,
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
    print_sub_header: impl Fn(),
) -> DiffOut {
    println!("\n>>> bench_diff: unit={unit:?}, exec_count={exec_count}");
    print_sub_header();
    println!();

    let warm_up_status = {
        let mut status_len: usize = 0;

        move |_: usize, elapsed_millis: u64, warm_up_millis: u64| {
            if status_len == 0 {
                print!("Warming up ... ");
                stdout().flush().expect("unexpected I/O error");
            }
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{elapsed_millis} millis of {warm_up_millis}.");
            if elapsed_millis.lt(&warm_up_millis) {
                status_len = status.len();
            } else {
                status_len = 0; // reset status in case of multiple warm-up phases
            };
            eprint!("{status}");
            stderr().flush().expect("unexpected I/O error");
        }
    };

    let pre_exec = || {
        print!(" Executing bench_diff ... ");
        stdout().flush().expect("unexpected I/O error");
    };

    let exec_status = {
        let mut status_len: usize = 0;
        let mut i = 0;

        move || {
            i += 2; // account for duos
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{i} of {exec_count}.");
            status_len = status.len();
            eprint!("{status}");
            stdout().flush().expect("unexpected I/O error");
        }
    };

    let diff_out = bench_diff_x(
        unit,
        f1,
        f2,
        exec_count,
        warm_up_status,
        pre_exec,
        exec_status,
    );

    println!();

    diff_out
}

#[cfg(test)]
#[cfg(feature = "test_support")]
mod test {
    use super::*;
    use crate::{
        dev_utils::nest_btree_map,
        test_support::{
            ALPHA, BETA, Claim, ClaimResults, HI_1PCT_FACTOR, HI_10PCT_FACTOR, HI_25PCT_FACTOR,
            ScaleParams, default_hi_stdev_log, default_lo_stdev_log, get_scale_params,
            get_scenario,
        },
    };
    use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
    use rand_distr::LogNormal;
    use std::{fmt::Debug, ops::Deref};

    enum MyFnMut {
        Det {
            median: f64,
        },

        NonDet {
            median: f64,
            lognormal: LogNormal<f64>,
            rng: StdRng,
        },
    }

    impl MyFnMut {
        fn new_deterministic(median: f64) -> Self {
            Self::Det { median }
        }

        fn new_non_deterministic(median: f64, stdev_log: f64) -> Self {
            let mu = 0.0_f64;
            let sigma = stdev_log;
            Self::NonDet {
                median,
                lognormal: LogNormal::new(mu, sigma).expect("stdev_log must be > 0"),
                rng: StdRng::from_rng(&mut rand::rng()),
            }
        }

        pub fn invoke(&mut self) -> f64 {
            match self {
                Self::Det { median } => *median,

                Self::NonDet {
                    median,
                    lognormal,
                    rng,
                } => {
                    let factor = lognormal.sample(rng);
                    *median * factor
                }
            }
        }
    }

    const NAMED_FNS: [(&str, fn(f64) -> MyFnMut); 12] = {
        [
            ("base_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median)
            }),
            ("hi_1pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_1PCT_FACTOR)
            }),
            ("hi_10pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_10PCT_FACTOR)
            }),
            ("hi_25pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_25PCT_FACTOR)
            }),
            ("base_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median, default_lo_stdev_log())
            }),
            ("hi_1pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_1PCT_FACTOR, default_lo_stdev_log())
            }),
            ("hi_10pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(
                    base_median * HI_10PCT_FACTOR,
                    default_lo_stdev_log(),
                )
            }),
            ("hi_25pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(
                    base_median * HI_25PCT_FACTOR,
                    default_lo_stdev_log(),
                )
            }),
            ("base_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median, default_hi_stdev_log())
            }),
            ("hi_1pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_1PCT_FACTOR, default_hi_stdev_log())
            }),
            ("hi_10pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(
                    base_median * HI_10PCT_FACTOR,
                    default_hi_stdev_log(),
                )
            }),
            ("hi_25pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(
                    base_median * HI_25PCT_FACTOR,
                    default_hi_stdev_log(),
                )
            }),
        ]
    };

    fn get_fn(name: &str) -> fn(f64) -> MyFnMut {
        NAMED_FNS
            .iter()
            .find(|pair| pair.0 == name)
            .expect(&format!("invalid fn name: {name}"))
            .1
    }

    fn diff_x(
        mut f1: impl FnMut() -> f64,
        mut f2: impl FnMut() -> f64,
        exec_count: usize,
    ) -> DiffOut {
        let mut state = DiffState::new();

        for _ in 1..=exec_count {
            let (elapsed1, elapsed2) = (f1() as u64, f2() as u64);
            state.capture_data(elapsed1, elapsed2);
        }

        state
    }

    fn run_with_claims<T: Deref<Target = str> + Debug>(
        scale_params: &ScaleParams,
        name1: T,
        name2: T,
        verbose: bool,
        nrepeats: usize,
        run_name: &str,
    ) {
        let print_args = || {
            println!("*** arguments ***");
            println!("SCALE_NAME=\"{}\"", scale_params.name);
            println!(
                "unit={:?}, exec_count={}, base_median={}",
                scale_params.unit, scale_params.exec_count, scale_params.base_median
            );
            println!("FN_NAME_PAIR=\"({name1:?}, {name2:?})\"");
            println!("VERBOSE=\"{verbose}\"");
            println!("nrepeats={nrepeats}");
            println!("run_name=\"{run_name}\"");
        };

        println!();
        print_args();
        println!();

        let scenario = get_scenario(&name1, &name2);

        let mut f1 = {
            let mut my_fn = get_fn(&name1)(scale_params.base_median);
            move || my_fn.invoke()
        };

        let mut f2 = {
            let mut my_fn = get_fn(&name2)(scale_params.base_median);
            move || my_fn.invoke()
        };

        let mut results = ClaimResults::new();

        for _ in 1..=nrepeats {
            let diff_out = diff_x(&mut f1, &mut f2, scale_params.exec_count);
            scenario.check_claims(&mut results, &diff_out, verbose);
        }

        if verbose {
            println!("*** failures ***");
            for claim_result in results.failures().iter() {
                println!("{claim_result:?}");
            }

            println!();
            println!("*** failure_summary ***");
            for ((name_pair, claim_name), count) in results.failure_summary() {
                println!("{name_pair:?} | {claim_name} ==> count={count}");
            }

            println!();
            println!("*** success_summary ***");
            for (name_pair, claim_name) in results.success_summary() {
                println!("{name_pair:?} | {claim_name}");
            }
        } else {
            println!("*** claim_summary ***");
            for ((name_pair, claim_name), count) in results.summary() {
                println!("{name_pair:?} | {claim_name} ==> count={count}");
            }
        }

        let type_i_and_ii_errors =
            results.type_i_and_ii_errors(ALPHA, BETA, &Claim::CRITICAL_NAMES, nrepeats);
        assert!(
            type_i_and_ii_errors.is_empty(),
            "\n*** type_i_and_ii_errors: {:?}\n",
            nest_btree_map(type_i_and_ii_errors)
        );
    }

    const SCALE_NAMES: [&'static str; 1] = [
        "micros_scale",
        // "millis_scale",
        // "nanos_scale"
    ];

    #[test]
    fn test_base_median_lo_var_base_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "base_median_lo_var",
                false,
                300,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_base_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "base_median_hi_var",
                false,
                300,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_1pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_1pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_10pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_10pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_25pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_25pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    // Below test always fails due to insufficient sample size for required BETA.
    // #[test]
    // fn test_base_median_lo_var_hi_1pct_median_hi_var() {
    //     for name in SCALE_NAMES {
    //         let scale = get_scale_params(name);
    //         run_with_claims(
    //             scale,
    //             "base_median_lo_var",
    //             "hi_1pct_median_hi_var",
    //             false,
    //             100,
    //             "test",
    //         );
    //     }
    // }

    #[test]
    fn test_base_median_lo_var_hi_10pct_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_10pct_median_hi_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_25pct_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_25pct_median_hi_var",
                false,
                100,
                "test",
            );
        }
    }
}

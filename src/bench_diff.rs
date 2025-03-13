//! Module to compare the difference in latency between two closures.

use crate::{
    SummaryStats, Timing, new_timing,
    statistics::{
        HypTestResult, PositionWrtCi, SampleMoments, bernoulli_psucc_ci, bernoulli_test,
        sample_mean, sample_stdev, student_one_sample_ci, student_one_sample_t,
        student_one_sample_test, welch_ci, welch_deg_freedom, welch_t, welch_test,
    },
    summary_stats,
};
use core::f64;
use hdrhistogram::Histogram;
use std::{
    error::Error,
    io::{Write, stdout},
    time::{Duration, Instant},
};

#[cfg(feature = "wilcoxon")]
use crate::statistics::{self, AltHyp};

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
fn quad_exec(mut f1: impl FnMut(), mut f2: impl FnMut()) -> [(Duration, Duration); 4] {
    let l01 = latency(&mut f1);
    let l02 = latency(&mut f2);

    let l11 = latency(&mut f1);
    let l12 = latency(&mut f2);

    let l22 = latency(&mut f2);
    let l21 = latency(&mut f1);

    let l32 = latency(&mut f2);
    let l31 = latency(&mut f1);

    [(l01, l02), (l11, l12), (l21, l22), (l31, l32)]
}

pub struct BenchDiffOut {
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

impl BenchDiffOut {
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

    pub fn median_f1(&self) -> f64 {
        self.summary_f1().median as f64
    }

    pub fn median_f2(&self) -> f64 {
        self.summary_f2().median as f64
    }

    // Ratio of medians computed from histograms.
    pub fn ratio_medians_f1_f2(&self) -> f64 {
        self.median_f1() / self.median_f2()
    }

    // Alternative ratio of medians computed from the `exp` of the mean difference of `ln`s of latencies.
    pub fn ratio_medians_f1_f2_from_lns(&self) -> f64 {
        self.mean_diff_ln_f1_f2().exp()
    }

    /// Estimator of mean of Bernoulli distribution.
    pub fn bernoulli_prob(&self) -> f64 {
        (self.count_f1_lt_f2() as f64 + self.count_f1_eq_f2 as f64 / 2.0)
            / (self.count_f1_lt_f2() + self.count_f1_eq_f2 + self.count_f1_gt_f2()) as f64
    }

    /// Confidence interval for Bernoulli distribution (Wilson score interval).
    pub fn bernoulli_prob_ci(&self, alpha: f64) -> (f64, f64) {
        let p = self.bernoulli_prob();
        let n = self.n();
        bernoulli_psucc_ci(n, p, alpha)
    }

    pub fn bernoulli_prob_value_position_wrt_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let (low, high) = self.bernoulli_prob_ci(alpha);
        PositionWrtCi::position_of_value(value, low, high)
    }

    pub fn bernoulli_prob_eq_half_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let p_hat = self.bernoulli_prob();
        bernoulli_test(p_hat, 1.0 / 2.0, alt_hyp, alpha)
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
    pub fn welch_ln_ci(&self, alpha: f64) -> (f64, f64) {
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
    pub fn welch_ratio_ci(&self, alpha: f64) -> (f64, f64) {
        let (log_low, log_high) = self.welch_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        (low, high)
    }

    pub fn welch_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let (low, high) = self.welch_ratio_ci(alpha);
        PositionWrtCi::position_of_value(value, low, high)
    }

    pub fn welch_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_test(&moments1, &moments2, alt_hyp, alpha)
    }

    pub fn student_diff_t(&self) -> f64 {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2,
            self.sum2_diff_f1_f2,
        );
        student_one_sample_t(&moments, 0.0)
    }

    pub fn student_diff_deg_freedom(&self) -> f64 {
        self.n() - 1.0
    }

    pub fn student_diff_ci(&self, alpha: f64) -> (f64, f64) {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2,
            self.sum2_diff_f1_f2,
        );
        student_one_sample_ci(&moments, alpha)
    }

    pub fn student_value_position_wrt_diff_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let (low, high) = self.student_diff_ci(alpha);
        PositionWrtCi::position_of_value(value, low, high)
    }

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

    pub fn student_diff_ln_ci(&self, alpha: f64) -> (f64, f64) {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2,
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_ci(&moments, alpha)
    }
    pub fn student_ratio_ci(&self, alpha: f64) -> (f64, f64) {
        let (log_low, log_high) = self.student_diff_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        (low, high)
    }

    pub fn student_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let (low, high) = self.student_ratio_ci(alpha);
        PositionWrtCi::position_of_value(value, low, high)
    }

    pub fn student_diff_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2,
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_test(&moments, 0.0, alt_hyp, alpha)
    }

    #[cfg(feature = "wilcoxon")]
    pub fn wilcoxon_rank_sum_z(&self) -> f64 {
        statistics::wilcoxon_rank_sum_z(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "wilcoxon")]
    pub fn wilcoxon_rank_sum_z_no_ties_adjust(&self) -> f64 {
        statistics::wilcoxon_rank_sum_z_no_ties_adjust(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "wilcoxon")]
    pub fn wilcoxon_rank_sum_p(&self, alt_hyp: AltHyp) -> f64 {
        statistics::wilcoxon_rank_sum_p(&self.hist_f1, &self.hist_f2, alt_hyp)
    }

    #[cfg(feature = "wilcoxon")]
    pub fn wilcoxon_rank_sum_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        statistics::wilcoxon_rank_sum_test(&self.hist_f1, &self.hist_f2, alt_hyp, alpha)
    }
}

type BenchDiffState = BenchDiffOut;

impl BenchDiffState {
    fn new() -> BenchDiffState {
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

    fn reset(&mut self) {
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

    fn execute(
        &mut self,
        unit: LatencyUnit,
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        exec_count: usize,
        pre_exec: impl Fn(),
        mut exec_status: impl FnMut(usize),
    ) {
        pre_exec();

        for i in 1..=exec_count / 4 {
            let pairs = quad_exec(&mut f1, &mut f2);

            for (latency1, latency2) in pairs {
                let elapsed1 = unit.latency_as_u64(latency1);
                let elapsed2 = unit.latency_as_u64(latency2);

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

            exec_status(i * 4);
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
            self.execute(
                unit,
                &mut f1,
                &mut f2,
                WARMUP_INCREMENT_COUNT,
                || {},
                |_| {},
            );
            let elapsed = Instant::now().duration_since(start);
            warm_up_status(i, elapsed.as_millis() as u64, WARMUP_MILLIS);
            if elapsed.ge(&Duration::from_millis(WARMUP_MILLIS)) {
                break;
            }
        }
    }

    fn merge_reversed(&mut self, other: BenchDiffState) -> Result<(), Box<dyn Error>> {
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
    mut exec_status: impl FnMut(usize),
) -> BenchDiffOut {
    let exec_count2 = exec_count / 2;

    let mut state = BenchDiffState::new();
    state.warm_up(unit, &mut f1, &mut f2, &mut warm_up_status);
    state.reset();
    state.execute(
        unit,
        &mut f1,
        &mut f2,
        exec_count2,
        &pre_exec,
        &mut exec_status,
    );

    let mut state_rev = BenchDiffState::new();
    // state_rev.warm_up(unit, &mut f2, &mut f1, &mut warm_up_status);
    // state_rev.reset();
    state_rev.execute(
        unit,
        &mut f2,
        &mut f1,
        exec_count2,
        &pre_exec,
        &mut exec_status,
    );

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
) -> BenchDiffOut {
    bench_diff_x(unit, f1, f2, exec_count, |_, _, _| {}, || (), |_| ())
}

pub fn bench_diff_print(
    unit: LatencyUnit,
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
    print_sub_header: impl Fn(),
    print_stats: impl Fn(&BenchDiffOut),
) -> BenchDiffOut {
    println!("\n>>> bench_diff: unit={unit:?}, exec_count={exec_count}");
    print_sub_header();
    println!();

    let warm_up_status = {
        let mut status_len: usize = 0;
        let mut phase = 1;

        move |_: usize, elapsed_millis: u64, warm_up_millis: u64| {
            if status_len == 0 {
                print!("Phase {phase}: Warming up ... ");
                phase += 1;
                stdout().flush().expect("unexpected I/O error");
            }
            print!("{}", "\u{8}".repeat(status_len));
            let status = format!("{elapsed_millis} millis of {warm_up_millis}.");
            if elapsed_millis.lt(&warm_up_millis) {
                status_len = status.len();
            } else {
                status_len = 0;
            };
            print!("{status}");
            stdout().flush().expect("unexpected I/O error");
        }
    };

    let pre_exec = || {
        print!(" Executing bench_diff: ");
        stdout().flush().expect("unexpected I/O error");
    };

    let exec_status = {
        let exec_count2 = exec_count / 2;
        let mut status_len: usize = 0;

        move |i| {
            print!("{}", "\u{8}".repeat(status_len));
            let status = format!("{i}/{exec_count2}.");
            status_len = status.len();
            print!("{status}");
            stdout().flush().expect("unexpected I/O error");
            if i >= exec_count2 {
                status_len = 0;
                println!();
            }
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

    print_stats(&diff_out);

    diff_out
}

//! Module to compare the difference in latency between two closures.

use crate::{
    new_timing, sample_mean, sample_stdev, sample_sum2_deviations, statistics, summary_stats,
    PositionInCi, SummaryStats, Timing,
};
use hdrhistogram::Histogram;
use statrs::distribution::{ContinuousCDF, StudentsT};
use std::{
    error::Error,
    hint,
    io::{stdout, Write},
    time::{Duration, Instant},
};

const WARMUP_MILLIS: u64 = 3_000;
const WARMUP_INCREMENT_COUNT: usize = 20;

#[derive(Clone, Copy, Debug)]
pub enum LatencyUnit {
    Milli,
    Micro,
    Nano,
}

#[inline(always)]
pub fn latency(unit: LatencyUnit, mut f: impl FnMut()) -> u64 {
    let start = Instant::now();
    f();
    let duration = Instant::now().duration_since(start);
    match unit {
        LatencyUnit::Milli => duration.as_millis() as u64,
        LatencyUnit::Micro => duration.as_micros() as u64,
        LatencyUnit::Nano => duration.as_nanos() as u64,
    }
}

#[inline(always)]
fn quad_exec(unit: LatencyUnit, mut f1: impl FnMut(), mut f2: impl FnMut()) -> [(u64, u64); 4] {
    let l01 = latency(unit, &mut f1);
    let l02 = latency(unit, &mut f2);

    let l11 = latency(unit, &mut f1);
    let l12 = latency(unit, &mut f2);

    let l22 = latency(unit, &mut f2);
    let l21 = latency(unit, &mut f1);

    let l32 = latency(unit, &mut f2);
    let l31 = latency(unit, &mut f1);

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

    /// Welch's t statistic for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    ///
    /// See [Welch's t-test](https://en.wikipedia.org/wiki/Welch%27s_t-test)
    pub fn welch_ln_t(&self) -> f64 {
        let n = self.n();
        let dx = self.mean_ln_f1() - self.mean_ln_f2();
        let s2_x1 = self.stdev_ln_f1().powi(2) / n;
        let s2_x2 = self.stdev_ln_f2().powi(2) / n;
        let s_dx = (s2_x1 + s2_x2).sqrt();
        dx / s_dx
    }

    /// Degrees of freedom for Welch's t-test for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    ///
    /// See [Welch's t-test](https://en.wikipedia.org/wiki/Welch%27s_t-test)
    pub fn welch_ln_deg_freedom(&self) -> f64 {
        let n = self.n();
        let s2_x1 = self.stdev_ln_f1().powi(2) / n;
        let s2_x2 = self.stdev_ln_f2().powi(2) / n;
        let s2_dx = s2_x1 + s2_x2;
        (n - 1.0) * s2_dx.powi(2) / (s2_x1.powi(2) + s2_x2.powi(2))
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
        let n = self.n();
        let dx = self.mean_ln_f1() - self.mean_ln_f2();
        let s2_x1 = self.stdev_ln_f1().powi(2) / n;
        let s2_x2 = self.stdev_ln_f2().powi(2) / n;
        let nu = self.welch_ln_deg_freedom();

        let stud = StudentsT::new(0.0, 1.0, nu)
            .expect("can't happen: degrees of freedom is always >= 3 by construction");
        let t = -stud.inverse_cdf(alpha / 2.0);

        let mid = dx;
        let radius = (s2_x1 + s2_x2).sqrt() * t;

        (mid - radius, mid + radius)
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

    pub fn welch_position_of_1_in_ratio_ci(&self, alpha: f64) -> PositionInCi {
        let (low, high) = self.welch_ratio_ci(alpha);
        PositionInCi::position_of_value(1.0, low, high)
    }

    pub fn student_diff_t(&self) -> f64 {
        let n = self.n();
        let dx = self.mean_diff_f1_f2();
        let s_dx = self.stdev_diff_f1_f2();
        dx / s_dx * n.sqrt()
    }

    pub fn student_diff_deg_freedom(&self) -> f64 {
        self.n() - 1.0
    }

    pub fn student_diff_ci(&self, alpha: f64) -> (f64, f64) {
        let nu = self.student_diff_deg_freedom();
        let stud = StudentsT::new(0.0, 1.0, nu)
            .expect("can't happen: degrees of freedom is always >= 3 by construction");
        let t = -stud.inverse_cdf(alpha / 2.0);

        let mid = self.mean_diff_f1_f2();
        let radius = (self.stdev_diff_f1_f2() / self.n().sqrt()) * t;

        (mid - radius, mid + radius)
    }

    pub fn student_position_of_0_in_diff_ci(&self, alpha: f64) -> PositionInCi {
        let (low, high) = self.student_diff_ci(alpha);
        PositionInCi::position_of_value(0.0, low, high)
    }

    pub fn student_diff_ln_t(&self) -> f64 {
        let n = self.n();
        let dx = self.mean_diff_ln_f1_f2();
        let s_dx = self.stdev_diff_ln_f1_f2();
        dx / s_dx * n.sqrt()
    }

    pub fn student_diff_ln_deg_freedom(&self) -> f64 {
        self.n() - 1.0
    }

    pub fn student_diff_ln_ci(&self, alpha: f64) -> (f64, f64) {
        let nu = self.student_diff_ln_deg_freedom();
        let stud = StudentsT::new(0.0, 1.0, nu)
            .expect("can't happen: degrees of freedom is always >= 3 by construction");
        let t = -stud.inverse_cdf(alpha / 2.0);

        let mid = self.mean_diff_ln_f1_f2();
        let radius = (self.stdev_diff_ln_f1_f2() / self.n().sqrt()) * t;

        (mid - radius, mid + radius)
    }
    pub fn student_ratio_ci(&self, alpha: f64) -> (f64, f64) {
        let (log_low, log_high) = self.student_diff_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        (low, high)
    }

    pub fn student_position_of_1_in_ratio_ci(&self, alpha: f64) -> PositionInCi {
        let (low, high) = self.student_ratio_ci(alpha);
        PositionInCi::position_of_value(1.0, low, high)
    }

    pub fn wilcoxon_rank_sum_f1_lt_f2_p(&self) -> f64 {
        statistics::wilcoxon_rank_sum_a_lt_b_p(&self.hist_f1, &self.hist_f2)
    }

    pub fn wilcoxon_rank_sum_f1_gt_f2_p(&self) -> f64 {
        statistics::wilcoxon_rank_sum_a_gt_b_p(&self.hist_f1, &self.hist_f2)
    }

    pub fn wilcoxon_rank_sum_f1_ne_f2_p(&self) -> f64 {
        statistics::wilcoxon_rank_sum_a_ne_b_p(&self.hist_f1, &self.hist_f2)
    }
}

type BenchDiffState = BenchDiffOut;

impl BenchDiffState {
    fn new() -> BenchDiffState {
        let hist_f1_lt_f2 = new_timing(20 * 1000 * 1000, 5);
        let count_f1_eq_f2 = 0;
        let hist_f1_gt_f2 = Histogram::<u64>::new_from(&hist_f1_lt_f2);
        let hist_f1 = Histogram::<u64>::new_from(&hist_f1_lt_f2);
        let hist_f2 = Histogram::<u64>::new_from(&hist_f1_lt_f2);
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
            let pairs = quad_exec(unit, &mut f1, &mut f2);

            for (elapsed1, elapsed2) in pairs {
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

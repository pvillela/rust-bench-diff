//! Module defining the key data structure produced by [`crate::bench_diff`].

use crate::{
    SummaryStats, Timing,
    basic_stats::{
        core::{
            AltHyp, Ci, HypTestResult, PositionWrtCi, SampleMoments, sample_mean, sample_stdev,
        },
        normal::{
            student_one_sample_ci, student_one_sample_t, student_one_sample_test, welch_ci,
            welch_df, welch_t, welch_test,
        },
    },
    new_timing, summary_stats,
};
use hdrhistogram::Histogram;

#[cfg(feature = "_dev_support")]
use crate::basic_stats::{bernoulli, wilcoxon};

/// Contains the data resulting from a benchmark comparing two closures `f1` and `f2`.
///
/// It is returned by the core benchmarking functions in this library.
/// Its methods provide descriptive and inferential statistics about the latency samples of the two
/// benchmarked functions, individually and in relation to each other.
///
/// All statistics involving differences refer to a value for `f1` minus the corresponding
/// value for `f2`.
pub struct DiffOut {
    pub(super) hist_f1: Timing,
    pub(super) hist_f2: Timing,
    pub(super) hist_f1_lt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    pub(super) count_f1_eq_f2: u64,
    pub(super) hist_f1_gt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    pub(super) sum_f1: i64,
    pub(super) sum_f2: i64,
    pub(super) sum_ln_f1: f64,
    pub(super) sum2_ln_f1: f64,
    pub(super) sum_ln_f2: f64,
    pub(super) sum2_ln_f2: f64,
    pub(super) sum2_diff_f1_f2: i64,
    pub(super) sum2_diff_ln_f1_f2: f64,
}

impl DiffOut {
    /// Creates a new empty instance.
    pub(crate) fn new() -> Self {
        let hist_f1 = new_timing(20 * 1000 * 1000, 5);
        let hist_f2 = Histogram::<u64>::new_from(&hist_f1);
        let hist_f1_lt_f2 = Histogram::<u64>::new_from(&hist_f1);
        let count_f1_eq_f2 = 0;
        let hist_f1_gt_f2 = Histogram::<u64>::new_from(&hist_f1);
        let sum_f1 = 0;
        let sum_f2 = 0;
        let sum_ln_f1 = 0.;
        let sum2_ln_f1 = 0.;
        let sum_ln_f2 = 0.;
        let sum2_ln_f2 = 0.;
        let sum2_diff_f1_f2 = 0;
        let sum2_diff_ln_f1_f2 = 0.;

        Self {
            hist_f1,
            hist_f2,
            hist_f1_lt_f2,
            count_f1_eq_f2,
            hist_f1_gt_f2,
            sum_f1,
            sum_f2,
            sum_ln_f1,
            sum2_ln_f1,
            sum_ln_f2,
            sum2_ln_f2,
            sum2_diff_f1_f2,
            sum2_diff_ln_f1_f2,
        }
    }

    /// Number of observations (sample size) for a function, as an integer.
    ///
    /// It is the same value for `f1` and `f2`.
    #[inline(always)]
    pub fn n(&self) -> u64 {
        self.hist_f1.len()
    }

    /// Number of observations (sample size) for a function, as a floating point number.
    ///
    /// It is the same value for `f1` and `f2`.
    #[inline(always)]
    pub fn nf(&self) -> f64 {
        self.hist_f1.len() as f64
    }

    /// Summary descriptive statistics for `f1`.
    ///
    /// Includes sample size, mean, standard deviation, median, several percentiles, min, and max.
    pub fn summary_f1(&self) -> SummaryStats {
        summary_stats(&self.hist_f1)
    }

    /// Summary descriptive statistics for `f2`.
    ///
    /// Includes sample size, mean, standard deviation, median, several percentiles, min, and max.
    pub fn summary_f2(&self) -> SummaryStats {
        summary_stats(&self.hist_f2)
    }

    fn sum_diff_f1_f2(&self) -> f64 {
        (self.sum_f1 - self.sum_f2) as f64
    }

    fn sum_diff_ln_f1_f2(&self) -> f64 {
        self.sum_ln_f1 - self.sum_ln_f2
    }

    /// Mean of `f1`'s latencies.
    pub fn mean_f1(&self) -> f64 {
        self.summary_f1().mean
    }

    /// Mean of `f1`'s latencies.
    pub fn mean_f2(&self) -> f64 {
        self.summary_f2().mean
    }

    /// Median of `f1`'s latencies.
    pub fn median_f1(&self) -> f64 {
        self.summary_f1().median as f64
    }

    /// Median of `f2`'s latencies.
    pub fn median_f2(&self) -> f64 {
        self.summary_f2().median as f64
    }

    /// Difference between the median of `f1`'s latencies and the median of `f2`'s latencies.
    pub fn diff_medians_f1_f2(&self) -> f64 {
        self.median_f1() - self.median_f2()
    }

    /// Ratio of the median of `f1`'s latencies to the median of `f2`'s latencies.
    pub fn ratio_medians_f1_f2(&self) -> f64 {
        self.median_f1() / self.median_f2()
    }

    #[cfg(feature = "_dev_support")]
    /// Ratio of the minimum of `f1`'s latencies to the minimum of `f2`'s latencies.
    pub fn ratio_mins_f1_f2(&self) -> f64 {
        self.summary_f1().min as f64 / self.summary_f2().min as f64
    }

    /// Count of paired observations where `f1`'s latency is less than `f2`'s.
    pub fn count_f1_lt_f2(&self) -> u64 {
        self.hist_f1_lt_f2.len()
    }

    /// Count of paired observations where `f1`'s latency is equal to `f2`'s.
    pub fn count_f1_eq_f2(&self) -> u64 {
        self.count_f1_eq_f2
    }

    /// Count of paired observations where `f1`'s latency is greater than `f2`'s.
    pub fn count_f1_gt_f2(&self) -> u64 {
        self.hist_f1_gt_f2.len()
    }

    /// Mean of the natural logarithms of `f1`'s latencies.
    pub fn mean_ln_f1(&self) -> f64 {
        sample_mean(self.n(), self.sum_ln_f1)
    }

    /// Standard deviation of the natural logarithms `f1`'s latecies.
    pub fn stdev_ln_f1(&self) -> f64 {
        sample_stdev(self.n(), self.sum_ln_f1, self.sum2_ln_f1)
    }

    /// Mean of the natural logarithms of `f2`'s latencies.
    pub fn mean_ln_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_ln_f2)
    }

    /// Standard deviation of the natural logarithms `f2`'s latecies.
    pub fn stdev_ln_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_ln_f2, self.sum2_ln_f2)
    }

    /// Mean of the differences between paired latencies of `f1` and `f2`.
    /// Equal to the difference between the mean of `f1`'s latencies and the mean of `f2`'s latencies.
    pub fn mean_diff_f1_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_diff_f1_f2())
    }

    /// Standard deviation of the differences between paired latencies of `f1` and `f2`.
    /// (*Not* the difference between the standard deviation of `f1`'s latencies and
    /// the standard deviation of`f2`'s latencies.)
    pub fn stdev_diff_f1_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_diff_f1_f2(), self.sum2_diff_f1_f2 as f64)
    }

    /// Mean of the differences between the natural logarithms of paired latencies of `f1` and `f2`.
    /// (Same as the difference between the mean of the natural logarithms of `f1`'s latencies and
    /// the mean of the natural logarithms of`f2`'s latencies.)
    pub fn mean_diff_ln_f1_f2(&self) -> f64 {
        sample_mean(self.n(), self.sum_diff_ln_f1_f2())
    }

    /// Standard deviation of the differences between the natural logarithms of paired latencies of `f1` and `f2`.
    /// (*Not* the difference between the standard deviation of the natural logarithms of `f1`'s latencies and
    /// the standard deviation of the natural logarithms of`f2`'s latencies.)
    pub fn stdev_diff_ln_f1_f2(&self) -> f64 {
        sample_stdev(self.n(), self.sum_diff_ln_f1_f2(), self.sum2_diff_ln_f1_f2)
    }

    /// Estimated ratio of the median `f1` latency to the median `f2` latency,
    /// computed as the `exp()` of [`Self::mean_diff_ln_f1_f2`].
    pub fn ratio_medians_f1_f2_from_lns(&self) -> f64 {
        self.mean_diff_ln_f1_f2().exp()
    }

    #[cfg(feature = "_dev_support")]
    /// Estimate of the probability that `f1`s latency is greater than `f2`s in a paired observation
    /// (Bernoulli distribution).
    pub fn bernoulli_prob_f1_gt_f2(&self) -> f64 {
        (self.count_f1_gt_f2() as f64 + self.count_f1_eq_f2 as f64 / 2.)
            / (self.count_f1_lt_f2() + self.count_f1_eq_f2 + self.count_f1_gt_f2()) as f64
    }

    #[cfg(feature = "_dev_support")]
    /// Confidence interval (Wilson score interval) for the mean of the Bernoulli distribution
    /// whose parameter *p* is the probability probability that `f1`s latency is greater than `f2`s
    /// (in a paired observation).
    pub fn bernoulli_ci(&self, alpha: f64) -> Ci {
        let p_hat = self.bernoulli_prob_f1_gt_f2();
        let n = self.n();
        bernoulli::bernoulli_psucc_ci(n, p_hat, alpha)
    }

    #[cfg(feature = "_dev_support")]
    /// Position of `value` with respect to the
    /// confidence interval (Wilson score interval) for the mean of the Bernoulli distribution
    /// whose parameter *p* is the probability probability that `f1`s latency is greater than `f2`s
    /// (in a paired observation).
    pub fn bernoulli_value_position_wrt_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.bernoulli_ci(alpha);
        ci.position_of(value)
    }

    #[cfg(feature = "_dev_support")]
    /// Statistical test of the hypothesis that
    /// the probability that `f1`s latency is greater than `f2`s (in a paired observation) is `p0`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    pub fn bernoulli_test(&self, p0: f64, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let p_hat = self.bernoulli_prob_f1_gt_f2();
        bernoulli::bernoulli_test(self.n(), p_hat, p0, alt_hyp, alpha)
    }

    #[cfg(feature = "_dev_support")]
    /// Statistical test of the hypothesis that
    /// the probability that `f1`s latency is greater than `f2`s (in a paired observation) is `0.5`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    pub fn bernoulli_eq_half_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        self.bernoulli_test(1. / 2., alt_hyp, alpha)
    }

    /// Welch's t statistic for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm).
    pub fn welch_ln_t(&self) -> f64 {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_t(&moments1, &moments2)
    }

    /// Degrees of freedom for Welch's t-test for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm).
    pub fn welch_ln_df(&self) -> f64 {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_df(&moments1, &moments2)
    }

    /// Welch confidence interval for
    /// `mean(ln(latency(f1))) - mean(ln(latency(f2)))` (where `ln` is the natural logarithm),
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    ///
    /// This is also the confidence interval for the difference of medians of logarithms under the above assumption.
    pub fn welch_ln_ci(&self, alpha: f64) -> Ci {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_ci(&moments1, &moments2, alpha)
    }

    /// Welch confidence interval for
    /// `median(latency(f1)) / median(latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn welch_ratio_ci(&self, alpha: f64) -> Ci {
        let Ci(log_low, log_high) = self.welch_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        Ci(low, high)
    }

    /// Position of `value` with respect to the
    /// Welch confidence interval for
    /// `median(latency(f1)) / median(latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn welch_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.welch_ratio_ci(alpha);
        ci.position_of(value)
    }

    /// Welch's test of the hypothesis that
    /// `median(latency(f1)) == median(latency(f2))`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn welch_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments1 = SampleMoments::new(self.hist_f1.len(), self.sum_ln_f1, self.sum2_ln_f1);
        let moments2 = SampleMoments::new(self.hist_f2.len(), self.sum_ln_f2, self.sum2_ln_f2);
        welch_test(&moments1, &moments2, alt_hyp, alpha)
    }

    #[cfg(feature = "_dev_support")]
    /// Student's one-sample t statistic for
    /// `mean(latency(f1) - latency(f2))`.
    pub fn student_diff_t(&self) -> f64 {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2(),
            self.sum2_diff_f1_f2 as f64,
        );
        student_one_sample_t(&moments, 0.)
    }

    #[cfg(feature = "_dev_support")]
    /// Degrees of freedom for Student's one-sample t-test for
    /// `mean(latency(f1) - latency(f2))`.
    pub fn student_diff_df(&self) -> f64 {
        self.nf() - 1.
    }

    #[cfg(feature = "_dev_support")]
    /// Student's confidence interval for
    /// `mean(latency(f1) - latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that `latency(f1) - latency(f2)` is normally distributed. This assumption is *not* supported by
    /// performance analysis theory or empirical data.
    pub fn student_diff_ci(&self, alpha: f64) -> Ci {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2(),
            self.sum2_diff_f1_f2 as f64,
        );
        student_one_sample_ci(&moments, alpha)
    }

    #[cfg(feature = "_dev_support")]
    /// Position of `value` with respect to
    /// Student's confidence interval for
    /// `mean(latency(f1) - latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that `latency(f1) - latency(f2)` is normally distributed. This assumption is *not* supported by
    /// performance analysis theory or empirical data.
    pub fn student_value_position_wrt_diff_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.student_diff_ci(alpha);
        ci.position_of(value)
    }

    #[cfg(feature = "_dev_support")]
    /// Student's one-sample test of the hypothesis that
    /// `mean(latency(f1) - latency(f2)) == 0`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    ///
    /// Assumes that `latency(f1) - latency(f2)` is normally distributed. This assumption is *not* supported by
    /// performance analysis theory or empirical data.
    pub fn student_diff_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_f1_f2(),
            self.sum2_diff_f1_f2 as f64,
        );
        student_one_sample_test(&moments, 0., alt_hyp, alpha)
    }

    /// Student's one-sample t statistic for
    /// `mean(ln(latency(f1)) - ln(latency(f2)))` (where `ln` is the natural logarithm).
    pub fn student_diff_ln_t(&self) -> f64 {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2(),
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_t(&moments, 0.)
    }

    /// Degrees of freedom for Student's one-sample t-test for
    /// `mean(ln(latency(f1)) - ln(latency(f2)))` (where `ln` is the natural logarithm).
    pub fn student_diff_ln_df(&self) -> f64 {
        self.nf() - 1.
    }

    /// Student's one-sample confidence interval for
    /// `mean(ln(latency(f1)) - ln(latency(f2)))` (where `ln` is the natural logarithm).
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn student_diff_ln_ci(&self, alpha: f64) -> Ci {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2(),
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_ci(&moments, alpha)
    }

    /// Student's one-sample confidence interval for
    /// `median(latency(f1)) / median(latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn student_ratio_ci(&self, alpha: f64) -> Ci {
        let Ci(log_low, log_high) = self.student_diff_ln_ci(alpha);
        let low = log_low.exp();
        let high = log_high.exp();
        Ci(low, high)
    }

    /// Position of `value` with respect to
    /// Student's one-sample confidence interval for
    /// `median(latency(f1)) / median(latency(f2))`,
    /// with confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn student_value_position_wrt_ratio_ci(&self, value: f64, alpha: f64) -> PositionWrtCi {
        let ci = self.student_ratio_ci(alpha);
        ci.position_of(value)
    }

    /// Student's one-sample test of the hypothesis that
    /// `median(latency(f1)) == median(latency(f2))`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    ///
    /// Assumes that both `latency(f1)` and `latency(f2)` are approximately log-normal.
    /// This assumption is widely supported by performance analysis theory and empirical data.
    pub fn student_diff_ln_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        let moments = SampleMoments::new(
            self.hist_f1.len(),
            self.sum_diff_ln_f1_f2(),
            self.sum2_diff_ln_f1_f2,
        );
        student_one_sample_test(&moments, 0., alt_hyp, alpha)
    }

    #[cfg(feature = "_dev_support")]
    /// Wilcoxon rank sum *W* statistic for `latency(f1)` and `latency(f2)`.
    pub fn wilcoxon_rank_sum_w(&self) -> f64 {
        wilcoxon::wilcoxon_rank_sum_w(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "_dev_support")]
    /// Wilcoxon rank sum normal approximation *z* value for `latency(f1)` and `latency(f2)`.
    pub fn wilcoxon_rank_sum_z(&self) -> f64 {
        wilcoxon::wilcoxon_rank_sum_z(&self.hist_f1, &self.hist_f2)
    }

    #[cfg(feature = "_dev_support")]
    /// Wilcoxon rank sum normal approximation *p* value for `latency(f1)` and `latency(f2)`.
    pub fn wilcoxon_rank_sum_p(&self, alt_hyp: AltHyp) -> f64 {
        wilcoxon::wilcoxon_rank_sum_p(&self.hist_f1, &self.hist_f2, alt_hyp)
    }

    #[cfg(feature = "_dev_support")]
    /// Wilcoxon rank sum test for for `latency(f1)` and `latency(f2)`,
    /// with alternative hypothesis `alt_hyp` and confidence level `(1 - alpha)`.
    pub fn wilcoxon_rank_sum_test(&self, alt_hyp: AltHyp, alpha: f64) -> HypTestResult {
        wilcoxon::wilcoxon_rank_sum_test(&self.hist_f1, &self.hist_f2, alt_hyp, alpha)
    }
}

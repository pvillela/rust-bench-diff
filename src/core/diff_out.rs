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

#[cfg(feature = "dev_utils")]
use crate::statistics::{self, bernoulli_psucc_ci, bernoulli_test};

pub struct DiffOut {
    pub(super) hist_f1: Timing,
    pub(super) hist_f2: Timing,
    pub(super) hist_f1_lt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    pub(super) count_f1_eq_f2: u64,
    pub(super) hist_f1_gt_f2: Timing, //todo: replace with count, sum and sum of squares of ratios
    pub(super) sum_ln_f1: f64,
    pub(super) sum2_ln_f1: f64,
    pub(super) sum_ln_f2: f64,
    pub(super) sum2_ln_f2: f64,
    pub(super) sum_diff_f1_f2: f64,
    pub(super) sum2_diff_f1_f2: f64,
    pub(super) sum_diff_ln_f1_f2: f64,
    pub(super) sum2_diff_ln_f1_f2: f64,
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

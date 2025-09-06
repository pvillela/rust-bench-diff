#[derive(Debug)]
#[allow(unused)] // Used in debug print.
/// Percentiles in this data structure are, by convention, inclusive.
pub struct MeasuredRatioSummary {
    pub target_ratio: f64,
    pub count: usize,
    pub mean: f64,
    pub stdev: f64,
    /// Root mean square deviation from target ratio.
    pub rms_dev_from_target: f64,
    pub min: f64,
    pub p1: f64,
    pub p5: f64,
    pub p10: f64,
    pub p25: f64,
    pub median: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub max: f64,
}

/// Returns summary stats for `arr`. Requires that `arr` be sorted in ascending order.
pub fn measured_ratio_summary(measured_ratios: &[f64], target_ratio: f64) -> MeasuredRatioSummary {
    let count = measured_ratios.len();
    let mean = measured_ratios.iter().sum::<f64>() / count as f64;
    let sum2_dev_mean = measured_ratios
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>();
    let stdev = (sum2_dev_mean / count as f64).sqrt();
    let sum2_dev_target = measured_ratios
        .iter()
        .map(|x| (x - target_ratio).powi(2))
        .sum::<f64>();
    let rms_dev_from_target = (sum2_dev_target / count as f64).sqrt();
    let idx_min = 0;
    let idx_p1 = (count / 100).max(1) - 1;
    let idx_p5 = (count / 20).max(1) - 1;
    let idx_p10 = (count / 10).max(1) - 1;
    let idx_p25 = (count / 4).max(1) - 1;
    let idx_median = (count / 2).max(1) - 1;
    let idx_p75 = (count - count / 4).max(1) - 1;
    let idx_p90 = (count - count / 10).max(1) - 1;
    let idx_p95 = (count - count / 20).max(1) - 1;
    let idx_p99 = (count - count / 100).max(1) - 1;
    let idx_max = count - 1;

    MeasuredRatioSummary {
        target_ratio,
        count,
        mean,
        stdev,
        rms_dev_from_target,
        min: measured_ratios[idx_min],
        p1: measured_ratios[idx_p1],
        p5: measured_ratios[idx_p5],
        p10: measured_ratios[idx_p10],
        p25: measured_ratios[idx_p25],
        median: measured_ratios[idx_median],
        p75: measured_ratios[idx_p75],
        p90: measured_ratios[idx_p90],
        p95: measured_ratios[idx_p95],
        p99: measured_ratios[idx_p99],
        max: measured_ratios[idx_max],
    }
}

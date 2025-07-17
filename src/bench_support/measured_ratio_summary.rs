#[derive(Debug)]
#[allow(unused)] // Used in debug print.
pub struct MeasuredRatioSummary {
    pub target_ratio: f64,
    pub count: usize,
    pub mean: f64,
    /// Root mean square deviation from target ratio.
    pub rms_dev_from_target: f64,
    pub min: f64,
    pub p25: f64,
    pub median: f64,
    pub p75: f64,
    pub max: f64,
}

/// Returns summary stats for `arr`. Requires that `arr` be sorted in ascending order.
pub fn measured_ratio_summary(measured_ratios: &[f64], target_ratio: f64) -> MeasuredRatioSummary {
    let count = measured_ratios.len();
    let mean = measured_ratios.iter().sum::<f64>() / count as f64;
    let sum2_dev = measured_ratios
        .iter()
        .map(|x| (x - target_ratio).powi(2))
        .sum::<f64>();
    let rms_dev_from_target = (sum2_dev / count as f64).sqrt();
    let idx_p25 = (count / 4) - 1;
    let idx_median = (count / 2) - 1;
    let idx_p75 = (count * 3 / 4) - 1;

    MeasuredRatioSummary {
        target_ratio,
        count,
        mean,
        rms_dev_from_target,
        min: measured_ratios[0],
        p25: measured_ratios[idx_p25],
        median: measured_ratios[idx_median],
        p75: measured_ratios[idx_p75],
        max: measured_ratios[count - 1],
    }
}

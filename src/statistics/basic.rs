use statrs::distribution::{ContinuousCDF, StudentsT};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionInCi {
    Below,
    In,
    Above,
}

impl PositionInCi {
    pub fn position_of_value(value: f64, low: f64, high: f64) -> Self {
        match value {
            _ if value <= low => PositionInCi::Below,
            _ if low < value && value < high => PositionInCi::In,
            _ => PositionInCi::Above,
        }
    }
}

#[inline(always)]
pub fn sample_mean(n: f64, sum: f64) -> f64 {
    sum / n
}

#[inline(always)]
pub fn sample_sum2_deviations(n: f64, sum: f64, sum2: f64) -> f64 {
    sum2 - sum.powi(2) / n
}

#[inline(always)]
pub fn sample_var(n: f64, sum: f64, sum2: f64) -> f64 {
    sample_sum2_deviations(n, sum, sum2) / (n - 1.0)
}

#[inline(always)]
pub fn sample_stdev(n: f64, sum: f64, sum2: f64) -> f64 {
    sample_var(n, sum, sum2).sqrt()
}

pub struct SampleMoments {
    count: u64,
    sum: f64,
    sum2: f64,
    min: f64,
    max: f64,
}

impl SampleMoments {
    pub fn new(count: u64, sum: f64, sum2: f64, min: f64, max: f64) -> Self {
        Self {
            count,
            sum,
            sum2,
            min,
            max,
        }
    }

    pub fn n(&self) -> f64 {
        self.count as f64
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn mean(&self) -> f64 {
        self.sum / self.n()
    }

    pub fn sum2(&self) -> f64 {
        self.sum2
    }

    pub fn sum2_deviations(&self) -> f64 {
        sample_sum2_deviations(self.n(), self.sum, self.sum2)
    }

    /// Returns `Nan` if  `self.n()` == 1
    pub fn var(&self) -> f64 {
        sample_var(self.n(), self.sum, self.sum2)
    }

    /// Returns `Nan` if  `self.n()` == 1
    pub fn stdev(&self) -> f64 {
        sample_stdev(self.n(), self.sum, self.sum2)
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Default for SampleMoments {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            sum2: 0.0,
            min: f64::NAN,
            max: f64::NAN,
        }
    }
}

pub fn collect_moments(moments: &mut SampleMoments, value: f64) {
    moments.count += 1;
    moments.sum += value;
    moments.sum2 += value * value;
    moments.min = value.min(moments.min);
    moments.max = value.max(moments.max);
}

pub fn welch_t(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let s_dx = (s2_mean_a + s2_mean_b).sqrt();
    dx / s_dx
}

pub fn welch_deg_freedom(moments_a: &SampleMoments, moments_b: &SampleMoments) -> f64 {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let numerator = (s2_mean_a + s2_mean_b).powi(2);
    let denominator = s2_mean_a.powi(2) / (n_a - 1.0) + s2_mean_b.powi(2) / (n_b - 1.0);
    numerator / denominator
}

pub fn welch_ci(moments_a: &SampleMoments, moments_b: &SampleMoments, alpha: f64) -> (f64, f64) {
    let n_a = moments_a.n();
    let n_b = moments_b.n();
    let dx = moments_a.mean() - moments_b.mean();
    let s2_a = moments_a.stdev().powi(2);
    let s2_b = moments_b.stdev().powi(2);
    let s2_mean_a = s2_a / n_a;
    let s2_mean_b = s2_b / n_b;
    let nu = welch_deg_freedom(moments_a, moments_b);

    let stud = StudentsT::new(0.0, 1.0, nu)
        .expect("can't happen: degrees of freedom is always >= 3 by construction");
    let t = -stud.inverse_cdf(alpha / 2.0);

    let mid = dx;
    let radius = (s2_mean_a + s2_mean_b).sqrt() * t;

    (mid - radius, mid + radius)
}

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
pub fn sample_stdev(n: f64, sum: f64, sum2: f64) -> f64 {
    (sample_sum2_deviations(n, sum, sum2) / (n - 1.0)).sqrt()
}

pub struct Moments {
    count: u64,
    sum: f64,
    sum2: f64,
}

impl Moments {
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            sum2: 0.0,
        }
    }

    pub fn n(&self) -> f64 {
        self.count as f64
    }

    pub fn mean(&self) -> f64 {
        self.sum / self.n()
    }

    /// # Panics
    /// If `self.n` == 1
    pub fn var(&self) -> f64 {
        sample_sum2_deviations(self.n(), self.sum, self.sum2) / (self.n() - 1.0)
    }

    /// # Panics
    /// If `self.n` <= 1
    pub fn stdev(&self) -> f64 {
        self.var().sqrt()
    }
}

pub fn collect_moments(moments: &mut Moments, value: f64) {
    moments.count += 1;
    moments.sum += value;
    moments.sum2 += value * value;
}

pub fn welch_t(n1: u64, n2: u64, mean1: f64, mean2: f64, stdev1: f64, stdev2: f64) -> f64 {
    todo!()
}

pub fn welch_deg_freedom(n1: u64, n2: u64, stdev1: f64, stdev2: f64) -> f64 {
    todo!()
}

pub fn welch_ci(
    n1: u64,
    n2: u64,
    mean1: f64,
    mean2: f64,
    stdev1: f64,
    stdev2: f64,
    alpha: f64,
) -> (f64, f64) {
    todo!()
}

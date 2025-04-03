pub const ALPHA: f64 = 0.05;
pub const BETA: f64 = 0.05;
pub const BETA_01: f64 = 0.01;

pub const HI_1PCT_FACTOR: f64 = 1.01;
pub const HI_10PCT_FACTOR: f64 = 1.1;
pub const HI_25PCT_FACTOR: f64 = 1.25;

pub fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.
}

pub fn default_hi_stdev_log() -> f64 {
    2.4_f64.ln() / 2.
}

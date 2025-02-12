//! Module that provides functions to simulate work, used to support benchmarks.

use crate::latency;
use sha2::{Digest, Sha256};
use std::{hint::black_box, thread, time::Duration};

/// Function that sleeps to simulate work to support benchmarks.
pub fn lazy_work(micros: u64) {
    thread::sleep(Duration::from_micros(micros));
}

/// Function that does a significant amount of computation to support benchmarks.
pub fn real_work(iterations: u32) {
    let extent = black_box(iterations);
    let seed = black_box(0_u64);
    let buf = seed.to_be_bytes();
    let mut hasher = Sha256::new();
    for _ in 0..extent {
        hasher.update(buf);
    }
    let hash = hasher.finalize();
    black_box(hash);
}

/// Returns an estimate of the number of iterations required for [`real_work`] to have a latency
/// of `target_micros`. `calibration_iterations` is the number of iterations executed during calibration.
///
/// `calibration_iterations` should be greater than or equal to the returned value, ideally many times larger.
/// If it is not, the calibration is inaccurate and should be done again with a higher `calibration_iterations`
/// value.
pub fn calibrate_real_work(target_micros: u64, calibration_iterations: u32) -> u32 {
    let elapsed = latency(|| real_work(calibration_iterations));
    ((target_micros as f64 / elapsed as f64) * calibration_iterations as f64).ceil() as u32
}

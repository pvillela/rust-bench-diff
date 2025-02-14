//! Module that provides functions to simulate work, used to support creation of synthetic benchmarks.

use crate::{latency, LatencyUnit};
use sha2::{Digest, Sha256};
use std::{hint::black_box, thread, time::Duration};

/// Function that sleeps to simulate work to support benchmarks.
pub fn lazy_work(micros: u64) {
    thread::sleep(Duration::from_micros(micros));
}

/// Function that does a significant amount of computation to support benchmarks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn real_work(effort: u32) {
    let extent = black_box(effort);
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
/// of `target_micros`.
///
/// Calls [`calibrate_real_work_x`] with a predefined `calibration_effort`;
pub fn calibrate_real_work(unit: LatencyUnit, target_latency: u64) -> u32 {
    const CALIBRATION_EFFORT: u32 = 1000;
    calibrate_real_work_x(unit, target_latency, CALIBRATION_EFFORT)
}

/// Returns an estimate of the number of iterations required for [`real_work`] to have a latency
/// of `target_micros`. `calibration_effort` is the number of iterations executed during calibration.
pub fn calibrate_real_work_x(
    unit: LatencyUnit,
    target_latency: u64,
    calibration_effort: u32,
) -> u32 {
    let elapsed = latency(unit, || real_work(calibration_effort));
    ((target_latency as f64 / elapsed as f64) * calibration_effort as f64).ceil() as u32
}

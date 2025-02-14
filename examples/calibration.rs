use bench_diff::dev_utils::{calibrate_real_work, real_work};
use bench_diff::{latency, LatencyUnit};

fn main() {
    let unit = LatencyUnit::Nano;
    let target_effort = calibrate_real_work(unit, 1000);
    let latency_nanos = latency(unit, || real_work(target_effort));
    println!("target_effort={}", target_effort);
    println!("latency_nanos={}", latency_nanos);
}

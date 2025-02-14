use bench_diff::{
    dev_utils::{calibrate_real_work, real_work},
    BenchDiffOut, LatencyUnit,
};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Distribution, LogNormal};

pub fn print_diff_out(diff_out: BenchDiffOut) {
    println!("summary_f1={:?}", diff_out.summary_f1());
    println!("\nsummary_f2={:?}", diff_out.summary_f2());
    println!("\ncount_f1_lt_f2={}", diff_out.count_f1_lt_f2());
    println!("\ncount_f1_ge_f2={}", diff_out.count_f1_ge_f2());
    println!();
}

const BASE_MEDIAN: f64 = 400.0;
const HI_MEDIAN: f64 = 440.0;
const LO_LOG_STDEV: f64 = 0.5;
const HI_LOG_STDEV: f64 = 1.0;

fn synth(median_effort: u32, log_stdev: f64) -> impl FnMut() {
    let mu = 0.0_f64;
    let sigma = log_stdev;
    let lognormal = LogNormal::new(mu, sigma).expect("log_stdev must be > 0");
    let mut rng = StdRng::from_rng(&mut rand::rng());

    move || {
        let factor = lognormal.sample(&mut rng);
        let effort = (median_effort as f64) * factor;
        real_work(effort as u32);
    }
}

pub fn make_fn_tuple() -> (impl FnMut(), impl FnMut(), impl FnMut(), impl FnMut()) {
    let base_effort = calibrate_real_work(LatencyUnit::Nano, BASE_MEDIAN as u64);
    let hi_effort = (base_effort as f64 * HI_MEDIAN / BASE_MEDIAN) as u32;

    let base_median_lo_var = synth(base_effort, LO_LOG_STDEV);
    let base_median_hi_var = synth(base_effort, HI_LOG_STDEV);
    let hi_median_lo_var = synth(hi_effort, LO_LOG_STDEV);
    let hi_median_hi_var = synth(hi_effort, HI_LOG_STDEV);

    (
        base_median_lo_var,
        base_median_hi_var,
        hi_median_lo_var,
        hi_median_hi_var,
    )
}

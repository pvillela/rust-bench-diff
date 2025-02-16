use bench_diff::{
    bench_diff_print,
    dev_utils::{calibrate_real_work, real_work},
    BenchDiffOut, LatencyUnit,
};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::{Distribution, LogNormal};

pub struct Params {
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub hi_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

fn print_diff_out(diff_out: BenchDiffOut) {
    println!("summary_f1={:?}", diff_out.summary_f1());
    println!("\nsummary_f2={:?}", diff_out.summary_f2());
    println!("\ncount_f1_lt_f2={}", diff_out.count_f1_lt_f2());
    println!("\ncount_f1_ge_f2={}", diff_out.count_f1_ge_f2());
    println!();
}

fn synth(median_effort: u32, stdev_log: f64) -> impl FnMut() {
    let mu = 0.0_f64;
    let sigma = stdev_log;
    let lognormal = LogNormal::new(mu, sigma).expect("stdev_log must be > 0");
    let mut rng = StdRng::from_rng(&mut rand::rng());

    move || {
        let factor = lognormal.sample(&mut rng);
        let effort = (median_effort as f64) * factor;
        real_work(effort as u32);
    }
}

fn make_fn_tuple(params: &Params) -> (impl FnMut(), impl FnMut(), impl FnMut(), impl FnMut()) {
    let base_effort = calibrate_real_work(params.unit, params.base_median as u64);
    let hi_effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;

    let base_median_lo_var = synth(base_effort, params.lo_stdev_log);
    let base_median_hi_var = synth(base_effort, params.hi_stdev_log);
    let hi_median_lo_var = synth(hi_effort, params.lo_stdev_log);
    let hi_median_hi_var = synth(hi_effort, params.hi_stdev_log);

    (
        base_median_lo_var,
        base_median_hi_var,
        hi_median_lo_var,
        hi_median_hi_var,
    )
}

pub fn bench(params: Params) {
    let (
        mut base_median_lo_var,
        mut base_median_hi_var,
        mut hi_median_lo_var,
        mut hi_median_hi_var,
    ) = make_fn_tuple(&params);

    bench_diff_print(
        params.unit,
        &mut base_median_lo_var,
        &mut base_median_hi_var,
        params.exec_count,
        || println!("f1=base_median_lo_var, f2=base_median_hi_var"),
        print_diff_out,
    );

    bench_diff_print(
        params.unit,
        &mut base_median_lo_var,
        &mut hi_median_lo_var,
        params.exec_count,
        || println!("f1=base_median_lo_var, f2=hi_median_lo_var"),
        print_diff_out,
    );

    bench_diff_print(
        params.unit,
        &mut base_median_lo_var,
        &mut hi_median_hi_var,
        params.exec_count,
        || println!("f1=base_median_lo_var, f2=hi_median_hi_var"),
        print_diff_out,
    );
}

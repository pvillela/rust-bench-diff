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

fn print_diff_out(diff_out: &BenchDiffOut) {
    const ALPHA: f64 = 0.1;

    let ratio_median_f1_f2 =
        diff_out.summary_f1().median as f64 / diff_out.summary_f2().median as f64;
    let ratio_ci = diff_out.welch_ratio_ci(ALPHA);
    let position_in_ci_ratio_1 = diff_out.welch_position_in_ci_ratio_1(ALPHA);
    let mean_diff_f1_f2 = diff_out.mean_diff_f1_f2();
    let diff_ci = diff_out.student_diff_ci(ALPHA);
    let position_in_ci_diff_0 = diff_out.student_position_in_ci_diff_0(ALPHA);

    let wilcoxon_rank_sum_z = diff_out.wilcoxon_rank_sum_z();
    let wilcoxon_rank_sum_p = diff_out.wilcoxon_rank_sum_p();

    println!("summary_f1={:?}", diff_out.summary_f1());
    println!("\nsummary_f2={:?}", diff_out.summary_f2());
    println!("\ncount_f1_lt_f2={}", diff_out.count_f1_lt_f2());
    println!("count_f1_eq_f2={}", diff_out.count_f1_eq_f2());
    println!("count_f1_gt_f2={}", diff_out.count_f1_gt_f2());
    println!("ratio_median_f1_f2={}", ratio_median_f1_f2);
    println!("ratio_ci={:?}", ratio_ci);
    println!("position_in_ci_ratio_1={:?}", position_in_ci_ratio_1);
    println!("mean_diff_f1_f2={}", mean_diff_f1_f2);
    println!("diff_ci={:?}", diff_ci);
    println!("position_in_ci_diff_0={:?}", position_in_ci_diff_0);
    println!("wilcoxon_rank_sum_z={:?}", wilcoxon_rank_sum_z);
    println!("wilcoxon_rank_sum_p={:?}", wilcoxon_rank_sum_p);
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

fn make_fn_tuple(
    base_effort: u32,
    params: &Params,
) -> (
    impl FnMut(),
    impl FnMut(),
    impl FnMut(),
    impl FnMut(),
    impl FnMut(),
) {
    let hi_effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;

    let base_median_lo_var = synth(base_effort, params.lo_stdev_log);
    let base_median_lo_var1 = synth(base_effort, params.lo_stdev_log);
    let base_median_hi_var = synth(base_effort, params.hi_stdev_log);
    // let base_median_hi_var = synth(base_effort * 3 / 4, params.lo_stdev_log * 0.5);
    let hi_median_lo_var = synth(hi_effort, params.lo_stdev_log);
    let hi_median_hi_var = synth(hi_effort, params.hi_stdev_log);

    (
        base_median_lo_var,
        base_median_lo_var1,
        base_median_hi_var,
        hi_median_lo_var,
        hi_median_hi_var,
    )
}

pub fn bench(params: Params) {
    let base_effort = calibrate_real_work(params.unit, params.base_median as u64);

    let (
        mut base_median_lo_var,
        mut base_median_lo_var1,
        mut base_median_hi_var,
        mut hi_median_lo_var,
        mut hi_median_hi_var,
    ) = make_fn_tuple(base_effort, &params);

    let base_median_no_var = || {
        real_work(base_effort);
    };

    {
        let diff_out = bench_diff_print(
            params.unit,
            &base_median_no_var,
            &base_median_no_var,
            params.exec_count,
            || println!("f1=base_median_no_var, f2=base_median_no_var"),
            print_diff_out,
        );

        let ratio_median_f1_f2 =
            diff_out.summary_f1().median as f64 / diff_out.summary_f2().median as f64;
        let ratio_ci = diff_out.welch_ratio_ci(0.1);
        assert!(
            ratio_median_f1_f2 > ratio_ci.0 && ratio_median_f1_f2 < ratio_ci.1,
            "median in ci"
        );
    }

    // bench_diff_print(
    //     params.unit,
    //     &mut base_median_lo_var,
    //     &mut base_median_lo_var1,
    //     params.exec_count,
    //     || println!("f1=base_median_lo_var, f2=base_median_lo_var1"),
    //     print_diff_out,
    // );

    // bench_diff_print(
    //     params.unit,
    //     &mut base_median_lo_var,
    //     &mut base_median_hi_var,
    //     params.exec_count,
    //     || println!("f1=base_median_lo_var, f2=base_median_hi_var"),
    //     print_diff_out,
    // );

    // bench_diff_print(
    //     params.unit,
    //     &mut base_median_hi_var,
    //     &mut base_median_lo_var,
    //     params.exec_count,
    //     || println!("f1=base_median_hi_var, f2=base_median_lo_var"),
    //     print_diff_out,
    // );

    bench_diff_print(
        params.unit,
        &mut base_median_lo_var,
        &mut hi_median_lo_var,
        params.exec_count,
        || println!("f1=base_median_lo_var, f2=hi_median_lo_var"),
        print_diff_out,
    );

    // bench_diff_print(
    //     params.unit,
    //     &mut base_median_lo_var,
    //     &mut hi_median_hi_var,
    //     params.exec_count,
    //     || println!("f1=base_median_lo_var, f2=hi_median_hi_var"),
    //     print_diff_out,
    // );
}

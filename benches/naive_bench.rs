use bench_diff::{
    LatencyUnit,
    bench_support::{bench_naive::bench_naive, calibrated_fn_params, get_fn},
    test_support::{ScaleParams, default_hi_stdev_log, default_lo_stdev_log},
};

fn main() {
    let base_median = 100_000.0;
    let scale_params = ScaleParams {
        name: "micros_scale".into(),
        unit: LatencyUnit::Nano,
        exec_count: 2_000,
        base_median,
        lo_stdev_log: default_lo_stdev_log(),
        hi_stdev_log: default_hi_stdev_log(),
    };

    let calibrated_fn_params = calibrated_fn_params(&scale_params);

    {
        let name = "hi_1pct_median_no_var";
        let f = {
            let mut my_fn = get_fn(name)(&calibrated_fn_params);
            move || my_fn.invoke()
        };

        let out = bench_naive(LatencyUnit::Nano, f, scale_params.exec_count);
        println!("\n{}: {:?}", name, out.summary_f1());
        println!();
    }

    {
        let name = "base_median_no_var";
        let f = {
            let mut my_fn = get_fn(name)(&calibrated_fn_params);
            move || my_fn.invoke()
        };

        let out = bench_naive(LatencyUnit::Nano, f, scale_params.exec_count);
        println!("\n{}: {:?}", name, out.summary_f1());
        println!();
    }
}

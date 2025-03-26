use std::time::Duration;

use bench_diff::{
    DiffOut, LatencyUnit, bench_diff_print,
    dev_utils::{busy_work, calibrate_busy_work},
    statistics::AltHyp,
};

fn f1(effort: u32) {
    busy_work(effort);
}

fn f2(effort: u32) {
    let hi_effort = ((effort as f64) * 1.05) as u32;
    busy_work(hi_effort);
}

fn main() {
    let effort = calibrate_busy_work(Duration::from_micros(100));

    println!("*** 1st benchmark ***");
    {
        let out: DiffOut = bench_diff_print(
            LatencyUnit::Nano,
            || f1(effort),
            || f2(effort),
            1000,
            || println!("Comparing latency of f1 vs. f2."),
            |_| (),
        );
        print_diff_out(&out);
    }

    println!("*** 2nd benchmark ***");
    {
        let out: DiffOut = bench_diff_print(
            LatencyUnit::Nano,
            || f1(effort),
            || f1(effort),
            1000,
            || println!("Comparing latency of f1 vs. f1."),
            |_| (),
        );
        print_diff_out(&out);
    }
}

const ALPHA: f64 = 0.05;

fn print_diff_out(out: &DiffOut) {
    let ratio_medians_f1_f2 = out.ratio_medians_f1_f2();
    let ratio_medians_f1_f2_from_lns = out.mean_diff_ln_f1_f2().exp();

    println!();
    println!("summary_f1={:?}", out.summary_f1());
    println!();
    println!("summary_f2={:?}", out.summary_f2());
    println!();
    println!(
        "ratio_medians_f1_f2={}, ratio_medians_f1_f2_from_lns={}, diff={}",
        ratio_medians_f1_f2,
        ratio_medians_f1_f2_from_lns,
        ratio_medians_f1_f2 - ratio_medians_f1_f2_from_lns
    );
    println!();
    println!("welch_ratio_ci={:?}", out.welch_ratio_ci(ALPHA),);
    println!(
        "welch_ln_test_lt:{:?}",
        out.welch_ln_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "welch_ln_test_eq:{:?}",
        out.welch_ln_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "welch_ln_test_gt:{:?}",
        out.welch_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("student_ratio_ci={:?}", out.student_ratio_ci(ALPHA),);
    println!(
        "student_diff_ln_test_lt:{:?}",
        out.student_diff_ln_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "student_diff_ln_test_eq:{:?}",
        out.student_diff_ln_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "student_diff_ln_test_gt:{:?}",
        out.student_diff_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("mean_diff_f1_f2={}", out.mean_diff_f1_f2());
    println!(
        "relative_mean_diff_f1_f2={}",
        out.mean_diff_f1_f2() / (out.mean_f1() + out.mean_f2()) * 2.0
    );
    println!("diff_medians_f1_f2={}", out.diff_medians_f1_f2());
    println!(
        "relative_diff_medians_f1_f2={}",
        out.diff_medians_f1_f2() / (out.median_f1() + out.median_f2()) * 2.0
    );
    println!();
    println!(
        "count_f1_lt_f2={}, count_f1_eq_f2={}, count_f1_gt_f2={}",
        out.count_f1_lt_f2(),
        out.count_f1_eq_f2(),
        out.count_f1_gt_f2()
    );
    println!(
        "wilcoxon_rank_sum_test_lt:{:?}",
        out.wilcoxon_rank_sum_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "wilcoxon_rank_sum_test_eq:{:?}",
        out.wilcoxon_rank_sum_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "wilcoxon_rank_sum_test_gt:{:?}",
        out.wilcoxon_rank_sum_test(AltHyp::Gt, ALPHA)
    );
    println!();
}

//! Implementaton of main benchmarking logic to verify [`bench_diff`].

use super::params_args::{Args, calibrated_fn_params, get_args, get_fn};
use crate::{
    DiffOut, bench_diff, bench_diff_print,
    dev_utils::nest_btree_map,
    statistics::{AltHyp, SampleMoments, collect_moments},
    test_support::{ALPHA, BETA, Claim, ClaimResults, ScaleParams, get_scale_params, get_scenario},
};
use std::{collections::BTreeMap, fmt::Debug, ops::Deref};

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
    println!(
        "student_diff_test_lt:{:?}",
        out.student_diff_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "student_diff_test_eq:{:?}",
        out.student_diff_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "student_diff_test_gt:{:?}",
        out.student_diff_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!(
        "count_f1_lt_f2={}, count_f1_eq_f2={}, count_f1_gt_f2={}",
        out.count_f1_lt_f2(),
        out.count_f1_eq_f2(),
        out.count_f1_gt_f2()
    );
    println!(
        "bernoulli_prob_f1_gt_f2={:?}",
        out.bernoulli_prob_f1_gt_f2()
    );
    println!("bernoulli_ci={:?}", out.bernoulli_ci(ALPHA),);
    println!(
        "bernoulli_eq_half_test_lt:{:?}",
        out.bernoulli_eq_half_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "bernoulli_eq_half_test_eq:{:?}",
        out.bernoulli_eq_half_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "bernoulli_eq_half_test_gt:{:?}",
        out.bernoulli_eq_half_test(AltHyp::Gt, ALPHA)
    );
    println!();
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

/// Runs benchmarks with statistical t-tests for target functions and comparison scenarios defined by
/// environment variables and command line arguments.
/// Defaults are provided for environment variables and command line arguments not defined.
pub fn bench_with_claims_and_args() {
    let Args {
        scale_name,
        fn_name_pairs,
        verbose,
        noise_stats,
        nrepeats,
        run_name,
    } = get_args();
    let scale_params = get_scale_params(&scale_name);

    bench_with_claims(
        scale_params,
        &fn_name_pairs,
        verbose,
        noise_stats,
        nrepeats,
        &run_name,
    );
}

/// Runs benchmarks with statistical tests and other claims for target functions parameterized by `fn_params`,
/// with comparison scenarios defined by `fn_name_pairs`, repeating the benchmarks `nrepeats` times and collecting summary
/// results for the claims.
///
///  `verbose` determines the verbosity of output, `print_args` is a closure that prints the
/// configuration arguments for the benchmarks and `run_name` is a string that designates the run in the print-out.
pub fn bench_with_claims<T: Deref<Target = str> + Debug>(
    scale_params: &ScaleParams,
    fn_name_pairs: &[(T, T)],
    verbose: bool,
    noise_stats: bool,
    nrepeats: usize,
    run_name: &str,
) {
    let print_args = || {
        println!("*** arguments ***");
        println!("SCALE_NAME=\"{}\"", scale_params.name);
        println!(
            "unit={:?}, exec_count={}, base_median={}",
            scale_params.unit, scale_params.exec_count, scale_params.base_median
        );
        println!("FN_NAME_PAIRS=\"{fn_name_pairs:?}\"");
        println!("VERBOSE=\"{verbose}\"");
        println!("NOISE_STATS=\"{noise_stats}\"");
        println!("nrepeats={nrepeats}");
        println!("run_name=\"{run_name}\"");
    };

    let calibrated_fn_params = calibrated_fn_params(scale_params);

    println!();
    print_args();
    println!();

    let total_iterations = nrepeats * fn_name_pairs.len();
    let mut cumulative_iter = 0;

    for (name1, name2) in fn_name_pairs {
        let scenario_name = format!("f1={}, f2={}", name1.deref(), name2.deref());
        let scenario = get_scenario(name1, name2);

        let mut f1 = {
            let mut my_fn = get_fn(name1)(&calibrated_fn_params);
            move || my_fn.invoke()
        };

        let mut f2 = {
            let mut my_fn = get_fn(name2)(&calibrated_fn_params);
            move || my_fn.invoke()
        };

        let mut results = ClaimResults::new();

        let mut ratio_medians_from_lns_noises =
            BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
        let mut diff_ratio_medians_noises =
            BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
        let mut diff_ln_stdev_noises =
            BTreeMap::<(&'static str, &'static str), SampleMoments>::new();

        for i in 1..=nrepeats {
            cumulative_iter += 1;
            eprintln!(
                "*** run_name=\"{run_name}\", scenario=\"{scenario_name}\", scenario_iteration={i}, ({cumulative_iter} of {total_iterations}) ***"
            );

            let diff_out = if verbose {
                let out = bench_diff_print(
                    scale_params.unit,
                    &mut f1,
                    &mut f2,
                    scale_params.exec_count,
                    || println!("{scenario_name}"),
                );
                print_diff_out(&out);
                out
            } else {
                bench_diff(scale_params.unit, &mut f1, &mut f2, scale_params.exec_count)
            };

            scenario.check_claims(&mut results, &diff_out, verbose);

            if noise_stats {
                let ratio_medians_from_lns_noise = ratio_medians_from_lns_noises
                    .entry((name1, name2))
                    .or_insert_with(|| SampleMoments::default());
                collect_moments(
                    ratio_medians_from_lns_noise,
                    diff_out.mean_diff_ln_f1_f2().exp(),
                );

                let diff_ratio_medians_noise = diff_ratio_medians_noises
                    .entry((name1, name2))
                    .or_insert_with(|| SampleMoments::default());
                collect_moments(
                    diff_ratio_medians_noise,
                    diff_out.ratio_medians_f1_f2() - diff_out.ratio_medians_f1_f2_from_lns(),
                );

                let diff_ln_stdev_noise = diff_ln_stdev_noises
                    .entry((name1, name2))
                    .or_insert_with(|| SampleMoments::default());
                collect_moments(diff_ln_stdev_noise, diff_out.stdev_diff_ln_f1_f2());
            }
        }

        if verbose {
            println!("*** failures ***");
            for claim_result in results.failures().iter() {
                println!("{claim_result:?}");
            }

            println!();
            print_args();

            println!();
            println!("*** failure_summary ***");
            for ((name_pair, claim_name), count) in results.failure_summary() {
                println!("{name_pair:?} | {claim_name} ==> count={count}");
            }

            println!();
            println!("*** success_summary ***");
            for (name_pair, claim_name) in results.success_summary() {
                println!("{name_pair:?} | {claim_name}");
            }
        } else {
            println!("*** claim_summary ***");
            for ((name_pair, claim_name), count) in results.summary() {
                println!("{name_pair:?} | {claim_name} ==> count={count}");
            }
        }

        let type_i_and_ii_errors =
            results.type_i_and_ii_errors(ALPHA, BETA, &Claim::CRITICAL_NAMES, nrepeats);
        if !type_i_and_ii_errors.is_empty() {
            println!(
                ">>> type_i_and_ii_errors: {:?}",
                nest_btree_map(type_i_and_ii_errors)
            );
        }

        if noise_stats {
            println!();
            println!("*** noise statistics ***");
            for (name1, name2) in fn_name_pairs {
                println!();
                println!("scenario: fn1={}, fn2={}", name1.deref(), name2.deref());
                println!(
                    "ratio_medians_from_lns_noise_mean={}, ratio_medians_from_lns_noise_stdev={}",
                    ratio_medians_from_lns_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .mean(),
                    ratio_medians_from_lns_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .stdev()
                );
                println!(
                    "diff_ratio_medians_noise_mean={}, diff_ratio_medians_noise_stdev={}, diff_ratio_medians_noise_min={}, diff_ratio_medians_noise_max={}",
                    diff_ratio_medians_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .mean(),
                    diff_ratio_medians_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .stdev(),
                    diff_ratio_medians_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .min(),
                    diff_ratio_medians_noises
                        .get(&(name1, name2))
                        .unwrap()
                        .max()
                );
                println!(
                    "diff_ln_stdev_noise_mean={}, diff_ln_stdev_noise_stdev={}",
                    diff_ln_stdev_noises.get(&(name1, name2)).unwrap().mean(),
                    diff_ln_stdev_noises.get(&(name1, name2)).unwrap().stdev()
                );
            }
        }
    }
}

//! Implementaton of main logic used by benchmark tests to verify [`bench_diff`].

use super::params_args::{BenchArgs, get_args};
use crate::{
    DiffOut, bench_diff, bench_diff_with_status,
    bench_support::params_args::BenchMode,
    dev_utils::nest_btree_map,
    get_bench_cfg,
    stats_types::AltHyp,
    test_support::{
        ALPHA, BETA, BETA_01, ClaimResults, MyFnMut, ScaleParams, binomial_inv_cdf,
        binomial_nsigmas_gt_critical_value, get_scale_params,
    },
};
use bench_utils::{Comp, bench_run, bench_run_with_status, calibrate_busy_work};

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
        out.welch_ln_test(0., AltHyp::Lt, ALPHA)
    );
    println!(
        "welch_ln_test_eq:{:?}",
        out.welch_ln_test(0., AltHyp::Ne, ALPHA)
    );
    println!(
        "welch_ln_test_gt:{:?}",
        out.welch_ln_test(0., AltHyp::Gt, ALPHA)
    );
    println!();
    println!("mean_diff_f1_f2={}", out.mean_diff_f1_f2());
    println!(
        "relative_mean_diff_f1_f2={}",
        out.mean_diff_f1_f2() / (out.mean_f1() + out.mean_f2()) * 2.
    );
    println!("diff_medians_f1_f2={}", out.diff_medians_f1_f2());
    println!(
        "relative_diff_medians_f1_f2={}",
        out.diff_medians_f1_f2() / (out.median_f1() + out.median_f2()) * 2.
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
    println!("binomial_prob_f1_gt_f2={:?}", out.prop_f1_gt_f2());
    println!("binomial_ci={:?}", out.binomial_f1_gt_f2_ws_ci(ALPHA),);
    println!(
        "binomial_eq_half_test_lt:{:?}",
        out.exact_binomial_f1_gt_f2_eq_half_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "binomial_eq_half_test_eq:{:?}",
        out.exact_binomial_f1_gt_f2_eq_half_test(AltHyp::Ne, ALPHA)
    );
    println!(
        "binomial_eq_half_test_gt:{:?}",
        out.exact_binomial_f1_gt_f2_eq_half_test(AltHyp::Gt, ALPHA)
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

fn print_comp_out(comp: &Comp) {
    let ratio_medians_f1_f2 = comp.ratio_medians_f1_f2();
    let ratio_medians_f1_f2_from_lns = comp.mean_diff_ln_f1_f2().exp();

    println!();
    println!("summary_f1={:?}", comp.out_f1().summary());
    println!();
    println!("summary_f2={:?}", comp.out_f2().summary());
    println!();
    println!(
        "ratio_medians_f1_f2={}, ratio_medians_f1_f2_from_lns={}, diff={}",
        ratio_medians_f1_f2,
        ratio_medians_f1_f2_from_lns,
        ratio_medians_f1_f2 - ratio_medians_f1_f2_from_lns
    );
    println!();
    println!("welch_ratio_ci={:?}", comp.welch_ratio_ci(ALPHA),);
    println!(
        "welch_ln_test_lt:{:?}",
        comp.welch_ln_test(0., AltHyp::Lt, ALPHA)
    );
    println!(
        "welch_ln_test_eq:{:?}",
        comp.welch_ln_test(0., AltHyp::Ne, ALPHA)
    );
    println!(
        "welch_ln_test_gt:{:?}",
        comp.welch_ln_test(0., AltHyp::Gt, ALPHA)
    );
    println!();
    println!("mean_diff_f1_f2={}", comp.mean_diff_f1_f2());
    println!(
        "relative_mean_diff_f1_f2={}",
        comp.mean_diff_f1_f2() / (comp.out_f1().mean() + comp.out_f2().mean()) * 2.
    );
    println!("diff_medians_f1_f2={}", comp.diff_medians_f1_f2());
    println!(
        "relative_diff_medians_f1_f2={}",
        comp.diff_medians_f1_f2() / (comp.out_f1().median() + comp.out_f2().median()) * 2.
    );
    println!();
}

/// Runs benchmarks with statistical t-tests for target functions and comparison scenarios defined by
/// environment variables and command line arguments.
/// Defaults are provided for environment variables and command line arguments not defined.
pub fn bench_with_claims_and_args() {
    bench_with_claims(get_args());
}

/// Runs benchmarks with statistical tests and other claims for target functions parameterized by `fn_params`,
/// with comparison scenarios defined by `fn_name_pairs`, repeating the benchmarks `nrepeats` times and collecting summary
/// results for the claims.
///
///  `verbose` determines the verbosity of output, `print_args` is a closure that prints the
/// configuration arguments for the benchmarks and `run_name` is a string that designates the run in the print-out.
pub fn bench_with_claims(args: BenchArgs) {
    let BenchArgs {
        ref scale_name,
        bench_mode,
        ref fn_spec_pairs,
        verbose,
        nrepeats,
        ref run_name,
    } = args;

    let scale_params = get_scale_params(&scale_name);

    let ScaleParams {
        name: _,
        recording_unit,
        reporting_unit,
        exec_count,
        base_latency,
    } = scale_params;

    get_bench_cfg()
        .with_recording_unit(*recording_unit)
        .with_reporting_unit(*reporting_unit)
        .set();

    let print_args = || {
        println!("*** args = {args:?}");
        println!("*** scale_params = {scale_params:?}");
        println!("*** other parameters ***");
        let tau = 0.95;
        let nsigmas = 2.;
        println!(
            "ALPHA={ALPHA}, exact_type_i_gt_critical_value({tau})={}, nsigmas_type_i_gt_critical_value({nsigmas})={}",
            binomial_inv_cdf(nrepeats as u64, ALPHA, tau).unwrap(),
            binomial_nsigmas_gt_critical_value(nrepeats as u64, ALPHA, nsigmas)
        );
        {
            // `binomial_inv_cdf` errors for `nrepeats <= 7` and `tau` = 0.67.
            let tau = 0.67;
            let nsigmas = 1.;
            print!(
                "ALPHA={ALPHA}, tau={tau}, binomial_inv_cdf={:?}, ",
                binomial_inv_cdf(nrepeats as u64, ALPHA, tau),
            );
            println!(
                "exact_type_i_gt_critical_value({tau})={}, nsigmas_type_i_gt_critical_value({nsigmas})={}",
                binomial_inv_cdf(nrepeats as u64, ALPHA, tau).unwrap(),
                binomial_nsigmas_gt_critical_value(nrepeats as u64, ALPHA, nsigmas)
            );
        }
        println!(
            "BETA={BETA}, exact_type_ii_gt_critical_value({tau})={}, nsigmas_type_ii_gt_critical_value({nsigmas})={}",
            binomial_inv_cdf(nrepeats as u64, BETA, tau).unwrap(),
            binomial_nsigmas_gt_critical_value(nrepeats as u64, BETA, nsigmas)
        );
    };

    let base_effort = calibrate_busy_work(*base_latency);

    println!();
    print_args();
    println!();

    let total_iterations = nrepeats * fn_spec_pairs.len();
    let mut cumulative_iter = 0;

    for (spec_f1, spec_f2) in fn_spec_pairs {
        let scenario_name = format!("f1={}, f2={}", spec_f1, spec_f2);

        let mut f1 = {
            let mut my_fn = MyFnMut::new(base_effort, *spec_f1);
            move || my_fn.invoke()
        };

        let mut f2 = {
            let mut my_fn = MyFnMut::new(base_effort, *spec_f2);
            move || my_fn.invoke()
        };

        let mut results = ClaimResults::new();

        for i in 1..=nrepeats {
            cumulative_iter += 1;
            eprintln!(
                "*** run_name=\"{run_name}\", scenario=\"{scenario_name}\", scenario_iteration={i}, ({cumulative_iter} of {total_iterations}) ***"
            );

            match bench_mode {
                BenchMode::Diff => {
                    let diff_out = if verbose {
                        let out = bench_diff_with_status(
                            &mut f1,
                            &mut f2,
                            *exec_count,
                            |exec_count| {
                                println!("{scenario_name}");
                                println!(
                                    "\n>>> bench_diff for ({spec_f1}, {spec_f2}): exec_count={exec_count}",
                                );
                                println!();
                            },
                        );
                        print_diff_out(&out);
                        out
                    } else {
                        bench_diff(&mut f1, &mut f2, scale_params.exec_count)
                    };

                    results.check_claims_diff(*spec_f1, *spec_f2, ALPHA, &diff_out, verbose);
                }

                BenchMode::Comp => {
                    let (out1, out2) = if verbose {
                        let out1 = bench_run_with_status(&mut f1, *exec_count, |exec_count| {
                            println!("{scenario_name}");
                            println!("\n>>> bench_run for {spec_f1}: exec_count={exec_count}",);
                            println!();
                        });
                        let out2 = bench_run_with_status(&mut f2, *exec_count, |exec_count| {
                            println!("{scenario_name}");
                            println!("\n>>> bench_run for {spec_f2}: exec_count={exec_count}",);
                            println!();
                        });
                        (out1, out2)
                    } else {
                        let out1 = bench_run(&mut f1, scale_params.exec_count);
                        let out2 = bench_run(&mut f2, scale_params.exec_count);
                        (out1, out2)
                    };

                    let comp = Comp::new(&out1, &out2);
                    print_comp_out(&comp);
                    results.check_claims_comp(*spec_f1, *spec_f2, ALPHA, &comp, verbose);
                }
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

        let type_i_and_ii_errors_67 = results.excess_type_i_and_ii_errors(
            ALPHA,
            BETA,
            &ClaimResults::CRITICAL_CLAIM_NAMES,
            nrepeats,
            0.67,
        );
        if !type_i_and_ii_errors_67.is_empty() {
            println!(
                ">>> type_i_and_ii_errors_67: {:?}",
                nest_btree_map(type_i_and_ii_errors_67)
            );
        }

        let type_i_and_ii_errors_95 = results.excess_type_i_and_ii_errors(
            ALPHA,
            BETA,
            &ClaimResults::CRITICAL_CLAIM_NAMES,
            nrepeats,
            0.95,
        );
        if !type_i_and_ii_errors_95.is_empty() {
            println!(
                ">>> type_i_and_ii_errors_95: {:?}",
                nest_btree_map(type_i_and_ii_errors_95)
            );
        }

        let type_i_and_ii_errors_95_beta_01 = results.excess_type_i_and_ii_errors(
            ALPHA,
            BETA_01,
            &ClaimResults::CRITICAL_CLAIM_NAMES,
            nrepeats,
            0.95,
        );
        if !type_i_and_ii_errors_95_beta_01.is_empty() {
            println!(
                ">>> type_i_and_ii_errors_95_beta_01: {:?}",
                nest_btree_map(type_i_and_ii_errors_95_beta_01)
            );
        }
    }
}

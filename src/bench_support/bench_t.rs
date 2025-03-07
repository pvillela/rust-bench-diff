//!

use super::params_args::{Args, FnParams, get_args, get_fn, get_params, get_spec};
use crate::{
    BenchDiffOut, PositionInCi, SampleMoments, bench_diff, bench_diff_print, collect_moments,
    dev_utils::calibrate_real_work,
};
use std::{collections::BTreeMap, fmt::Debug, ops::Deref};

#[derive(Debug)]
struct TestResult {
    scenario: String,
    test: &'static str,
    passed: bool,
    #[allow(dead_code)]
    result_string: String,
}

impl TestResult {
    #[allow(dead_code)]
    fn check(scenario: &str, test: &'static str, condition: bool, result_string: String) -> Self {
        Self {
            scenario: scenario.to_owned(),
            test,
            passed: condition,
            result_string,
        }
    }

    fn check_eq<T: PartialEq + Debug>(
        scenario: &str,
        test: &'static str,
        expected: T,
        actual: T,
    ) -> Self {
        Self {
            scenario: scenario.to_owned(),
            test,
            passed: expected == actual,
            result_string: format!("expected={expected:?}, actual={actual:?}"),
        }
    }
}

struct TestFailures(Vec<(TestResult, bool)>);

impl TestFailures {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push_failure(&mut self, result: TestResult, must_pass: bool) {
        if !result.passed {
            self.0.push((result, must_pass))
        };
    }

    fn failures(&self) -> Vec<&TestResult> {
        self.0.iter().map(|p| &p.0).collect()
    }

    fn failed_must_pass(&self) -> Vec<&TestResult> {
        self.0
            .iter()
            .filter(|p| !p.0.passed && p.1)
            .map(|x| &x.0)
            .collect()
    }
}

const ALPHA: f64 = 0.05;

fn all_tests<'a>(
    diff_out: &BenchDiffOut,
    test_failures: &mut TestFailures,
    scenario: String,
    expected: PositionInCi,
    must_pass1: bool,
    must_pass2: bool,
    must_pass3: bool,
) {
    test_failures.push_failure(
        TestResult::check_eq(
            &scenario,
            "welch_position_of_1_in_ratio_ci",
            expected,
            diff_out.welch_position_of_1_in_ratio_ci(ALPHA),
        ),
        must_pass1,
    );

    test_failures.push_failure(
        TestResult::check_eq(
            &scenario,
            "student_position_of_0_in_diff_ci",
            expected,
            diff_out.student_position_of_0_in_diff_ci(ALPHA),
        ),
        must_pass2,
    );

    test_failures.push_failure(
        TestResult::check_eq(
            &scenario,
            "student_position_of_1_in_ratio_ci",
            expected,
            diff_out.student_position_of_1_in_ratio_ci(ALPHA),
        ),
        must_pass3,
    );
}

fn print_diff_out(diff_out: &BenchDiffOut) {
    let ratio_median_f1_f2 =
        diff_out.summary_f1().median as f64 / diff_out.summary_f2().median as f64;
    let welch_ratio_ci = diff_out.welch_ratio_ci(ALPHA);
    let welch_position_of_1_in_ratio_ci = diff_out.welch_position_of_1_in_ratio_ci(ALPHA);
    let mean_diff_f1_f2 = diff_out.mean_diff_f1_f2();
    let student_diff_ci = diff_out.student_diff_ci(ALPHA);
    let student_position_of_0_in_diff_ci = diff_out.student_position_of_0_in_diff_ci(ALPHA);
    let student_position_of_1_in_ratio_ci = diff_out.student_position_of_1_in_ratio_ci(ALPHA);

    let wilcoxon_rank_sum_f1_lt_f2_p = diff_out.wilcoxon_rank_sum_f1_lt_f2_p();
    let wilcoxon_rank_sum_f1_gt_f2_p = diff_out.wilcoxon_rank_sum_f1_gt_f2_p();

    println!("summary_f1={:?}", diff_out.summary_f1());
    println!("\nsummary_f2={:?}", diff_out.summary_f2());
    println!("\ncount_f1_lt_f2={}", diff_out.count_f1_lt_f2());
    println!("count_f1_eq_f2={}", diff_out.count_f1_eq_f2());
    println!("count_f1_gt_f2={}", diff_out.count_f1_gt_f2());
    println!("ratio_median_f1_f2={}", ratio_median_f1_f2);
    println!("welch_ratio_ci={:?}", welch_ratio_ci);
    println!(
        "welch_position_of_1_in_ratio_ci={:?}",
        welch_position_of_1_in_ratio_ci
    );
    println!("mean_diff_f1_f2={}", mean_diff_f1_f2);
    println!("diff_ci={:?}", student_diff_ci);
    println!(
        "student_position_of_0_in_diff_ci={:?}",
        student_position_of_0_in_diff_ci
    );
    println!(
        "student_position_of_1_in_ratio_ci={:?}",
        student_position_of_1_in_ratio_ci
    );

    println!(
        "wilcoxon_rank_sum_f1_lt_f2_p={:?}",
        wilcoxon_rank_sum_f1_lt_f2_p
    );
    println!(
        "wilcoxon_rank_sum_f1_gt_f2_p={:?}",
        wilcoxon_rank_sum_f1_gt_f2_p
    );
    println!();
}

/// Runs benchmarks with statistical t-tests for target functions and comparison scenarios defined by
/// environment variables and command line arguments.
/// Defaults are provided for environment variables and command line arguments not defined.
pub fn bench_diff_t_with_args() {
    let Args {
        params_name,
        fn_name_pairs,
        verbose,
        nrepeats,
    } = get_args();
    let fn_params = get_params(&params_name);
    bench_diff_t(fn_params, &fn_name_pairs, nrepeats, verbose);
}

/// Runs benchmarks with statistical t-tests for target functions parameterized by `fn_params`,
/// with comparison scenarios defined by `fn_name_pairs`.
pub fn bench_diff_t<T: Deref<Target = str>>(
    fn_params: &FnParams,
    fn_name_pairs: &[(T, T)],
    nrepeats: usize,
    verbose: bool,
) {
    let unit = fn_params.unit;
    let base_effort = calibrate_real_work(unit.latency_from_f64(fn_params.base_median));

    let mut test_failures = TestFailures::new();
    let mut ratio_medians_noises = BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
    let mut diff_ln_stdev_noises = BTreeMap::<(&'static str, &'static str), SampleMoments>::new();

    for i in 1..=nrepeats {
        println!("*** iteration = {i} ***");

        for (name1, name2) in fn_name_pairs {
            let scenario = format!("f1={}, f2={}", name1.deref(), name2.deref());

            let f1 = {
                let mut my_fn = get_fn(name1)(base_effort, &fn_params);
                move || my_fn.invoke()
            };

            let f2 = {
                let mut my_fn = get_fn(name2)(base_effort, &fn_params);
                move || my_fn.invoke()
            };

            let diff_out = if verbose {
                bench_diff_print(
                    fn_params.unit,
                    f1,
                    f2,
                    fn_params.exec_count,
                    || println!("{scenario}"),
                    print_diff_out,
                )
            } else {
                bench_diff(fn_params.unit, f1, f2, fn_params.exec_count)
            };

            let ratio_medians_noise = ratio_medians_noises
                .entry((name1, name2))
                .or_insert_with(|| SampleMoments::default());
            collect_moments(ratio_medians_noise, diff_out.mean_diff_ln_f1_f2().exp());

            let diff_ln_stdev_noise = diff_ln_stdev_noises
                .entry((name1, name2))
                .or_insert_with(|| SampleMoments::default());
            collect_moments(diff_ln_stdev_noise, diff_out.stdev_diff_ln_f1_f2());

            let spec = get_spec(name1, name2);

            all_tests(
                &diff_out,
                &mut test_failures,
                scenario,
                spec.position_in_ci,
                spec.must_pass1,
                spec.must_pass2,
                spec.must_pass3,
            );
        }
    }

    let failures = test_failures.failures();
    let failed_must_pass = test_failures.failed_must_pass();
    let mut failures_summary = BTreeMap::<(&'static str, &'static str), u32>::new();

    {
        println!("*** failures ***");
        if !failures.is_empty() {
            for test_result in &failures {
                println!("{test_result:?}");
                let count = failures_summary
                    .entry((&test_result.scenario, test_result.test))
                    .or_insert(0);
                *count += 1;
            }
        } else {
            println!("none")
        }
    }

    if !failed_must_pass.is_empty() {
        println!();
        println!("*** failed_must_pass ***");
        for test_result in &failed_must_pass {
            println!("{test_result:?}");
        }
    }

    println!();
    println!("*** failures_summary ***");
    for ((scenario, test), count) in failures_summary {
        println!("{scenario} | {test} ==> count={count}");
    }

    println!();
    println!("*** noise ***");
    for (name1, name2) in fn_name_pairs {
        println!();
        println!("scenario: fn1={}, fn2={}", name1.deref(), name2.deref());
        println!(
            "ratio_medians_noise_mean={}, ratio_medians_noise_stdev={}",
            ratio_medians_noises.get(&(name1, name2)).unwrap().mean(),
            ratio_medians_noises.get(&(name1, name2)).unwrap().stdev()
        );
        println!(
            "diff_ln_stdev_noise_mean={}, diff_ln_stdev_noise_stdev={}",
            diff_ln_stdev_noises.get(&(name1, name2)).unwrap().mean(),
            diff_ln_stdev_noises.get(&(name1, name2)).unwrap().stdev()
        );
    }

    assert!(
        failed_must_pass.len() == 0,
        "{} must-pass tests have failed",
        failed_must_pass.len()
    );
}

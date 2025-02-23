use std::{collections::HashMap, fmt::Debug};

use bench_diff::{
    bench_diff_print,
    dev_utils::{calibrate_real_work, real_work},
    BenchDiffOut, LatencyUnit, PositionInCi,
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

#[derive(Debug)]
pub struct TestResult {
    pub scenario: &'static str,
    pub test: &'static str,
    pub passed: bool,
    pub result_string: String,
}

impl TestResult {
    pub fn check(
        scenario: &'static str,
        test: &'static str,
        condition: bool,
        result_string: String,
    ) -> Self {
        Self {
            scenario,
            test,
            passed: condition,
            result_string,
        }
    }

    pub fn check_eq<T: PartialEq + Debug>(
        scenario: &'static str,
        test: &'static str,
        expected: T,
        actual: T,
    ) -> Self {
        Self {
            scenario,
            test,
            passed: expected == actual,
            result_string: format!("expected={expected:?}, actual={actual:?}"),
        }
    }
}

pub struct TestResults(Vec<(TestResult, bool)>);

impl TestResults {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, result: TestResult, must_pass: bool) {
        self.0.push((result, must_pass));
    }

    pub fn failures(&self) -> Vec<&TestResult> {
        self.0.iter().map(|p| &p.0).filter(|x| !x.passed).collect()
    }

    pub fn failed_must_pass(&self) -> Vec<&TestResult> {
        self.0
            .iter()
            .filter(|p| !p.0.passed && p.1)
            .map(|x| &x.0)
            .collect()
    }
}

pub fn default_hi_median_ratio() -> f64 {
    1.01
}

pub fn default_lo_stdev_log() -> f64 {
    1.2_f64.ln() / 2.0
}

pub fn default_hi_stdev_log() -> f64 {
    2.4_f64.ln() / 2.0
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

fn cmd_line_args() -> Option<usize> {
    let mut args = std::env::args();

    let nrepeats = match args.nth(1) {
        Some(v) if v.eq("--bench") => return None,
        Some(v) => v
            .parse::<usize>()
            .expect("argument, if provided, must be integer"),
        None => return None,
    };
    Some(nrepeats)
}

pub fn bench(params: Params) {
    const ALPHA: f64 = 0.05;
    let nrepeats = cmd_line_args().unwrap_or(1);

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

    let hi_median_no_var = || {
        let effort = (base_effort as f64 * default_hi_median_ratio()) as u32;
        real_work(effort);
    };

    let mut test_results = TestResults::new();

    for i in 1..=nrepeats {
        println!("*** iteration = {i} ***");

        {
            let scenario = "f1=base_median_no_var, f2=base_median_no_var";

            let diff_out = bench_diff_print(
                params.unit,
                &base_median_no_var,
                &base_median_no_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::In,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                true,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::In,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                false,
            );
        }

        {
            let scenario = "f1=base_median_no_var, f2=hi_median_no_var";

            let diff_out = bench_diff_print(
                params.unit,
                &base_median_no_var,
                &hi_median_no_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::Above,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                true,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::Above,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                true,
            );
        }

        {
            let scenario = "f1=hi_median_no_var, f2=base_median_no_var";

            let diff_out = bench_diff_print(
                params.unit,
                &hi_median_no_var,
                &base_median_no_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::Below,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                true,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::Below,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                true,
            );
        }

        {
            let scenario = "f1=base_median_lo_var, f2=base_median_lo_var1";

            let diff_out = bench_diff_print(
                params.unit,
                &mut base_median_lo_var,
                &mut base_median_lo_var1,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::In,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                false,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::In,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                false,
            );
        }

        {
            let scenario = "f1=base_median_lo_var, f2=base_median_hi_var";

            let diff_out = bench_diff_print(
                params.unit,
                &mut base_median_lo_var,
                &mut base_median_hi_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::In,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                false,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::In,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                false,
            );
        }

        {
            let scenario = "f1=base_median_hi_var, f2=base_median_lo_var";

            let diff_out = bench_diff_print(
                params.unit,
                &mut base_median_hi_var,
                &mut base_median_lo_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::In,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                false,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::In,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                false,
            );
        }

        {
            let scenario = "f1=base_median_lo_var, f2=hi_median_lo_var";

            let diff_out = bench_diff_print(
                params.unit,
                &mut base_median_lo_var,
                &mut hi_median_lo_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::Above,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                true,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::Above,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                true,
            );
        }

        {
            let scenario = "f1=base_median_lo_var, f2=hi_median_hi_var";

            let diff_out = bench_diff_print(
                params.unit,
                &mut base_median_lo_var,
                &mut hi_median_hi_var,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "welch_position_in_ci_ratio_1",
                    PositionInCi::Above,
                    diff_out.welch_position_in_ci_ratio_1(ALPHA),
                ),
                false,
            );

            test_results.push(
                TestResult::check_eq(
                    scenario,
                    "student_position_in_ci_diff_0",
                    PositionInCi::Above,
                    diff_out.student_position_in_ci_diff_0(ALPHA),
                ),
                true,
            );
        }
    }

    let failures = test_results.failures();
    let failed_must_pass = test_results.failed_must_pass();
    let mut failures_summary = HashMap::<(&'static str, &'static str), u32>::new();

    {
        println!("*** failures ***");
        if !failures.is_empty() {
            for test_result in &failures {
                println!("{test_result:?}");
                let count = failures_summary
                    .entry((test_result.scenario, test_result.test))
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

    assert!(
        failed_must_pass.len() == 0,
        "{} must-pass tests have failed",
        failed_must_pass.len()
    );
}

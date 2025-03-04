use crate::{
    BenchDiffOut, LatencyUnit, Moments, PositionInCi, bench_diff_print, collect_moments,
    dev_utils::{calibrate_real_work, real_work},
};
use rand::{SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, LogNormal};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

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
    pub scenario: String,
    pub test: &'static str,
    pub passed: bool,
    pub result_string: String,
}

impl TestResult {
    pub fn check(
        scenario: &str,
        test: &'static str,
        condition: bool,
        result_string: String,
    ) -> Self {
        Self {
            scenario: scenario.to_owned(),
            test,
            passed: condition,
            result_string,
        }
    }

    pub fn check_eq<T: PartialEq + Debug>(
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

pub struct TestFailures(Vec<(TestResult, bool)>);

impl TestFailures {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_failure(&mut self, result: TestResult, must_pass: bool) {
        if !result.passed {
            self.0.push((result, must_pass))
        };
    }

    pub fn failures(&self) -> Vec<&TestResult> {
        self.0.iter().map(|p| &p.0).collect()
    }

    pub fn failed_must_pass(&self) -> Vec<&TestResult> {
        self.0
            .iter()
            .filter(|p| !p.0.passed && p.1)
            .map(|x| &x.0)
            .collect()
    }
}

pub const ALPHA: f64 = 0.05;

pub fn all_tests<'a>(
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

enum MyFnMut {
    Constant {
        median_effort: u32,
    },

    Variable {
        median_effort: u32,
        lognormal: LogNormal<f64>,
        rng: StdRng,
    },
}

impl MyFnMut {
    fn new_constant(median_effort: u32) -> Self {
        Self::Constant { median_effort }
    }

    fn new_variable(median_effort: u32, stdev_log: f64) -> Self {
        let mu = 0.0_f64;
        let sigma = stdev_log;
        Self::Variable {
            median_effort,
            lognormal: LogNormal::new(mu, sigma).expect("stdev_log must be > 0"),
            rng: StdRng::from_rng(&mut rand::rng()),
        }
    }

    fn invoke(&mut self) {
        match self {
            Self::Constant { median_effort } => {
                real_work(*median_effort);
            }

            Self::Variable {
                median_effort,
                lognormal,
                rng,
            } => {
                let factor = lognormal.sample(rng);
                let effort = (*median_effort as f64) * factor;
                real_work(effort as u32);
            }
        }
    }
}

fn make_base_median_no_var(base_effort: u32, _: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_constant(effort)
}

fn make_hi_median_no_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_constant(effort)
}

fn make_base_median_lo_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_hi_median_lo_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.lo_stdev_log)
}

fn make_base_median_hi_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = base_effort;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
}

fn make_hi_median_hi_var(base_effort: u32, params: &Params) -> MyFnMut {
    let effort = (base_effort as f64 * params.hi_median / params.base_median) as u32;
    MyFnMut::new_variable(effort, params.hi_stdev_log)
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

pub static SCENARIO_SPECS: [ScenarioSpec; 2] = [
    ScenarioSpec::new(
        "base_median_no_var",
        "base_median_no_var",
        PositionInCi::In,
        true,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_no_var",
        "hi_median_no_var",
        PositionInCi::Above,
        true,
        true,
        true,
    ),
];

pub struct ScenarioSpec {
    pub name1: &'static str,
    pub name2: &'static str,
    pub position_in_ci: PositionInCi,
    pub must_pass1: bool,
    pub must_pass2: bool,
    pub must_pass3: bool,
}

impl ScenarioSpec {
    pub const fn new(
        name1: &'static str,
        name2: &'static str,
        position_in_ci: PositionInCi,
        must_pass1: bool,
        must_pass2: bool,
        must_pass3: bool,
    ) -> Self {
        Self {
            name1,
            name2,
            position_in_ci,
            must_pass1,
            must_pass2,
            must_pass3,
        }
    }
}

pub fn bench_t(params: Params, scenario_specs: &[ScenarioSpec]) {
    let nrepeats = cmd_line_args().unwrap_or(1);

    let base_effort = calibrate_real_work(params.unit, params.base_median as u64);

    let named_fns: Vec<(&str, fn(u32, &Params) -> MyFnMut)> = vec![
        ("base_median_no_var", make_base_median_no_var),
        ("hi_median_no_var", make_hi_median_no_var),
        ("base_median_lo_var", make_base_median_lo_var),
        ("hi_median_lo_var", make_hi_median_lo_var),
        ("base_median_hi_var", make_base_median_hi_var),
        ("hi_median_hi_var", make_hi_median_hi_var),
    ];

    let get_fn = |name: &'static str| -> fn(u32, &Params) -> MyFnMut {
        named_fns
            .iter()
            .find(|pair| pair.0 == name)
            .expect("invalid fn name")
            .1
    };

    let mut test_failures = TestFailures::new();
    let mut ratio_medians_noises = BTreeMap::<(&'static str, &'static str), Moments>::new();
    let mut diff_ln_stdev_noises = BTreeMap::<(&'static str, &'static str), Moments>::new();

    for i in 1..=nrepeats {
        println!("*** iteration = {i} ***");

        // {
        //     let scenario = "f1=base_median_no_var, f2=base_median_no_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &base_median_no_var,
        //         &base_median_no_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     collect_mean_stdev(&mut ratio_medians_noise, diff_out.mean_diff_ln_f1_f2());
        //     collect_mean_stdev(&mut base_noise_moments2, diff_out.stdev_diff_ln_f1_f2());

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::In,
        //         true,
        //         false,
        //         true,
        //     );
        // }

        for ScenarioSpec {
            name1,
            name2,
            position_in_ci,
            must_pass1,
            must_pass2,
            must_pass3,
        } in scenario_specs
        {
            let scenario = format!("f1={name1}, f2={name2}");

            let f1 = {
                let mut my_fn = get_fn(name1)(base_effort, &params);
                move || my_fn.invoke()
            };

            let f2 = {
                let mut my_fn = get_fn(name2)(base_effort, &params);
                move || my_fn.invoke()
            };

            let diff_out = bench_diff_print(
                params.unit,
                f1,
                f2,
                params.exec_count,
                || println!("{scenario}"),
                print_diff_out,
            );

            let ratio_medians_noise = ratio_medians_noises
                .entry((name1, name2))
                .or_insert_with(|| Moments::new());
            collect_moments(ratio_medians_noise, diff_out.mean_diff_ln_f1_f2().exp());
            // println!(
            //     ">>>>> mean_diff_ln_f1_f2.exp()={}",
            //     diff_out.mean_diff_ln_f1_f2().exp()
            // );
            // println!(
            //     ">>>>> mean_ln_f1.exp()/mean_ln_f2.exp()={}",
            //     diff_out.mean_ln_f1().exp() / diff_out.mean_ln_f2().exp()
            // );

            // assert!(
            //     (diff_out.mean_diff_ln_f1_f2() - (diff_out.mean_ln_f1() - diff_out.mean_ln_f2()))
            //         .abs()
            //         < 0.000001,
            //     "bench_t: two different ways to compute mean_diff_ln_f1_f2"
            // );

            let diff_ln_stdev_noise = diff_ln_stdev_noises
                .entry((name1, name2))
                .or_insert_with(|| Moments::new());
            collect_moments(diff_ln_stdev_noise, diff_out.stdev_diff_ln_f1_f2());

            all_tests(
                &diff_out,
                &mut test_failures,
                scenario,
                *position_in_ci,
                *must_pass1,
                *must_pass2,
                *must_pass3,
            );
        }

        // {
        //     let scenario = "f1=hi_median_no_var, f2=base_median_no_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &hi_median_no_var,
        //         &base_median_no_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::Below,
        //         true,
        //         true,
        //         true,
        //     );
        // }

        // {
        //     let scenario = "f1=base_median_lo_var, f2=base_median_lo_var1";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &mut base_median_lo_var,
        //         &mut base_median_lo_var1,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::In,
        //         false,
        //         false,
        //         true,
        //     );
        // }

        // {
        //     let scenario = "f1=base_median_lo_var, f2=base_median_hi_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &mut base_median_lo_var,
        //         &mut base_median_hi_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::In,
        //         false,
        //         false,
        //         true,
        //     );
        // }

        // {
        //     let scenario = "f1=base_median_hi_var, f2=base_median_lo_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &mut base_median_hi_var,
        //         &mut base_median_lo_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::In,
        //         false,
        //         false,
        //         true,
        //     );
        // }

        // {
        //     let scenario = "f1=base_median_lo_var, f2=hi_median_lo_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &mut base_median_lo_var,
        //         &mut hi_median_lo_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::Above,
        //         true,
        //         true,
        //         true,
        //     );
        // }

        // {
        //     let scenario = "f1=base_median_lo_var, f2=hi_median_hi_var";

        //     let diff_out = bench_diff_print(
        //         params.unit,
        //         &mut base_median_lo_var,
        //         &mut hi_median_hi_var,
        //         params.exec_count,
        //         || println!("{scenario}"),
        //         print_diff_out,
        //     );

        //     all_tests(
        //         &diff_out,
        //         &mut test_failures,
        //         scenario,
        //         PositionInCi::Above,
        //         false,
        //         true,
        //         false,
        //     );
        // }
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
    for ScenarioSpec { name1, name2, .. } in scenario_specs {
        println!();
        println!("scenario: fn1={name1}, fn2={name2}");
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

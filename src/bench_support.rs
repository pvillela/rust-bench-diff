use crate::{
    BenchDiffOut, LatencyUnit, PositionInCi, SampleMoments, bench_diff, bench_diff_print,
    collect_moments,
    dev_utils::{calibrate_real_work, real_work},
};
use rand::{SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, LogNormal};
use std::{collections::BTreeMap, fmt::Debug, ops::Deref};

pub struct Params {
    pub unit: LatencyUnit,
    pub exec_count: usize,
    pub base_median: f64,
    pub hi_median: f64,
    pub lo_stdev_log: f64,
    pub hi_stdev_log: f64,
}

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

pub static FN_NAME_PAIRS: [(&'static str, &'static str); 8] = [
    ("base_median_no_var", "base_median_no_var"),
    ("base_median_no_var", "hi_median_no_var"),
    ("hi_median_no_var", "base_median_no_var"),
    ("base_median_lo_var", "base_median_lo_var"),
    ("base_median_lo_var", "base_median_hi_var"),
    ("base_median_hi_var", "base_median_lo_var"),
    ("base_median_lo_var", "hi_median_lo_var"),
    ("base_median_lo_var", "hi_median_hi_var"),
];

pub static VERBOSE: bool = false;

struct ScenarioSpec {
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

const NAMED_FNS: [(&str, fn(u32, &Params) -> MyFnMut); 6] = [
    ("base_median_no_var", make_base_median_no_var),
    ("hi_median_no_var", make_hi_median_no_var),
    ("base_median_lo_var", make_base_median_lo_var),
    ("hi_median_lo_var", make_hi_median_lo_var),
    ("base_median_hi_var", make_base_median_hi_var),
    ("hi_median_hi_var", make_hi_median_hi_var),
];

const SCENARIO_SPECS: [ScenarioSpec; 8] = [
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
    ScenarioSpec::new(
        "hi_median_no_var",
        "base_median_no_var",
        PositionInCi::Below,
        true,
        true,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "base_median_lo_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "base_median_hi_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_hi_var",
        "base_median_lo_var",
        PositionInCi::In,
        false,
        false,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "hi_median_lo_var",
        PositionInCi::Above,
        true,
        true,
        true,
    ),
    ScenarioSpec::new(
        "base_median_lo_var",
        "hi_median_hi_var",
        PositionInCi::Above,
        false,
        true,
        false,
    ),
];

fn get_fn(name: &str) -> fn(u32, &Params) -> MyFnMut {
    NAMED_FNS
        .iter()
        .find(|pair| pair.0 == name)
        .expect("invalid fn name")
        .1
}

fn get_spec(name1: &str, name2: &str) -> &'static ScenarioSpec {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.name1 == name1 && spec.name2 == name2)
        .expect("invalid fn name pair")
}

pub fn bench_t<T: Deref<Target = str>>(params: Params, fn_name_pairs: &[(T, T)], verbose: bool) {
    let nrepeats = cmd_line_args().unwrap_or(1);

    let base_effort = calibrate_real_work(params.unit, params.base_median as u64);

    let mut test_failures = TestFailures::new();
    let mut ratio_medians_noises = BTreeMap::<(&'static str, &'static str), SampleMoments>::new();
    let mut diff_ln_stdev_noises = BTreeMap::<(&'static str, &'static str), SampleMoments>::new();

    for i in 1..=nrepeats {
        println!("*** iteration = {i} ***");

        for (name1, name2) in fn_name_pairs {
            let scenario = format!("f1={}, f2={}", name1.deref(), name2.deref());

            let f1 = {
                let mut my_fn = get_fn(name1)(base_effort, &params);
                move || my_fn.invoke()
            };

            let f2 = {
                let mut my_fn = get_fn(name2)(base_effort, &params);
                move || my_fn.invoke()
            };

            let diff_out = if verbose {
                bench_diff_print(
                    params.unit,
                    f1,
                    f2,
                    params.exec_count,
                    || println!("{scenario}"),
                    print_diff_out,
                )
            } else {
                bench_diff(params.unit, f1, f2, params.exec_count)
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

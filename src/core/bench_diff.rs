//! Main module implementing functions to compare the difference in latency between two closures.

use super::{DiffOut, Timing};
use std::{
    cmp,
    io::{Write, stderr},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

static WARMUP_MILLIS: AtomicU64 = AtomicU64::new(3_000);

/// The currently defined number of milliseconds used to "warm-up" the benchmark. The default is 3,000 ms.
///
/// Use [`set_warmup_millis`] to change the value.
pub fn get_warmup_millis() -> u64 {
    WARMUP_MILLIS.load(Ordering::Relaxed)
}

/// Changes the number of milliseconds used to "warm-up" the benchmark. The default is 3,000 ms.
pub fn set_warmup_millis(millis: u64) {
    WARMUP_MILLIS.store(millis, Ordering::Relaxed);
}

const WARMUP_INCREMENT_COUNT: usize = 20;

/// Unit of time used to record latencies. Used as an argument in benchmarking functions.
#[derive(Clone, Copy, Debug)]
pub enum LatencyUnit {
    Milli,
    Micro,
    Nano,
}

impl LatencyUnit {
    /// Converts a `latency` [`Duration`] to a `u64` value according to the unit `self`.
    #[inline(always)]
    pub fn latency_as_u64(&self, latency: Duration) -> u64 {
        match self {
            Self::Nano => latency.as_nanos() as u64,
            Self::Micro => latency.as_micros() as u64,
            Self::Milli => latency.as_millis() as u64,
        }
    }

    /// Converts a `u64` value to a [`Duration`] according to the unit `self`.
    #[inline(always)]
    pub fn latency_from_u64(&self, elapsed: u64) -> Duration {
        match self {
            Self::Nano => Duration::from_nanos(elapsed),
            Self::Micro => Duration::from_micros(elapsed),
            Self::Milli => Duration::from_millis(elapsed),
        }
    }

    /// Converts a `latency` [`Duration`] to an `f64` value according to the unit `self`.
    #[inline(always)]
    pub fn latency_as_f64(&self, latency: Duration) -> f64 {
        self.latency_as_u64(latency) as f64
    }

    /// Converts an `f64` value to a [`Duration`] according to the unit `self`.
    #[inline(always)]
    pub fn latency_from_f64(&self, elapsed: f64) -> Duration {
        self.latency_from_u64(elapsed as u64)
    }
}

/// Invokes `f` once and returns its latency.
#[inline(always)]
pub fn latency(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

/// Invokes `f1` then `f2` then `f2` then `f1` and returns two pairs of latencies. For each pair,
/// the first component is an `f1` latency and the second component is an `f2` latency.
#[inline(always)]
fn duo_exec(mut f1: impl FnMut(), mut f2: impl FnMut()) -> [(Duration, Duration); 2] {
    let l01 = latency(&mut f1);
    let l02 = latency(&mut f2);

    let l12 = latency(&mut f2);
    let l11 = latency(&mut f1);

    [(l01, l02), (l11, l12)]
}

pub(crate) struct DiffState<'a> {
    hist_f1: &'a mut Timing,
    hist_f2: &'a mut Timing,
    hist_f1_lt_f2: &'a mut Timing,
    count_f1_eq_f2: &'a mut u64,
    hist_f1_gt_f2: &'a mut Timing,
    sum_f1: &'a mut i64,
    sum_f2: &'a mut i64,
    sum_ln_f1: &'a mut f64,
    sum2_ln_f1: &'a mut f64,
    sum_ln_f2: &'a mut f64,
    sum2_ln_f2: &'a mut f64,
    sum2_diff_f1_f2: &'a mut i64,
    sum2_diff_ln_f1_f2: &'a mut f64,
}

impl<'a> DiffState<'a> {
    pub fn new(out: &'a mut DiffOut) -> Self {
        Self {
            hist_f1: &mut out.hist_f1,
            hist_f2: &mut out.hist_f2,
            hist_f1_lt_f2: &mut out.hist_f1_lt_f2,
            count_f1_eq_f2: &mut out.count_f1_eq_f2,
            hist_f1_gt_f2: &mut out.hist_f1_gt_f2,
            sum_f1: &mut out.sum_f1,
            sum_f2: &mut out.sum_f2,
            sum_ln_f1: &mut out.sum_ln_f1,
            sum2_ln_f1: &mut out.sum2_ln_f1,
            sum_ln_f2: &mut out.sum_ln_f2,
            sum2_ln_f2: &mut out.sum2_ln_f2,
            sum2_diff_f1_f2: &mut out.sum2_diff_f1_f2,
            sum2_diff_ln_f1_f2: &mut out.sum2_diff_ln_f1_f2,
        }
    }

    pub fn reversed(&'a mut self) -> Self {
        Self {
            hist_f1: self.hist_f2,
            hist_f2: self.hist_f1,
            hist_f1_lt_f2: self.hist_f1_gt_f2,
            count_f1_eq_f2: self.count_f1_eq_f2,
            hist_f1_gt_f2: self.hist_f1_lt_f2,
            sum_f1: self.sum_f2,
            sum_f2: self.sum_f1,
            sum_ln_f1: self.sum_ln_f2,
            sum2_ln_f1: self.sum2_ln_f2,
            sum_ln_f2: self.sum_ln_f1,
            sum2_ln_f2: self.sum2_ln_f1,
            sum2_diff_f1_f2: self.sum2_diff_f1_f2,
            sum2_diff_ln_f1_f2: self.sum2_diff_ln_f1_f2,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.hist_f1.reset();
        self.hist_f2.reset();
        self.hist_f1_lt_f2.reset();
        *self.count_f1_eq_f2 = 0;
        self.hist_f1_gt_f2.reset();
        *self.sum_f1 = 0;
        *self.sum_f2 = 0;
        *self.sum_ln_f1 = 0.;
        *self.sum2_ln_f1 = 0.;
        *self.sum_ln_f2 = 0.;
        *self.sum2_ln_f2 = 0.;
        *self.sum2_diff_f1_f2 = 0;
        *self.sum2_diff_ln_f1_f2 = 0.;
    }

    /// Updates the state with an elapsed time for each function.
    #[inline(always)]
    pub(crate) fn capture_data(&mut self, elapsed1: u64, elapsed2: u64) {
        self.hist_f1
            .record(elapsed1)
            .expect("can't happen: histogram is auto-resizable");
        self.hist_f2
            .record(elapsed2)
            .expect("can't happen: histogram is auto-resizable");

        let diff = elapsed1 as i64 - elapsed2 as i64;

        match diff.cmp(&0) {
            cmp::Ordering::Less => self
                .hist_f1_lt_f2
                .record(diff as u64)
                .expect("can't happen: histogram is auto-resizable"),
            cmp::Ordering::Greater => self
                .hist_f1_gt_f2
                .record(-diff as u64)
                .expect("can't happen: histogram is auto-resizable"),
            cmp::Ordering::Equal => *self.count_f1_eq_f2 += 1,
        }

        assert!(elapsed1 > 0, "f1 latency must be > 0");
        *self.sum_f1 += elapsed1 as i64;
        let ln_f1 = (elapsed1 as f64).ln();
        *self.sum_ln_f1 += ln_f1;
        *self.sum2_ln_f1 += ln_f1.powi(2);

        assert!(elapsed2 > 0, "f2 latency must be > 0");
        *self.sum_f2 += elapsed2 as i64;
        let ln_f2 = (elapsed2 as f64).ln();
        *self.sum_ln_f2 += ln_f2;
        *self.sum2_ln_f2 += ln_f2.powi(2);

        let diff_f1_f2 = elapsed1 as i64 - elapsed2 as i64;
        *self.sum2_diff_f1_f2 += diff_f1_f2.pow(2);

        let diff_ln_f1_f2 = ln_f1 - ln_f2;
        *self.sum2_diff_ln_f1_f2 += diff_ln_f1_f2.powi(2);
    }

    /// Executes `f1` and `f2` repeatedly, using [`duo_exec`] `exec_count / 2` times, and captures their latencies.
    /// `pre_exec` is invoked once just before the invocations of `f1` and `f2`, and `exec_status` is invoked at the
    /// end of each iteration with [`duo_exec`].
    fn execute(
        &mut self,
        unit: LatencyUnit,
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        exec_count: usize,
        pre_exec: impl FnOnce(),
        mut exec_status: impl FnMut(usize),
        init_status_count: usize,
    ) {
        pre_exec();

        for i in 1..=exec_count / 2 {
            let pairs = duo_exec(&mut f1, &mut f2);

            for (latency1, latency2) in pairs {
                let elapsed1 = unit.latency_as_u64(latency1);
                let elapsed2 = unit.latency_as_u64(latency2);
                self.capture_data(elapsed1, elapsed2);
            }

            // `i * 2` to account for duos
            exec_status(init_status_count + i * 2);
        }
    }

    /// Warms-up the benchmark by invoking [`Self::execute`] repeatedly, each time with an `exec_count` value of
    /// [`WARMUP_INCREMENT_COUNT`], until the globally set number of warm-up millisecods [`WARMUP_MILLIS`] is
    /// reached or exceeded. `warmup_status` is invoked at the end of each invocation of [`Self::execute`].
    fn warmup(
        &mut self,
        unit: LatencyUnit,
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        mut warmup_status: impl FnMut(usize, u64, u64),
    ) {
        let warmup_millis = get_warmup_millis();
        let start = Instant::now();
        for i in 1.. {
            self.execute(
                unit,
                &mut f1,
                &mut f2,
                WARMUP_INCREMENT_COUNT,
                || {},
                |_| {},
                0,
            );
            let elapsed = Instant::now().duration_since(start);
            warmup_status(i, elapsed.as_millis() as u64, warmup_millis);
            if elapsed.ge(&Duration::from_millis(warmup_millis)) {
                break;
            }
        }
    }
}

/// Compares latencies for two closures `f1` and `f2` and *optionally* outputs information about the benchmark
/// and its execution status.
///
/// This function repeatedly executes *duos* of pairs (`f1`, `f2`), (`f2`, `f1`) and collects the resulting
/// latency data in a [`DiffOut`] object.
/// Prior to data collection, the benchmark is "warmed-up" by executing the duos of pairs for
/// [`get_warmup_millis`] milliseconds.
///
/// Arguments:
/// - `unit` - the unit used for data collection.
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function.
/// - `warmup_status` - is invoked every so often during warm-up and can be used to output the warm-up status,
///   e.g., how much warm-up time has elapsed and the target warm-up time. The first argument is the warm-up
///   execution iteration, the second is the elapsed warm-up time, and the third is the target warm-up time.
///   (See the source code of [`bench_diff_with_status`] for an example.)
/// - `pre_exec` - is invoked once at the beginning of data collection, after warm-up. It can be used,
///   for example, to output a preamble to the execution status (see `exec_status` below).
/// - `exec_status` - is invoked after the execution of each *duo* and can be used to output on the execution
///   status, e.g., how many observations have been collected for the pair of functions versus `exec_count`.
///   Its argument is the current number of executions performed.
///   (See the source code of [`bench_diff_with_status`] for an example.)
pub fn bench_diff_x(
    unit: LatencyUnit,
    mut f1: impl FnMut(),
    mut f2: impl FnMut(),
    exec_count: usize,
    mut warmup_status: impl FnMut(usize, u64, u64),
    pre_exec: impl FnOnce(),
    mut exec_status: impl FnMut(usize),
) -> DiffOut {
    let exec_count2 = exec_count / 2;

    let mut out = DiffOut::new();

    let mut state = DiffState::new(&mut out);
    state.warmup(unit, &mut f1, &mut f2, &mut warmup_status);
    state.reset();

    state.execute(
        unit,
        &mut f1,
        &mut f2,
        exec_count2,
        pre_exec,
        &mut exec_status,
        0,
    );

    let mut state_rev = state.reversed();
    state_rev.execute(
        unit,
        &mut f2,
        &mut f1,
        exec_count2,
        || (),
        &mut exec_status,
        exec_count2,
    );

    out
}

/// Compares latencies for two closures `f1` and `f2`.
///
/// This function repeatedly executes *duos* of pairs (`f1`, `f2`), (`f2`, `f1`) and collects the resulting
/// latency data in a [`DiffOut`] object.
/// Prior to data collection, the benchmark is "warmed-up" by executing the duos of pairs for
/// [`get_warmup_millis`] milliseconds.
/// This function calls [`bench_diff_x`] with no-op closures for the arguments that support the output of
/// benchmark status.
///
/// Arguments:
/// - `unit` - the unit used for data collection.
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function.
pub fn bench_diff(
    unit: LatencyUnit,
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
) -> DiffOut {
    bench_diff_x(unit, f1, f2, exec_count, |_, _, _| {}, || (), |_| ())
}

/// Compares latencies for two closures `f1` and `f2` and outputs information about the benchmark and its
/// execution status. Execution status is output to `stderr`.
///
/// This function repeatedly executes *duos* of pairs (`f1`, `f2`), (`f2`, `f1`) and collects the resulting
/// latency data in a [`DiffOut`] object.
/// Prior to data collection, the benchmark is "warmed-up" by executing the duos of pairs for
/// [`get_warmup_millis`] milliseconds.
/// This function calls [`bench_diff_x`] with pre-defined closures for the arguments that support the output of
/// benchmark status to `stderr`.
///
/// Arguments:
/// - `unit` - the unit used for data collection.
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function.
/// - `header` - is invoked once at the start of this function's execution; it can be used, for example,
///   to output information about the functions being compared to `stdout` and/or `stderr`. The first
///   argument is the the `LatencyUnit` and the second argument is the `exec_count`.
pub fn bench_diff_with_status(
    unit: LatencyUnit,
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
    header: impl FnOnce(LatencyUnit, usize),
) -> DiffOut {
    header(unit, exec_count);

    let warmup_status = {
        let mut status_len: usize = 0;

        move |_: usize, elapsed_millis: u64, warmup_millis: u64| {
            if status_len == 0 {
                eprint!("Warming up ... ");
                stderr().flush().expect("unexpected I/O error");
            }
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{elapsed_millis} millis of {warmup_millis}.");
            if elapsed_millis.lt(&warmup_millis) {
                status_len = status.len();
            } else {
                status_len = 0; // reset status in case of multiple warm-up phases
            };
            eprint!("{status}");
            stderr().flush().expect("unexpected I/O error");
        }
    };

    let pre_exec = || {
        eprint!(" Executing bench_diff ... ");
        stderr().flush().expect("unexpected I/O error");
    };

    let exec_status = {
        let mut status_len: usize = 0;

        move |i| {
            eprint!("{}", "\u{8}".repeat(status_len));
            let status = format!("{i} of {exec_count}.");
            status_len = status.len();
            eprint!("{status}");
            stderr().flush().expect("unexpected I/O error");
        }
    };

    bench_diff_x(
        unit,
        f1,
        f2,
        exec_count,
        warmup_status,
        pre_exec,
        exec_status,
    )
}

#[cfg(test)]
#[cfg(feature = "_test_support")]
#[allow(clippy::type_complexity)]
mod test {
    use super::*;
    use crate::{
        dev_utils::nest_btree_map,
        test_support::{
            ALPHA, BETA, Claim, ClaimResults, HI_1PCT_FACTOR, HI_10PCT_FACTOR, HI_25PCT_FACTOR,
            ScaleParams, default_hi_stdev_ln, default_lo_stdev_ln, get_scale_params, get_scenario,
        },
    };
    use rand::{SeedableRng, distr::Distribution, prelude::StdRng};
    use rand_distr::LogNormal;
    use std::{fmt::Debug, ops::Deref};

    #[allow(clippy::large_enum_variant)]
    enum MyFnMut {
        Det {
            median: f64,
        },

        NonDet {
            median: f64,
            lognormal: LogNormal<f64>,
            rng: StdRng,
        },
    }

    impl MyFnMut {
        fn new_deterministic(median: f64) -> Self {
            Self::Det { median }
        }

        fn new_non_deterministic(median: f64, stdev_ln: f64) -> Self {
            let mu = 0.0_f64;
            let sigma = stdev_ln;
            Self::NonDet {
                median,
                lognormal: LogNormal::new(mu, sigma).expect("stdev_ln must be > 0"),
                rng: StdRng::from_rng(&mut rand::rng()),
            }
        }

        pub fn invoke(&mut self) -> f64 {
            match self {
                Self::Det { median } => *median,

                Self::NonDet {
                    median,
                    lognormal,
                    rng,
                } => {
                    let factor = lognormal.sample(rng);
                    *median * factor
                }
            }
        }
    }

    const NAMED_FNS: [(&str, fn(f64) -> MyFnMut); 12] = {
        [
            ("base_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median)
            }),
            ("hi_1pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_1PCT_FACTOR)
            }),
            ("hi_10pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_10PCT_FACTOR)
            }),
            ("hi_25pct_median_no_var", |base_median| {
                MyFnMut::new_deterministic(base_median * HI_25PCT_FACTOR)
            }),
            ("base_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median, default_lo_stdev_ln())
            }),
            ("hi_1pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_1PCT_FACTOR, default_lo_stdev_ln())
            }),
            ("hi_10pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_10PCT_FACTOR, default_lo_stdev_ln())
            }),
            ("hi_25pct_median_lo_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_25PCT_FACTOR, default_lo_stdev_ln())
            }),
            ("base_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median, default_hi_stdev_ln())
            }),
            ("hi_1pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_1PCT_FACTOR, default_hi_stdev_ln())
            }),
            ("hi_10pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_10PCT_FACTOR, default_hi_stdev_ln())
            }),
            ("hi_25pct_median_hi_var", |base_median| {
                MyFnMut::new_non_deterministic(base_median * HI_25PCT_FACTOR, default_hi_stdev_ln())
            }),
        ]
    };

    fn get_fn(name: &str) -> fn(f64) -> MyFnMut {
        NAMED_FNS
            .iter()
            .find(|pair| pair.0 == name)
            .unwrap_or_else(|| panic!("invalid fn name: {name}"))
            .1
    }

    fn diff_x(
        mut f1: impl FnMut() -> f64,
        mut f2: impl FnMut() -> f64,
        exec_count: usize,
    ) -> DiffOut {
        let mut out = DiffOut::new();
        let mut state = DiffState::new(&mut out);

        for _ in 1..=exec_count {
            let (elapsed1, elapsed2) = (f1() as u64, f2() as u64);
            state.capture_data(elapsed1, elapsed2);
        }

        out
    }

    fn run_with_claims<T: Deref<Target = str> + Debug>(
        scale_params: &ScaleParams,
        name1: T,
        name2: T,
        verbose: bool,
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
            println!("FN_NAME_PAIR=\"({name1:?}, {name2:?})\"");
            println!("VERBOSE=\"{verbose}\"");
            println!("nrepeats={nrepeats}");
            println!("run_name=\"{run_name}\"");
        };

        println!();
        print_args();
        println!();

        let scenario = get_scenario(&name1, &name2);

        let mut f1 = {
            let mut my_fn = get_fn(&name1)(scale_params.base_median);
            move || my_fn.invoke()
        };

        let mut f2 = {
            let mut my_fn = get_fn(&name2)(scale_params.base_median);
            move || my_fn.invoke()
        };

        let mut results = ClaimResults::new();

        for _ in 1..=nrepeats {
            let diff_out = diff_x(&mut f1, &mut f2, scale_params.exec_count);
            scenario.check_claims(&mut results, &diff_out, verbose);
        }

        if verbose {
            println!("*** failures ***");
            for claim_result in results.failures().iter() {
                println!("{claim_result:?}");
            }

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

        let type_i_and_ii_errors_2sigma =
            results.excess_type_i_and_ii_errors(ALPHA, BETA, &Claim::CRITICAL_NAMES, nrepeats, 2.);
        assert!(
            type_i_and_ii_errors_2sigma.is_empty(),
            "\n*** type_i_and_ii_errors_2sigma: {:?}\n",
            nest_btree_map(type_i_and_ii_errors_2sigma)
        );
    }

    const SCALE_NAMES: [&str; 1] = [
        "micros_scale",
        // "millis_scale",
        // "nanos_scale"
    ];

    #[test]
    fn test_base_median_lo_var_base_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "base_median_lo_var",
                false,
                300,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_base_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "base_median_hi_var",
                false,
                300,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_1pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_1pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_10pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_10pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_25pct_median_lo_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_25pct_median_lo_var",
                false,
                100,
                "test",
            );
        }
    }

    // Below test always fails due to insufficient sample size for required BETA.
    // #[test]
    // fn test_base_median_lo_var_hi_1pct_median_hi_var() {
    //     for name in SCALE_NAMES {
    //         let scale = get_scale_params(name);
    //         run_with_claims(
    //             scale,
    //             "base_median_lo_var",
    //             "hi_1pct_median_hi_var",
    //             false,
    //             100,
    //             "test",
    //         );
    //     }
    // }

    #[test]
    fn test_base_median_lo_var_hi_10pct_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_10pct_median_hi_var",
                false,
                100,
                "test",
            );
        }
    }

    #[test]
    fn test_base_median_lo_var_hi_25pct_median_hi_var() {
        for name in SCALE_NAMES {
            let scale = get_scale_params(name);
            run_with_claims(
                scale,
                "base_median_lo_var",
                "hi_25pct_median_hi_var",
                false,
                100,
                "test",
            );
        }
    }
}

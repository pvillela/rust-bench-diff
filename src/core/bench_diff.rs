//! Main module implementing functions to compare the difference in latency between two closures.

use super::DiffOut;
use bench_utils::{BenchCfg, BenchOut, LatencyUnit, latency};
use std::{
    cmp,
    io::{Write, stderr},
    ops::Deref,
    sync::Mutex,
    time::{Duration, Instant},
};

static BENCH_CFG: Mutex<BenchCfg> = Mutex::new(BenchCfg::new(
    3000,
    LatencyUnit::Nano,
    LatencyUnit::Micro,
    3,
    &BENCH_CFG,
));

pub fn get_bench_cfg() -> BenchCfg {
    let guard = BENCH_CFG.lock().unwrap();
    guard.deref().clone()
}

const WARMUP_INCREMENT_COUNT: usize = 20;

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
    out_f1: &'a mut BenchOut,
    out_f2: &'a mut BenchOut,
    count_f1_lt_f2: &'a mut u64,
    count_f1_eq_f2: &'a mut u64,
    count_f1_gt_f2: &'a mut u64,
    sum2_diff_f1_f2: &'a mut i64,
    sum2_diff_ln_f1_f2: &'a mut f64,
}

impl<'a> DiffState<'a> {
    pub fn new(out: &'a mut DiffOut) -> Self {
        Self {
            out_f1: &mut out.out_f1,
            out_f2: &mut out.out_f2,
            count_f1_lt_f2: &mut out.count_f1_lt_f2,
            count_f1_eq_f2: &mut out.count_f1_eq_f2,
            count_f1_gt_f2: &mut out.count_f1_gt_f2,
            sum2_diff_f1_f2: &mut out.sum2_diff_f1_f2,
            sum2_diff_ln_f1_f2: &mut out.sum2_diff_ln_f1_f2,
        }
    }

    pub fn reversed(&'a mut self) -> Self {
        Self {
            out_f1: self.out_f2,
            out_f2: self.out_f1,
            count_f1_lt_f2: self.count_f1_gt_f2,
            count_f1_eq_f2: self.count_f1_eq_f2,
            count_f1_gt_f2: self.count_f1_lt_f2,
            sum2_diff_f1_f2: self.sum2_diff_f1_f2,
            sum2_diff_ln_f1_f2: self.sum2_diff_ln_f1_f2,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.out_f1.reset();
        self.out_f2.reset();
        *self.count_f1_lt_f2 = 0;
        *self.count_f1_eq_f2 = 0;
        *self.count_f1_gt_f2 = 0;
        *self.sum2_diff_f1_f2 = 0;
        *self.sum2_diff_ln_f1_f2 = 0.;
    }

    /// Updates the state with an elapsed time for each function.
    #[inline(always)]
    pub(crate) fn capture_data(&mut self, elapsed1: u64, elapsed2: u64) {
        self.out_f1.capture_data(elapsed1);
        self.out_f2.capture_data(elapsed2);

        let diff = elapsed1 as i64 - elapsed2 as i64;

        match diff.cmp(&0) {
            cmp::Ordering::Less => *self.count_f1_lt_f2 += 1,
            cmp::Ordering::Equal => *self.count_f1_eq_f2 += 1,
            cmp::Ordering::Greater => *self.count_f1_gt_f2 += 1,
        }

        assert!(elapsed1 > 0, "f1 latency must be > 0");
        let ln_f1 = (elapsed1 as f64).ln();

        assert!(elapsed2 > 0, "f2 latency must be > 0");
        let ln_f2 = (elapsed2 as f64).ln();

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
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        exec_count: usize,
        pre_exec: impl FnOnce(),
        mut exec_status: impl FnMut(usize),
        init_status_count: usize,
    ) {
        pre_exec();

        let recording_unit = get_bench_cfg().recording_unit();
        for i in 1..=exec_count / 2 {
            let pairs = duo_exec(&mut f1, &mut f2);

            for (latency1, latency2) in pairs {
                let elapsed1 = recording_unit.latency_as_u64(latency1);
                let elapsed2 = recording_unit.latency_as_u64(latency2);
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
        mut f1: impl FnMut(),
        mut f2: impl FnMut(),
        mut warmup_status: impl FnMut(usize, u64, u64),
    ) {
        let warmup_millis = get_bench_cfg().warmup_millis();
        let start = Instant::now();
        for i in 1.. {
            self.execute(&mut f1, &mut f2, WARMUP_INCREMENT_COUNT, || {}, |_| {}, 0);
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
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function. If it is not a multiple of 4, the
///   closest multiple of 4 less than it will be used.
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
    state.warmup(&mut f1, &mut f2, &mut warmup_status);
    state.reset();

    state.execute(&mut f1, &mut f2, exec_count2, pre_exec, &mut exec_status, 0);

    let mut state_rev = state.reversed();
    state_rev.execute(
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
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function. If it is not a multiple of 4, the
///   closest multiple of 4 less than it will be used.
pub fn bench_diff(f1: impl FnMut(), f2: impl FnMut(), exec_count: usize) -> DiffOut {
    bench_diff_x(f1, f2, exec_count, |_, _, _| {}, || (), |_| ())
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
/// - `f1` - first target for comparison.
/// - `f2` - second target for comparison.
/// - `exec_count` - number of executions (sample size) for each function. If it is not a multiple of 4, the
///   closest multiple of 4 less than it will be used.
/// - `header` - is invoked once at the start of this function's execution; it can be used, for example,
///   to output information about the functions being compared to `stdout` and/or `stderr`. The
///   argument is the `exec_count`.
pub fn bench_diff_with_status(
    f1: impl FnMut(),
    f2: impl FnMut(),
    exec_count: usize,
    header: impl FnOnce(usize),
) -> DiffOut {
    header(exec_count);

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

    bench_diff_x(f1, f2, exec_count, warmup_status, pre_exec, exec_status)
}

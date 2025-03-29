This library supports **reliable latency comparison** between two functions/closures. This library provides commonly used latency metrics for target functions (mean, standard deviation, median, percentiles, min, max), but it differentiates itself by providing support for the statistically rigorous comparison of latencies between two functions.

One could simply run tools like [Criterion](https://crates.io/crates/criterion) or [Divan](https://crates.io/crates/divan) on each function and compare the results. However, a couple of challenges arise:

- ***Ordering effect*** -- When running two benchmarks, one after the other, the first one may (and often does) get an edge over the second one, or vice-versa, due to changing machine conditions between the two runs. This can significantly distort the comparison. In some cases, a function that is known by construction to be 5% faster than another function can show a mean latency and/or median latency that is higher than the corresponding metric for the slower function.
- ***Time-dependent random noise*** -- Random noise can and often does change over time. This is a contributor to the *ordering effect*. Increasing the sample size (number of repetitions) can narrow the variance of a function's median or mean latency as measured at a point in time, but it is not effective by itself in mitigating the time-dependent variability.

One could attempt to address these challenges by running the individual benchmarks, one after the other, repeating that multiple times, and using appropriate statistical methods to compare the two observed latency distributions. This is, however, quite time consuming and requires careful planning and analysis.

The present library provides a convenient, efficient, and statistically sound alternative to the above cumbersome methodology.

# Quick Start

To run a benchmark with `bench_diff`, follow these simple steps:

1. Create a bench file -- see [Simple Bench Example](#simple-bench-example) for a representative example.

1. Create in `Cargo.toml` a `bench` section corresponding to the bench file. For example, given a file `benches/my_bench.rs`, add the following to `Cargo.toml`:

   ```toml
   [[bench]]
   name = "my_bench"
   harness = false
   ```

1. Execute the benchmark the usual way:
   
   ```bash
   cargo bench --bench my_bench
   ```

# Implementation Approach

This library addresses the ordering effect and random noise challenges as follows. Given two functions `f1` and `f2`:

- It repeatedly executes *duos* of pairs (`f1`, `f2`), (`f2`, `f1`). This ensures the following:
  - Obviously, both functions are executed the same number of times.
  - Each function is executed as many times preceded by itself as preceded by the other function. This removes the ordering effect.
  - Because the function excutions are paired, those executions are in close time proximity to each other, so the time-dependent variation is effectively neutralized in the comparison of latencies (even though it may persist in the latency distributions of the individual functions).
  - Latencies can be compared pairwise, as well as overall for both functions. This enables the analysis of data with statistical methods targeting either two independent samples or a single paired sample.
- The number of function executions can be specified according to the desired levels of confidence and fine-grained discrimination. More on this in the [Statistical Details](#statistical-details) section.
- It warms-up by executing the above pattern for 3 seconds before it starts tallying the results. This is similar to the `Criterion` warm-up strategy.

CONTINUE HERE
- It maintains four histograms:
  - `hist_f1`: captures the mean latency of `f1` over each inner loop.
  - `hist_f2`: captures the mean latency of `f2` over each inner loop.
  - `hist_f1_ge_f2`: captures the mean latency of `f1` minus the mean latency of `f2` over each inner loop, when the difference is greater than or equal to zero.
  - `hist_f1_lt_f2`: captures the mean latency of `f1` minus the mean latency of `f2` over each inner loop, when the difference is negative.
- Due to the execution structure with the outer and inner loops, groups of executions of `f1` and `f2` alternate repeatedly, effectively eliminating order bias.
- The inner loops address the noise problem by computing the respective means over a substantial number of iterations and assigning the resulting difference to either `hist_f1_ge_f2` or `hist_f1_lt_f2`.
- A simple comparison of the number of observations in `hist_f1_ge_f2` and `hist_f1_lt_f2` yields a reliable indicator of which function is faster, provided that the number of outer and inner loop iterations are reasonably large. If the count in `hist_f1_lt_f2` is higher then `f1` is faster, and vice-versa.
- The summary statistics for the four histograms are generated as well, but the key hypothesis testing purpose of this library is addressed by the above point.

# Statistical Details

 [MENTION LOGNORMAL ASSUMPTION, TESTS INDICATE X NUMBER OF EXECUTIONS OK FOR VALIDITY OF LOGNORMAL APPROXIMATION AND LOGNORMAL-BASED CONFIDENCE INTERVALS AND HYPOTHESIS TESTS AT MICROS SCALE, AS WELL AS TYPE II ERRORS WITH CERTAIN VARIANCE BOUNDS. SAMPLE SIZES (EXEC COUNTS) FOR A DESIRED TYPE II ERROR CAN BE FURTHER CALCULATED USING (PROVIDE REFERENCES).]

# Testing

This framework has been extensively tested:

- Test of inferential statistics
- Test benchmarks using both deterministic functions with known latency medians and non-deterministic functions with specified latency medians and variances.

# Examples


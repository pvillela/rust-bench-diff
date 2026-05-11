This library supports **reliable latency comparison** between two functions/closures. This can be trickier than it may seem, due to time-dependent random noise and ordering effects. This library differentiates itself by providing APIs for the statistically rigorous comparison of latencies between two functions, as well as commonly used latency metrics for the target functions (mean, standard deviation, median, percentiles, min, max).

One could simply use crates like [bench_utils](https://crates.io/crates/bench_utils), [Criterion](https://crates.io/crates/criterion) or [Divan](https://crates.io/crates/divan) on each function and compare the results. However, these challenges arise:

- ***Time-dependent random noise*** -- Random noise can and often does change over time. This can distort the comparison. (This is also a contributor to the *ordering effect* -- see below). Increasing the sample size (number of function executions) can narrow the variance of a function's median or mean latency as measured at a point in time, but it is not effective by itself in mitigating the time-dependent variability.
- ***Ordering effect*** -- When running two benchmarks, one after the other, it is sometimes observed that the first one may get an edge over the second one (or vice-versa). This can also distort the comparison.
- ***Command-line orientation*** -- Crates like `Criterion` and `Divan` focus on the generation of outputs to `stdout` and graphics, rather than APIs to programmatically access and process their outputs. As a result, additional processing of outputs (e.g., latency comparison between two functions) may require manual work or parsing of output files.

At the microseconds latency magnitude and under noisy conditions, a function that is known by construction to be 5% faster than another function could show a mean latency and/or median latency higher than the corresponding metric for the slower function. This effect would be less pronounced for latencies at the milliseconds magnitude or higher, but it could still distort results. In general, the smaller the difference between the latencies of the two functions, the harder it is to distinguish the faster function from the slower one with the usual benchmarking approaches.

One could tackle these challenges by running the individual benchmarks, one after the other, under ideal conditions (with the minimization of other loads on the system), repeating that multiple times, and using appropriate statistical methods to compare the two observed latency distributions. This is, however, quite time consuming and requires careful planning and analysis.

The present library has been validated by extensive benchmark [testing](#testing) and provides a convenient, efficient, and statistically sound alternative to the above more cumbersome methodology.

For the mathematically inclined reader, a later section presents a simple [model](#a-model-of-time-dependent-random-noise) to reason about time-dependent random noise.

# Quick Start

To run a benchmark with `bench_diff`, follow these simple steps:

1. Create a bench file -- see [Simple Bench Example](#simple-bench-example) for a representative example.

1. Add the `bench_diff` dependency in `Cargo.toml`:

   ```toml
   [dev-dependencies]
   bench_diff = "2.0"
   ```

1. Create in `Cargo.toml` a `bench` section corresponding to the bench file. For example, given a file `benches/simple_bench.rs`, add the following to `Cargo.toml`:

   ```toml
   [[bench]]
   name = "simple_bench"
   harness = false
   ```

1. Execute the benchmark the usual way:
   
   ```bash
   cargo bench --bench simple_bench
   ```

# Simple Bench Example

This example includes two comparisons of latencies of two functions and prints statistics for each comparison and each function.


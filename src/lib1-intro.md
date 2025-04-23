This library supports **reliable latency comparison** between two functions/closures. This is trickier than it may seem, due to time-dependent random noise and ordering effects. This library provides commonly used latency metrics for the target functions (mean, standard deviation, median, percentiles, min, max), but it differentiates itself by providing support for the statistically rigorous comparison of latencies between two functions.

One could simply try to run tools like [Criterion](https://crates.io/crates/criterion) or [Divan](https://crates.io/crates/divan) on each function and compare the results. However, these challenges arise:

- ***Time-dependent random noise*** -- Random noise can and often does change over time. This can significantly distort the comparison. (This is also a contributor to the *ordering effect* -- see below). Increasing the sample size (number of function executions) can narrow the variance of a function's median or mean latency as measured at a point in time, but it is not effective by itself in mitigating the time-dependent variability.
- ***Ordering effect*** -- When running two benchmarks, one after the other, it is not uncommon to observe that the first one may get an edge over the second one. This can also significantly distort the comparison.

Due to these effects, at the microseconds latency magnitude, a function that is known by construction to be 5% faster than another function can often show a mean latency and/or median latency that is higher than the corresponding metric for the slower function. This effect is less pronounced for latencies at the milliseconds magnitude or higher, but it can still distort results. In general, the smaller the difference between the latencies of the two functions, the harder it is to distinguish the faster function from the slower one with the usual benchmarking approaches.

One could tackle these challenges by running the individual benchmarks, one after the other, repeating that multiple times, and using appropriate statistical methods to compare the two observed latency distributions. This is, however, quite time consuming and requires careful planning and analysis.

The present library has been validated by extensive benchmark [testing](#testing) and provides a convenient, efficient, and statistically sound alternative to the above more cumbersome methodology.

For the mathematically inclined reader, a later section presents a simple [model](#a-model-of-time-dependent-random-noise) to reason about time-dependent random noise.

# Quick Start

To run a benchmark with `bench_diff`, follow these simple steps:

1. Create a bench file -- see [Simple Bench Example](#simple-bench-example) for a representative example.

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


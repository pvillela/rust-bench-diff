# Implementation Approach

This library addresses the ordering effect and random noise challenges as follows. Given two functions `f1` and `f2`:

- It repeatedly executes *duos* of pairs (`f1`, `f2`), (`f2`, `f1`). This ensures the following:
  - Obviously, both functions are executed the same number of times.
  - Each function is executed as many times preceded by itself as preceded by the other function. This removes the ordering effect.
  - Because the function excutions are paired, those executions are in close time proximity to each other, so the time-dependent variation is effectively neutralized in the comparison of latencies (even though it may persist in the latency distributions of the individual functions).
  - Latencies can be compared pairwise, as well as overall for both functions. This enables the analysis of data with statistical methods targeting either two independent samples or a single paired sample.
- The number of function executions can be specified according to the desired levels of confidence and fine-grained discrimination.
- It warms-up by executing the above pattern for some time (3 seconds by default) before it starts tallying the results. This is similar to the `Criterion` warm-up strategy.

An important tool used for data collection in this library is the [hdrhistogram](https://docs.rs/hdrhistogram/latest/hdrhistogram/) crate, which provides an efficient histogram implementation with high fidelity for wide ranges of positive values. Additionally, accumulators are used to support some inferential statistics.

# Statistics

Standard summary statistics (mean, standard deviation, median, percentiles, min, max) and a variety of inferential statistics (e.g., hypothesis tests, confidence intervals, t values, p values) are implemented for the [`DiffOut`] struct, which is output from the library's core benchmarking functions.

Much of the statistical inference supported by this library is based on the assumption that latency distributions tend to have an approximately lognormal distribution. This is a widely made assumption, which is supported by theory as well as empirical data. While the lognormal assumption underlies much of the statistical inference capability in this library, benchmarks using functions with controlled latencies have been conducted (see [Testing](#testing) below) to validate the soundness of the statistics.

**t-tests**

The [`DiffOut`] methods whose names include the string `welch` correspond to statistics associated with the Welch two-sample t-test for the difference between the means of the natural logarithms of the latencies of `f1` and `f2`. 

*Note: There are also [`DiffOut`] methods whose names include the string `student`. Those methods correspond to statistics associated with the Student one-sample t-test for equality to 0 of the mean of the differences of the natural logarithms of the paired latencies of `f1` and `f2`. Those methods have been deprecated as the results of further benchmark testing showed that they have substantially higher Type I error rates than the corresponding `welch` statistics.*

# Testing

This framework has been extensively tested using both standard Rust tests and test benchmarks.

**Tests of inferential statistics** -- The implementations of inferential statistics (hypothesis tests, confidence intervals, t values, p values, etc.) have been tested independently on their own, using Rust's standard testing approach. Sources for comparison included published data sets with their corresponding statistics, as well as calculations using R.

**Test benchmarks** -- Specific benchmarking scenarios were defined to test the library on its own and in comparison with "*traditional*" benchmarking.

- Comparative test benchmarks -- Using deterministic functions with specified latency medians, we compared results from `bench_diff` against results from the *traditional* separate benchmarking of the functions.
  - Deterministic functions `f1` and `f2` were defined such that the relative difference between the latency of `f1` and the latency of `f2` was known by construction.
    - Separate comparative tests were conducted with relative differences of 1%, 2%, 5%, and 10%.
    - Two base median latencies were used: 100 microseconds and 20 milliseconds.
  - A sample size (`exec_count`) of 2,000 was used with the base median of 200 microseconds and a sample size of 200 was used with the base median of 20 milliseconds.
  - For each comparative test:
    - A <u>*traditional* benchmarking suite</u> was conducted by benchmarking `f1` with `exec_count` executions, then benchmarking `f2` with `exec_count` executions, then benchmarking `f1` with `exec_count` executions, then benchmarking `f2` with `exec_count` executions, and so on, until both `f1` and `f2` had been benchmarked 100 times.
    - A <u>*bench_diff* benchmarking suite</u> was conducted by running `bench_diff`, with `f1`, `f2`, and `exec_count` as arguments, 100 times.
- Standalone `bench_diff` test benchmarks -- These used both deterministic functions with known latency medians and non-deterministic functions with specified latency medians and variances. Each benchmark was run repeatedly (100 times) to empirically verify the frequency of Type I and Type II statistical hypothesis testing errors.
  - Benchmarks included functions with relative latency differences of 0%, 1%, and 10%. The 0% cases are important to verify that the benchmark does not mistakenly conclude that one function is faster than the other when they in fact have the same median latency (i.e., a statistical Type I error).
  - Two base median latencies were used: 100 microseconds and 20 milliseconds.
  - A sample size (`exec_count`) of 2,000 was used with the base median of 200 microseconds and a sample size of 200 was used with the base median of 20 milliseconds.
  - Three variance levels were used: 0 (deterministic functions), low (standard deviation of the log of the distribution = ln(1.2) / 2, which corresponds to a multiplicative effect of 1.095 at the 68th percentile for a lognormal distribution), and high (standard deviation of the log of the distribution = ln(2.4) / 2, which corresponds to a multiplicative effect of 1.55 at the 68th percentile for a lognormal distribution).


**General observations** -- Following are general observations from the test benchmark results:

- Comparative test benchmarks

  - The raw data shows that relative latency differences measured with `bench_diff` are invariably more accurate than those measured with the *traditional* method.

  - We use the following definitions in the comparison table below:

    - A median and/or mean ***reversal*** occurs when the measured median and/or mean latency of `f2` is higher than that of `f1`. Recall that, by construction, `f1` has higher latency than `f2`.
    - A median and/or mean ***anomaly*** occurs when the measured relative difference of the medians and/or means of the two functions' latencies differs by more than 40% from the known relative difference.

  - Comparison table -- The following conclusions can be gleaned from the table below:

      - Reversals and anomalies were much more frequent with the *traditional* method than with `bench_diff`.
      - For latency magnitudes of hundreds of microseconds, reversals and anomalies were pervasive with the *traditional* method, while they were very low or non-existent with `bench_diff`. Furthermore, even when reversals were present with `bench_diff`, the t-tests always correctly identified which of the two functions was faster.
      - For latency magnitudes of tens of milliseconds, reversals and anomalies were lower for both the *traditional* method and `bench_diff`. They were practically non-existent with `bench_diff` for relative latency differences as low as 1%. By contrast, the frequencies of reversals and anomalies were still substantial with the *traditional* method for relative latency differences of 2% or lower. 
      
      | Benchmark type | Base median latency | Sample size | Relative difference | Reversals | Anomalies | t_test pass |
      | -------------- | :-----------------: | :-----------------: | :-------: | :-------: | :----------------: | :----------------: |
      | *traditional*  | 100 µs        |         2,000         |         1%          | 47 | 94 | N/A       |
      | `bench_diff`   | 100 µs | 2,000 | 1% | 2 | 13 | all |
      | *traditional*  | 100 µs | 2,000 | 2% | 35 | 89 | N/A |
      | `bench_diff` | 100 µs | 2,000 | 2% | 0 | 5 | all |
      | *traditional*  | 100 µs | 2,000 | 5% | 21 | 74 | N/A |
      | `bench_diff` | 100 µs | 2,000 | 5% | 0 | 1 | all |
    | *traditional*  | 100 µs | 2,000 | 10% | 19 | 57 | N/A |
    | `bench_diff` | 100 µs | 2,000 | 10% | 0 | 0 | all |
    | *traditional* | 20 ms | 200 | 1% | 31 | 82 | N/A |
    | `bench_diff` | 20 ms | 200 | 1% | 0 | 2 | all |
    | *traditional* | 20 ms | 200 | 2% | 24 | 75 | N/A |
    | `bench_diff` | 20 ms | 200 | 2% | 0 | 0 | all |
    | *traditional* | 20 ms | 200 | 5% | 0 | 6 | N/A |
    | `bench_diff` | 20 ms | 200 | 5% | 0 | 0 | all |
    | *traditional* | 20 ms | 200 | 10% | 0 | 1 | N/A |
    | `bench_diff` | 20 ms | 200 | 10% | 0 | 0 | all |
    

- Standalone `bench_diff` test benchmarks
  - For comparisons between deterministic functions (i.e., no randomness in the functions themselves):
    - Type I error frequency was in line (within 2 sigma) with a chosen alpha of 0.05 and and the corresponding binomial distribution (p=0.05, n=100).
    - Type II error frequency was in line (within 2 sigma) with a beta of 0.05 and the corresponding binomial distribution (p=0.05, n=100) for the sample sizes (`exec_count`) used, which were the same as those used in the comparative test benchmarks.
  - For comparisons where at least one of the functions was non-deterministic, with base median of 100 microseconds and sample size of 2,000:
    - Type I error frequency was in line (within 2 sigma) with a chosen alpha of 0.05 and and the corresponding binomial distribution (p=0.05, n=100).
    - In all but one case, Type II error frequency was in line (within 2 sigma) with a beta of 0.05 and the corresponding binomial distribution (p=0.05, n=100) for the sample size used. The only exception was a comparison involving a small relative latency difference (1%) and a high latency variance.

  - For comparisons where at least one of the functions was non-deterministic, with base median of 20 milliseconds and sample size of 200:
    - Type I error frequency was in line (within 2 sigma) with a chosen alpha of 0.05 and and the corresponding binomial distribution (p=0.05, n=100).
    - In most cases, Type II error frequency was in line (within 2 sigma) with a beta of 0.05 and the corresponding binomial distribution (p=0.05, n=100) for the sample size used. The exceptions were the comparisons involving either a small relative latency difference (1%) together with low or high variance, or a moderate relative latency difference (10%) together with a high latency variance.
  - Unsurprisingly, the observed Type II errors are higher when the difference in means/medians is smaller and/or the variance is higher. Of course, Type II errors can be reduced by increasing the sample size (and testing time), which reduces beta.


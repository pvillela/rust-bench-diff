# bench_diff

This library supports **reliable latency comparison** between two functions/closures. This is trickier than it may seem, due to time-dependent random noise and ordering effects. This library provides commonly used latency metrics for the target functions (mean, standard deviation, median, percentiles, min, max), but it differentiates itself by providing support for the statistically rigorous comparison of latencies between two functions.

One could simply try to run tools like [Criterion](https://crates.io/crates/criterion) or [Divan](https://crates.io/crates/divan) on each function and compare the results. However, these challenges arise:

- ***Time-dependent random noise*** -- Random noise can and often does change over time. This can significantly distort the comparison. (This is also a contributor to the *ordering effect* -- see below). Increasing the sample size (number of function executions) can narrow the variance of a function's median or mean latency as measured at a point in time, but it is not effective by itself in mitigating the time-dependent variability.
- ***Ordering effect*** -- When running two benchmarks, one after the other, it is not uncommon to observe that the first one may get an edge over the second one. This can also significantly distort the comparison.

Due to these effects, at the microseconds latency magnitude, a function that is known by construction to be 5% faster than another function can often show a mean latency and/or median latency that is higher than the corresponding metric for the slower function. This effect is less pronounced for latencies at the milliseconds magnitude or higher, but it can still distort results. In general, the smaller the difference between the latencies of the two functions, the harder it is to distinguish the faster function from the slower one with the usual benchmarking approaches.

One could tackle these challenges by running the individual benchmarks, one after the other, repeating that multiple times, and using appropriate statistical methods to compare the two observed latency distributions. This is, however, quite time consuming and requires careful planning and analysis.

The present library provides a convenient, efficient, and statistically sound alternative to the above cumbersome methodology.

## Documentation

See the [API documentation](https://docs.rs/bench_diff/latest/bench_diff/) on docs.rs. The documentation includes a comprehensive overview, with a description of the methodology used to develop and validate the library, and usage examples. See also the source [repo](https://github.com/pvillela/rust-bench-diff/tree/main).

## Support

Create a [new issue](https://github.com/pvillela/rust-bench-diff/issues/new) on GitHub.

## License

This library is distributed under the terms of the MIT license, with copyright retained by the author.

See [LICENSE](https://github.com/pvillela/rust-bench-diff/tree/main/LICENSE) for details.

#!/bin/bash

RUSTFLAGS="-Awarnings" cargo bench --bench bench_micros_in_nanos --all-features --target-dir target/bench-target -- 10

# cargo bench --bench bench_nanos --all-features
# cargo bench --bench bench_micros --all-features
# cargo bench --bench bench_millis --all-features

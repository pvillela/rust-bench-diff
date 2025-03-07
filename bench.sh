#!/bin/bash

RUSTFLAGS="-Awarnings" \
PARAMS_NAME="nanos_scale" \
FN_NAME_PAIRS="all" \
VERBOSE="true" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- 10

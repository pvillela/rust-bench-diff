#!/bin/bash

# Environment variables and their defaults:
#
# SCALE_NAME="micros_scale"
# FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var"
# VERBOSE="true"
# NOISE_STATS="true"

# Command line arguments and their defaults:
#
# $1 = 1  # nrepeats
# $2 = "" # run_name

export RUSTFLAGS="-Awarnings"
export VERBOSE="false"
export NOISE_STATS="false"

FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var hi_1pct_median_no_var/base_median_no_var" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 default

FN_NAME_PAIRS="base_median_no_var/base_median_no_var" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 base-no-var/base-no-var

FN_NAME_PAIRS="base_median_no_var/hi_1pct_median_no_var" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 base-no-var/hi-1pct-no-var

FN_NAME_PAIRS="hi_1pct_median_no_var/base_median_no_var" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 hi-1pct-no-var/base-no-var

FN_NAME_PAIRS="all" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 all

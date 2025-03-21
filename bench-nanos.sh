#!/bin/bash

# Environment variables and their defaults:
#
# SCALE_NAME="micros_scale"
# FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var"
# VERBOSE="true"

# Command line arguments and their defaults:
#
# $1 = 1  # nrepeats
# $2 = "" # run_name

RUSTFLAGS="-Awarnings" \
SCALE_NAME="nanos_scale" \
FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var hi_1pct_median_no_var/base_median_no_var" \
VERBOSE="false" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 default

RUSTFLAGS="-Awarnings" \
SCALE_NAME="nanos_scale" \
FN_NAME_PAIRS="base_median_no_var/base_median_no_var" \
VERBOSE="false" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 base-no-var/base-no-var

RUSTFLAGS="-Awarnings" \
SCALE_NAME="nanos_scale" \
FN_NAME_PAIRS="base_median_no_var/hi_1pct_median_no_var" \
VERBOSE="false" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 base-no-var/hi-1pct-no-var

RUSTFLAGS="-Awarnings" \
SCALE_NAME="nanos_scale" \
FN_NAME_PAIRS="hi_1pct_median_no_var/base_median_no_var" \
VERBOSE="false" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 hi-1pct-no-var/base-no-var

RUSTFLAGS="-Awarnings" \
SCALE_NAME="nanos_scale" \
FN_NAME_PAIRS="all" \
VERBOSE="false" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1 all

# FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var hi_1pct_median_no_var/base_median_no_var" \

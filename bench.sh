#!/bin/bash

# Environment variables and their defaults:
#
# PARAMS_NAME="micros_scale"
# FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_median_no_var"
# VERBOSE="true"

# Command line arguments:
#
# $1 = 1 # nrepeats


RUSTFLAGS="-Awarnings" \
PARAMS_NAME="micros_scale" \
FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_median_no_var hi_median_no_var/base_median_no_var" \
VERBOSE="true" \
cargo bench --bench bench_t --all-features --target-dir target/bench-target -- $1

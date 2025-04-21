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

echo "Started at: `date +"%H:%M:%S"`"

# FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/hi_1pct_median_no_var hi_1pct_median_no_var/base_median_no_var" \
# cargo bench --bench main_bench --features bench --target-dir target/bench-target -- $1 default

# FN_NAME_PAIRS="base_median_no_var/base_median_no_var" \
# cargo bench --bench main_bench --features bench --target-dir target/bench-target -- $1 base-no-var/base-no-var

# FN_NAME_PAIRS="base_median_no_var/hi_1pct_median_no_var" \
# cargo bench --bench main_bench --features bench --target-dir target/bench-target -- $1 base-no-var/hi-1pct-no-var

# FN_NAME_PAIRS="hi_1pct_median_no_var/base_median_no_var" \
# cargo bench --bench main_bench --features bench --target-dir target/bench-target -- $1 hi-1pct-no-var/base-no-var

cargo bench --bench main_bench --features bench --target-dir target/bench-target -- $1 $2

echo ""
echo "Finished at: `date +"%H:%M:%S"`"


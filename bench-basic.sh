#!/bin/bash

export RUSTFLAGS="-Awarnings"

# $1: number of repetitions 
# $2: TARGET_RELATIVE_DIFF_PCT
export TARGET_RELATIVE_DIFF_PCT="$2"

# Default:
# LATENCY_UNIT="nano" \
# BASE_MEDIAN=100000 \
# export EXEC_COUNT=2000 \

# Suggested for "micro" magnitude
# LATENCY_UNIT="micro" \
# BASE_MEDIAN=20000 \
# EXEC_COUNT=200 \

for ((i=1; i<=$1; i++)); do
    echo "*** i=$i ***" >&2
    echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
    echo
    cargo bench --bench basic_bench --features bench --target-dir target/bench-target
    echo
done


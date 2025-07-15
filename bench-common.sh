#!/bin/bash

# Environment variables and their defaults:
#
# SCALE_NAME="micros_scale"
# BENCH_MODE="diff" 
# FN_NAME_PAIRS=<panic>
# VERBOSE="false"

# TO_FILE="true"

# Command line arguments and their defaults:
#
# $1 = 1  # nrepeats
# $2 = "" # run_name

export RUSTFLAGS="-Awarnings"

output_target="/dev/stdout" # Default to stdout

if [[ -z "$TO_FILE" || "${TO_FILE,,}" == "true" ]]; then
    timestamp=$(date +"%Y%m%d_%H%M")
    bench_mode=${BENCH_MODE-"diff"}
    verb=""
    if [[ "${VERBOSE,,}" == "true" ]]; then
        verb="-verb"
    fi
    output_target="out/${bench_mode}-${SCALE_NAME}-${timestamp}${verb}.txt"
fi

echo "Started at: `date +"%H:%M:%S"`, output_target=$output_target" > $output_target

cargo bench --bench main_bench --features _bench --target-dir target/bench-target -- $1 $2 >> $output_target

echo "" >> $output_target
echo "Finished at: `date +"%H:%M:%S"`" >> $output_target


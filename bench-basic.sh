#!/bin/bash

export RUSTFLAGS="-Awarnings"

# $1: number of repetitions 
# $2: TARGET_RELATIVE_DIFF_PCT
export TARGET_RELATIVE_DIFF_PCT="$2"

# Default:
# LATENCY_UNIT="nano" \
# BASE_MEDIAN=100000 \
# export EXEC_COUNT=2000 \

# Suggested for "milli" magnitude
# LATENCY_UNIT="micro" \
# BASE_MEDIAN=20000 \
# EXEC_COUNT=200 \

echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
echo

for ((i=1; i<=$1; i++)); do
    echo "*** i=$i ***" | tee /dev/stderr
    cargo bench --bench basic_bench --features _bench --target-dir target/bench-target
done

echo ""
echo "Finished at: `date +"%H:%M:%S"`"



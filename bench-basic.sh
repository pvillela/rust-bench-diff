#!/bin/bash

export RUSTFLAGS="-Awarnings"
export TARGET_RELATIVE_DIFF_PCT="$2"

# export LATENCY_UNIT="micro"
# export BASE_MEDIAN=20000
# export EXEC_COUNT=200

for ((i=1; i<=$1; i++)); do
    echo "*** i=$i ***" >&2
    echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
    echo
    cargo bench --bench basic_bench --features bench --target-dir target/bench-target
    echo
done


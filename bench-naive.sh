#!/bin/bash

export RUSTFLAGS="-Awarnings"
export TARGET_RELATIVE_DIFF_PCT="$2"

for ((i=1; i<=$1; i++)); do
    echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
    echo
    cargo bench --bench naive_bench --features bench --target-dir target/bench-target
    echo
done


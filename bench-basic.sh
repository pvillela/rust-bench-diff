#!/bin/bash

export RUSTFLAGS="-Awarnings"

for ((i=1; i<=$1; i++)); do
    echo "----- Started: `date +"%Y-%m-%d at %H:%M:%S"` -----"
    echo
    cargo bench --bench basic_bench --features bench --target-dir target/bench-target
    echo
done


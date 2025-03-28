#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo bench --bench naive_bench --features bench --target-dir target/bench-target


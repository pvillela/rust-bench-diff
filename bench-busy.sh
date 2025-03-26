#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo bench --bench busy_bench --features dev_utils --target-dir target/bench-target


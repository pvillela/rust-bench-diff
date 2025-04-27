#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo bench --bench busy_bench --features _bench --target-dir target/bench-target


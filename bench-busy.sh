#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo bench --bench busy_bench --features _dev_utils --target-dir target/bench-target


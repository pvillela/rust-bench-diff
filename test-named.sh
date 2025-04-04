#!/bin/bash

export RUSTFLAGS="-Awarnings"

# Below runs any tests with names containing 'hypors'.
cargo nextest run $1 --lib --bins --tests --features dev_support --target-dir target/test-target

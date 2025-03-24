#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo nextest run --lib --bins --examples --tests --all-features --target-dir target/test-target
# cargo test --doc --all-features

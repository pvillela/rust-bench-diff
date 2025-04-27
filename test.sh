#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo nextest run --lib --bins --examples --tests --features _test_support --target-dir target/test-target
cargo test --doc --features _test_support

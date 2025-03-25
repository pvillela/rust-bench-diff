#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo nextest run --lib --bins --examples --tests --features test_support --target-dir target/test-target
cargo test --doc --features test_support

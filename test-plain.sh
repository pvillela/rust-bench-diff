#!/bin/bash

########
# Only tests src, with no features enabled.

export RUSTFLAGS="-Awarnings"

cargo nextest run --lib --bins --tests --target-dir target/test-target
cargo test --doc

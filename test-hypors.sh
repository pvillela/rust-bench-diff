#!/bin/bash

export RUSTFLAGS="-Awarnings"

# Below runs any tests with names containing 'hypors'.
cargo nextest run hypors --features _hypors --target-dir target/test-target

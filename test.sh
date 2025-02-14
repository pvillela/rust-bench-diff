#!/bin/bash

cargo nextest run --lib --bins --examples --tests --all-features
echo "*** Benchmark ***"
./bench.sh
cargo test --doc --all-features

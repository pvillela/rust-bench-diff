#!/bin/bash

# NOCOVER environment variable enables tests that are excluded from test coverage measurement.
export NOCOVER="1" 

echo "*****  --lib --bins --tests (default feature)"
cargo nextest run --lib --bins --tests --features _test_support --target-dir target/test-target

# Can't run without default features.
# echo "***** --no-default-features"
# cargo nextest run --lib --bins --tests --no-default-features --features _test_support --target-dir target/test-target

echo "***** doc"
cargo test --doc

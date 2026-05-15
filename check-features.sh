#!/bin/bash

set -e  # Stop script immediately on any error

### With default features: Externally exposed feature combinations

echo "***** (default feature)"
cargo check --lib --tests --target-dir target/test-target

### All targets and features

echo "***** --all-targets --all-features"
cargo check --all-targets --all-features --target-dir target/test-target

### Benches

echo "***** --features _bench"
cargo check --lib --tests --benches --features _bench --target-dir target/test-target

### Without default features: Externally exposed feature combinations

# Can't run without default features.

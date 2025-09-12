#!/bin/bash

echo "***** --all-targets --all-features"
cargo check --all-targets --all-features

echo "***** --lib --tests (default feature)"
cargo check --lib --tests

# Can't run withoug default features.
# echo "***** --no-default-features"
# cargo check --lib --tests --no-default-features

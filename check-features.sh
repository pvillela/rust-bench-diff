#!/bin/bash

echo "***** --all-targets --all-features"
cargo check --all-targets --all-features

echo "***** --lib --bins --tests (default feature)"
cargo check --lib --bins --tests

# Can't run withoug default features.
# echo "***** --no-default-features"
# cargo check --lib --bins --tests --no-default-features

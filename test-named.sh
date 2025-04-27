#!/bin/bash

export RUSTFLAGS="-Awarnings"

cargo nextest run $1 --lib --bins --tests --features _test_support --target-dir target/test-target

# Below runs any tests with names containing 'student':
# ./test-named student

#!/bin/bash

cargo bench --bench bench_nanos --all-features
cargo bench --bench bench_micros --all-features
cargo bench --bench bench_millis --all-features

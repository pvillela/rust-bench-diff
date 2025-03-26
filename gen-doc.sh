#!/bin/bash

rm -r target/doc

cargo makedocs -e hypors -e polars -e rand -e rand_distr -e sha2 -e statrs
cargo doc -p bench_diff --no-deps

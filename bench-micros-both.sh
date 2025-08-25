#!/bin/bash

BENCH_MODE=comp ./bench-micros.sh $1 comp

BENCH_MODE=diff ./bench-micros.sh $1 diff

#!/bin/bash

export FN_NAME_PAIRS="1@novar/1@novar | 1.01@novar/1@novar | 1.05@novar/1@novar | 1.1@novar/1@novar | 1.25@novar/1@novar"

VERBOSE=true BENCH_MODE=comp ./bench-micros.sh $1 "comp-only-novar"

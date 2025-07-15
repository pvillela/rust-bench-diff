#!/bin/bash

export FN_NAME_PAIRS="1@novar/1@novar | 1.01@novar/1@novar | 1.05@novar/1@novar | 1.1@novar/1@novar | 1.25@novar/1@novar"

BENCH_MODE=diff ./bench-micros.sh $1 "both-novar:diff"

BENCH_MODE=comp ./bench-micros.sh $1 "both-novar:comp"
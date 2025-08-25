#!/bin/bash

export FN_NAME_PAIRS="1@lovar/1@lovar | 1.01@lovar/1@lovar | 1.05@lovar/1@lovar | 1.1@lovar/1@lovar | 1.25@lovar/1@lovar"

BENCH_MODE=comp ./bench-micros.sh $1 "both-lovar:comp"

BENCH_MODE=diff ./bench-micros.sh $1 "both-lovar:diff"

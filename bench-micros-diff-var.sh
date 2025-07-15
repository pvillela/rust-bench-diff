#!/bin/bash

export FN_NAME_PAIRS="1@lovar/1@novar | 1@hivar/1@novar | 1.01@lovar/1@novar | 1.01@hivar/1@novar | 1.05@lovar/1@novar | 1.05@hivar/1@novar | 1.1@lovar/1@novar | 1.1@hivar/1@novar | 1.25@lovar/1@novar | 1.25@hivar/1@novar"

BENCH_MODE=diff ./bench-micros.sh $1 "diff-only-var"

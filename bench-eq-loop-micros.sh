#!/bin/bash

export FN_NAME_PAIRS="base_median_no_var/base_median_no_var"
export SCALE_NAME="micros_scale"


for ((i=1; i<=$3; i++)); do
    echo "=== i=$i ===" | tee /dev/stderr
    ./bench-common.sh $1 $2
done

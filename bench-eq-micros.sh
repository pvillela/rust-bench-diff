#!/bin/bash

export FN_NAME_PAIRS="base_median_no_var/base_median_no_var base_median_no_var/base_median_no_var"
export SCALE_NAME="micros_scale"

./bench-common.sh $1 $2

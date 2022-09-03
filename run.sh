#!/bin/bash

CUR_DIR=$(dirname $(readlink -f "$0"))
FFI_DIR=$CUR_DIR/ffi
RUN_DIR=$CUR_DIR/cryptoinfo

$FFI_DIR/build.sh
cd $RUN_DIR && make run


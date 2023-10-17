#!/bin/bash
LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")

export LD_LIBRARY_PATH=$DIR/lib:$LD_LIBRARY_PATH

RUST_LOG=error,warn,reqwest=off,rocket=off $DIR/cryptoinfo

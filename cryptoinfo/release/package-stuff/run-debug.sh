#!/bin/bash
LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")

export LD_LIBRARY_PATH=$DIR/lib:$LD_LIBRARY_PATH

ROCKET_ENV=production RUST_LOG=error,warn,info,debug,reqwest=off,rocket=off $DIR/AppRun

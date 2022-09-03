#!/bin/bash

CUR_DIR=$(dirname $(readlink -f "$0"))
LIB_DIR=$CUR_DIR/lib
INCLUDE_DIR=$CUR_DIR/include

if [ ! -d $LIB_DIR ]; then
    mkdir $LIB_DIR
fi

if [ ! -d $INCLUDE_DIR ]; then
    mkdir $INCLUDE_DIR
fi

c_libs=('foo-c')
for lib in ${c_libs[*]}
do
    cd $CUR_DIR/src/$lib && make install
done

cpp_libs=('bar-cpp')
for lib in ${cpp_libs[*]}
do
    cd $CUR_DIR/src/$lib && ./build.sh
done



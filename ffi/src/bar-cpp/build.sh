#!/bin/bash

TARGET_LIB="libbar.so"
CUR_DIR=$(dirname $(readlink -f "$0"))
INSTALL_DIR=$CUR_DIR/../../lib
BUILD_DIR=$CUR_DIR/build
INCLUDE_DIR=$CUR_DIR/../../include/bar

if [ ! -d $BUILD_DIR ]; then
    mkdir $BUILD_DIR
fi
cd $BUILD_DIR && cmake .. && make

if [ ! -f $TARGET_LIB ]; then
    echo "can't find $TARGET_LIB"
    cd $CUR_DIR && rm -rf $BUILD_DIR
    exit -1;
fi

if [ ! -d $INCLUDE_DIR ]; then
    mkdir $INCLUDE_DIR
fi
cp -f $CUR_DIR/*.h $INCLUDE_DIR

mv -f $TARGET_LIB $INSTALL_DIR && rm -rf $BUILD_DIR
echo "Install $TARGET_LIB successfully!"



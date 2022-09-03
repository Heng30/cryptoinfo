#!/bin/bash

TARGET_LIB="libcusimageplugin.so"
CUR_DIR=$(dirname $(readlink -f "$0"))
INSTALL_DIR=$CUR_DIR/../../qml/CusImage
BUILD_DIR=$CUR_DIR/build
OUT_DIR=$CUR_DIR/build/out

if [ ! -d $BUILD_DIR ]; then
    mkdir $BUILD_DIR
fi

if [ ! -d $INSTALL_DIR ]; then
    mkdir $INSTALL_DIR
fi

qmake cusimage.pro && make
cp -f $OUT_DIR/$TARGET_LIB $INSTALL_DIR
cp -f $CUR_DIR/qmldir $INSTALL_DIR

make clean && rm -rf $BUILD_DIR rm -f Makefile


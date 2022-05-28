#!/bin/bash

APP="cryptoinfo"
ROOT=$(dirname $(readlink -f "$0"))
APP_DIR=${ROOT}/release/${APP}
PKG_DIR="${ROOT}/release/package"

if [ ! -f "$APP_DIR" ];
then
    echo "$APP_DIR don't exist"
fi

if [ ! -d "$PKG_DIR" ];
then
    mkdir -p $PKG_DIR
fi

rm -r $PKG_DIR/lib*
rm -r $PKG_DIR/$APP
cp -rf $APP_DIR $PKG_DIR

arr=`ldd $APP_DIR | awk '{print $3}'`
arr2=()
for i in ${arr[*]}
do
    arr2[${#arr2[*]}]=$i
done

i=0
len=${#arr2[*]}
while [ $i -lt $len ]
do
    item=${arr2[$i]};
    libname=`basename $item`
    if [ ! -f "${PKG_DIR}/${libname}" ];
    then
        echo "cp $item  -> ${PKG_DIR}/${libname}"
        cp -L $item $PKG_DIR

        arr_tmp=`ldd $item | awk '{print $3}'`
        for j in ${arr_tmp[*]}
        do
            arr2[$len]=$j
            len=$(( $len+1 ))
        done
    fi

    i=$(( $i+1 ))
done


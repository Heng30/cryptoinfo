#!/bin/bash

APP="cryptoinfo"
ROOT=$(dirname $(readlink -f "$0"))
APP_DIR=${ROOT}/release/${APP}
PKG_DIR="${ROOT}/release/$APP-linux/lib"

if [ ! -f "$APP_DIR" ];
then
    echo "$APP_DIR don't exist"
fi

if [ ! -d "$PKG_DIR" ];
then
    mkdir -p $PKG_DIR
fi

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
    i=$(( $i+1 ))
    libname=`basename $item`

    name=`echo $libname | awk -F. '{print $1}'`
    if [ "${name}" == "libpthread" ]; then
        continue
    fi

    if [ "${name}" == "libc" ]; then
        continue
    fi

    if [ "${name}" == "libgcc_s" ]; then
        continue
    fi

    if [ "${name}" == "libdl" ]; then
        continue
    fi

    if [ "${name}" == "librt" ]; then
        continue
    fi

    if [ ! -f "${PKG_DIR}/${libname}" ];
    then
        cp -L $item $PKG_DIR

        arr_tmp=`ldd $item | awk '{print $3}'`
        for j in ${arr_tmp[*]}
        do
            arr2[$len]=$j
            len=$(( $len+1 ))
        done
    fi
done


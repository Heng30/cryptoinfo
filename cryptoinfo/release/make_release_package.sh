#!/bin/bash

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
echo $DIR
cd $DIR

if [ -d cryptoinfo-linux.tar.gz ]; then
    rm cryptoinfo-linux.tar.gz
fi

cp -rf package cryptoinfo-linux
sync
tar -zcvf cryptoinfo-linux.tar.gz cryptoinfo-linux

rm -rf cryptoinfo-linux

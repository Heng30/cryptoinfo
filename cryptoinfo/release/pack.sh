#!/bin/bash

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
cd $DIR

./set_pack_env.sh
if [ -d package ]; then
    rm -rf package
fi

mkdir package
cd package
cp ../package-stuff/cryptoinfo.desktop ../package-stuff/icon.png ./
cp ../cryptoinfo ./
linuxdeployqt cryptoinfo -appimage -unsupported-allow-new-glibc
rm -rf ./translations
cp -rf ../package-stuff/* ./
exit


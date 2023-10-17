#!/bin/bash

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
cd $DIR

src_dir="cryptoinfo-linux"
dst_dir="/opt/cryptoinfo"
desktop_dir=$HOME/.local/share/applications
app_desktop="cryptoinfo.desktop"
app_data_dir=$HOME/.local/share/cryptoinfo

for item in $app_desktop run-debug.sh run.sh AppRun uninstall.sh; do
    chmod a+x $item
done

cp -f $app_desktop $HOME/Desktop
cp -f $app_desktop $desktop_dir

cd ..

if [ ! -d $app_data_dir ]; then
    mkdir -p $app_data_dir
fi

if [ ! -d $dst_dir ]; then
    sudo mkdir -p $dst_dir
fi

sudo mv $src_dir/* $dst_dir
rm -rf $src_dir

echo "Install successfully!!!"

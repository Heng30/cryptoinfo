#!/bin/bash

dst_dir="/opt/cryptoinfo"
desktop_dir=$HOME/.local/share/applications
app_desktop="cryptoinfo.desktop"

rm -f $desktop_dir/$app_desktop
rm -f $HOME/Desktop/$app_desktop
sudo rm -rf $dst_dir

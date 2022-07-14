#!/bin/bash

dst_dir="/opt/cryptoinfo"
desktop_dir=$HOME/.local/share/applications
app_desktop="cryptoinfo.desktop"
app_data_web_dir=$HOME/.local/share/cryptoinfo/webserver

rm -f $desktop_dir/$app_desktop
rm -f $HOME/Desktop/$app_desktop
rm -rf $app_data_web_dir
sudo rm -rf $dst_dir

echo "Uninstall successfully!!!"

#!/bin/bash

ROOT=$(dirname $(readlink -f "$0"))
export LD_LIBRARY_PATH=$ROOT #:$LD_LIBRARY_PATH
export QT_PLUGIN_PATH=$ROOT/plugins #:$QT_PLUGIN_PATH
export QML2_PATH=$ROOT/qml:$QML2_PATH
APP="cryptoinfo"
ldd $APP
${ROOT}/${APP}
exit


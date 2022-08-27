#!/bin/bash

export PATH=/usr/lib:$PATH
export LIB_PATH=/usr/lib
export PLUGIN_PATH=/usr/lib/qt/plugins
export QML2_PATH=/usr/lib/qt/qml

SYS_LIB_PATH=/usr/lib
QT_LIB_PATH=/usr/lib

LOC=$(readlink -f "$0")
DIR=$(dirname "$LOC")
cd $DIR/release

target="cryptoinfo"
output_dir="$target-linux"
stuff_dir="package-stuff"
target_package="$output_dir.tar.gz"
target_package_header="bin-header.sh"

if [ ! -f $target ]; then
    echo "Can not find $target"
    exit -1
fi

rm -rf $output_dir
mkdir $output_dir
cp -rf $stuff_dir/* $output_dir
cp $target $output_dir

$DIR/cp_libs.sh

# 打包库到指定文件夹
# cd $output_dir
# linuxdeployqt $target -appimage -unsupported-allow-new-glibc
# cd ..

# 复制qml相关库和文件
package_lib="$output_dir/lib"
package_qml="$output_dir/qml"
package_plugins="$output_dir/plugins"

for dir in $package_lib $package_qml $package_plugins; do
    if [ ! -d $dir ]; then
       mkdir -p $dir
    fi
done

sys_libs=('libFcitx5Qt5DBusAddons.so')
for lib in "${sys_libs[@]}"; do
    lib=$SYS_LIB_PATH/$lib
    if [ ! -f $lib ]; then
        echo "Can not find $lib, please install corren lib"
        exit -1
    fi

    cp -rL $lib $package_lib
done

qt_libs=('libQt5Multimedia.so.5' 'libQt5QuickTemplates2.so.5' 'libQt5Charts.so.5' 'libQt5MultimediaQuick.so.5' 'libQt5QmlWorkerScript.so.5' 'libQt5QuickControls2.so.5' 'libQt5QuickShapes.so.5' 'libQt5DBus.so.5' 'libQt5Pdf.so.5' 'libQt5Svg.so.5' 'libQt5XcbQpa.so.5')
for lib in "${qt_libs[@]}"; do
    lib=$QT_LIB_PATH/$lib
    if [ ! -f $lib ]; then
        echo "Can not Find $lib, please install correct lib"
        exit -1
    fi

    cp -rL $lib $package_lib
done

qmls=('QtQml' 'QtCharts' 'QtMultimedia' 'QtQuick' 'QtQuick.2' 'QtGraphicalEffects')
for qml in "${qmls[@]}"; do
    qml=$QML2_PATH/$qml
    if [ ! -d $qml ]; then
        echo "Can not find $qml, please install corren qml module"
        exit -1
    fi

    cp -rf $qml $package_qml
done

plugins=('bearer' 'iconengines' 'imageformats' 'platforminputcontexts' 'platforms' 'xcbglintegrations')
for plugin in "${plugins[@]}"; do
    plugin=$PLUGIN_PATH/$plugin
    if [ ! -d $plugin ]; then
        echo "Can not find $plugin, please install corren qt plugin"
        exit -1
    fi

    cp -rf $plugin $package_plugins
done


cp -rf $DIR/../web $output_dir/webserver

# 制作tar.gz包
rm -f $target_package
tar -zcvf $target_package $output_dir > /dev/null

for item in $target_package_header $target_package; do
    if [ ! -f $item ]; then
        echo "Can not find $item"
        exit -1
    fi
done

# 制作run包
rm -f $output_dir-*.run
cat $target_package_header $target_package > $output_dir.run
chmod a+x $output_dir.run

md5=`md5sum $output_dir.run | awk '{print $1}'`
md5=`echo ${md5:0:6}`
version=`git tag | tail -n 1`
machine=`uname -m`
mv $output_dir.run $output_dir-$version-$machine-$md5.run

# 删除中间文件
rm -f $target_package
rm -rf $output_dir

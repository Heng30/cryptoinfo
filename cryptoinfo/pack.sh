#!/bin/bash

export PATH=/usr/lib:$PATH
export LIB_PATH=/usr/lib
export PLUGIN_PATH=/usr/lib/qt/plugins
export QML2_PATH=/usr/lib/qt/qml

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
mv $target $output_dir

# 打包库到指定文件夹
cd $output_dir
linuxdeployqt $target -appimage -unsupported-allow-new-glibc

# 复制qml相关库和文件
cd ..
package_lib="$output_dir/lib"
package_qml="$output_dir/qml"

for dir in $package_lib $package_qml; do
    if [ ! -d $dir ]; then
       mkdir -p $dir
    fi
done

libs=('libQt5Multimedia.so.5' 'libQt5QuickTemplates2.so.5' 'libQt5Charts.so.5' 'libQt5MultimediaQuick.so.5' 'libQt5QmlWorkerScript.so.5' 'libQt5QuickControls2.so.5' 'libQt5QuickShapes.so.5')
for lib in "${libs[@]}"; do
    lib=$LIB_PATH/$lib
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
rm -f $output_dir.run
cat $target_package_header $target_package > $output_dir.run

# 删除中间文件
rm -f $target_package
rm -rf $output_dir

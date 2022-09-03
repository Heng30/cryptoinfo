TEMPLATE = lib
QT += quick qml
CONFIG += c++17 plugin qmltypes

QML_IMPORT_NAME = CusImage
QML_IMPORT_MAJOR_VERSION = 1

TARGET = $$qtLibraryTarget(cusimageplugin)

MOC_DIR = $${PWD}/build/moc
RCC_DIR = $${PWD}/build/rcc
UI_DIR = $${PWD}/build/ui
OBJECTS_DIR = $${PWD}/build/objects
DESTDIR = $${PWD}/build/out

HEADERS += cusimage.h cusimageplugin.h

SOURCES += cusimage.cpp

OTHER_FILES += qmldir

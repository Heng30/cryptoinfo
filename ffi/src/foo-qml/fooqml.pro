TEMPLATE = lib
QT += quick qml
CONFIG += c++17 plugin qmltypes

QML_IMPORT_NAME = FooQml
QML_IMPORT_MAJOR_VERSION = 1

TARGET = $$qtLibraryTarget(fooqmlplugin)

MOC_DIR = $${PWD}/build/moc
RCC_DIR = $${PWD}/build/rcc
UI_DIR = $${PWD}/build/ui
OBJECTS_DIR = $${PWD}/build/objects
DESTDIR = $${PWD}/build/out

HEADERS += fooqml.h fooqmlplugin.h

SOURCES += fooqml.cpp

OTHER_FILES += qmldir

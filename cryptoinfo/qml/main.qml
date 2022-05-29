import QtQml 2.15
import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Window {
    id: main

    property alias theme: theme

    x: Screen.desktopAvailableWidth / 2 - width / 2
    y: Screen.desktopAvailableHeight / 2 - height / 2
    width: splash.visible ? splash.width : window.width
    height: splash.visible ? splash.height : window.height
    flags: Qt.Dialog | Qt.FramelessWindowHint | Qt.NoDropShadowWindowHint
    color: "transparent"
    title: "cryptoinfo"
    visible: true

    Base.Theme {
        id: theme
    }

    Splash {
        id: splash
        visible: true
    }

    MainWindow {
        id: window

        visible: !splash.visible
    }

}

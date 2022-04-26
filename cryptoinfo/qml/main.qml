import QtQml 2.15
import QtQuick 2.15
import QtQuick.Window 2.15

Window {
    id: main

    property bool _isPowerOn: true
    property alias theme: mainWindow.theme

    x: theme.splashStartupX
    y: theme.splashStartupY
    width: theme.splashWitdh
    height: theme.splashHeight
    color: "transparent"
    visible: _isPowerOn
    flags: Qt.Dialog | Qt.FramelessWindowHint | Qt.NoDropShadowWindowHint

    Splash {
        id: splash

        visible: _isPowerOn
    }

    MainWindow {
        id: mainWindow

        visible: !splash.visible
    }
}

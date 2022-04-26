import QtQuick 2.12
import QtQuick.Controls 2.12

ProgressBar {
    id: root

    property color bgColor: theme.itemBgColor
    property color valueBarColor: theme.invertBgColor

    background: Rectangle {
        width: root.width
        height: root.height
        color: root.bgColor
        radius: height / 2
    }

    contentItem: Rectangle {
        width: root.visualPosition * root.width
        height: root.height
        radius: height / 2
        color: root.valueBarColor
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15

ToolTip {
    id: root

    property color bgColor: theme.bgColor
    property color borderColor: theme.invertBgColor
    property color textColor: theme.fontColor
    property real textFontPixelSize: theme.fontPixelNormal

    background: Rectangle {
        color: bgColor
        border.width: 1
        border.color: borderColor
    }

    contentItem: Label {
        text: root.text
        color: root.textColor
        font.pixelSize: root.textFontPixelSize
    }

}

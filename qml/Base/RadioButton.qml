import QtQuick 2.15
import QtQuick.Controls 2.15

RadioButton {
    id: root

    property color checkedColor: theme.invertBgColor
    property color textColor: theme.fontColor
    property real textPixelSize: theme.fontPixelNormal

    leftPadding: theme.itemPadding

    contentItem: Text {
        text: root.text
        color: root.textColor
        font.pixelSize: root.textPixelSize
        leftPadding: root.indicator.width + root.spacing
        verticalAlignment: Text.AlignVCenter
    }

    indicator: Rectangle {
        x: root.leftPadding
        anchors.verticalCenter: parent.verticalCenter
        width: root.height / 2
        height: width
        radius: width / 2
        color: "transparent"
        border.width: 2
        border.color: root.checkedColor
        antialiasing: true

        Rectangle {
            id: checker

            anchors.centerIn: parent
            width: parent.width - parent.border.width * 2 - 4
            height: width
            antialiasing: true
            radius: width / 2
            color: root.checkedColor
            visible: root.checked
        }

    }

}

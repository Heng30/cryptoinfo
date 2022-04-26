import QtQuick 2.15
import QtQuick.Controls 2.15

Switch {
    id: root

    property color checkedColor: "transparent"
    property color unCheckedColor: "transparent"
    property color textColor: theme.fontColor
    property color indicatorBorderColor: theme.borderColor
    property color switchButtonColor: theme.switchButtonColor
    property real textPixelSize: theme.fontPixelNormal
    property real indicatorWidth: textPixelSize * 3
    property real indicatorRadius: 0

    leftPadding: theme.itemPadding

    contentItem: Text {
        id: txt

        text: root.text
        color: root.textColor
        font.pixelSize: root.textPixelSize
        leftPadding: root.indicator.width + root.spacing
        verticalAlignment: Text.AlignVCenter
    }

    indicator: Rectangle {
        x: root.leftPadding
        anchors.verticalCenter: parent.verticalCenter
        width: root.indicatorWidth
        height: txt.height
        color: root.checked ? root.checkedColor : root.unCheckedColor
        border.width: 1
        border.color: root.indicatorBorderColor
        antialiasing: true
        radius: root.indicatorRadius

        Rectangle {
            x: root.checked ? width + parent.border.width : parent.border.width
            y: parent.border.width
            anchors.verticalCenter: parent.verticalCenter
            width: parent.width / 2 - parent.border.width
            height: parent.height - parent.border.width * 2
            antialiasing: true
            color: root.switchButtonColor
            radius: root.indicatorRadius
        }

    }

}

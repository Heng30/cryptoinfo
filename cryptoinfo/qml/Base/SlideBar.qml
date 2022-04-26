import QtQuick 2.15
import QtQuick.Controls 2.15

Slider {
    id: root

    property color bgColor: theme.itemBgColor
    property color valueBarColor: theme.invertBgColor
    property color handlePressedColor: theme.itemEnteredBG
    property color handleReleaseColor: theme.itemEnxitedBG
    property bool showValue: false
    property alias tipText: tip.text

    implicitWidth: 200
    implicitHeight: 20

    Tip {
        id: tip
        parent: root.handle
        visible: root.pressed && root.showValue
        text: root.value
    }

    background: Rectangle {
        id: recBG

        x: root.leftPadding
        y: root.topPadding + root.availableHeight / 2 - height / 2
        width: root.availableWidth
        height: Math.max(root.height - 16, 4)
        radius: height / 2
        color: root.bgColor

        Rectangle {
            width: root.visualPosition * parent.width
            height: parent.height
            color: root.valueBarColor
            radius: height / 2
        }

    }

    handle: Rectangle {
        x: root.leftPadding + root.visualPosition * (root.availableWidth - width)
        y: root.topPadding + root.availableHeight / 2 - height / 2
        width: recBG.height + 16
        height: width
        radius: width / 2
        color: root.pressed ? root.handlePressedColor : root.handleReleaseColor
    }

}

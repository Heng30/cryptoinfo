import QtQuick 2.15
import QtQuick.Controls 2.15

Dial {
    id: root

    property real handleHeight: 8
    property color bgColor: "transparent"
    property color borderColor: theme.borderColor
    property color handleColor: theme.invertBgColor

    implicitWidth: 300
    implicitHeight: implicitWidth
    rotation: 90

    background: Rectangle {
        x: root.width / 2 - width / 2
        y: root.height / 2 - height / 2
        width: Math.max(4, Math.min(root.width, root.height))
        height: width
        color: root.bgColor
        radius: width / 2
        border.color: root.borderColor
    }

    handle: Rectangle {
        id: handleItem

        x: root.background.x + root.background.width / 2 - width
        y: root.background.y + root.background.height / 2 - height / 2
        width: root.width / 5 * 2
        height: root.handleHeight
        color: root.handleColor
        radius: height / 2
        antialiasing: true
        transform: [
            Rotation {
                angle: root.angle
                origin.x: handleItem.width
                origin.y: handleItem.height / 2
            }
        ]
    }

}

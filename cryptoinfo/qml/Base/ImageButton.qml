import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15

Rectangle {
    id: root

    property alias source: image.source
    property alias tipText: tip.text
    property bool checked: false
    property bool enableColorOverlay: true
    property bool enableMouseArea: true

    signal clicked(var mouse)

    implicitWidth: 32
    implicitHeight: 32
    color: "transparent"

    Image {
        id: image

        property bool _entered: false

        width: Math.min(parent.width, parent.height)
        height: width
        anchors.centerIn: parent
        fillMode: Image.PreserveAspectFit

        ColorOverlay {
            visible: root.enableColorOverlay
            anchors.fill: parent
            source: parent
            color: theme.imageColor
        }

        Rectangle {
            anchors.fill: parent
            color: parent._entered || root.checked ? theme.imageEnteredColor : "transparent"
            radius: width / 3
            opacity: root.checked ? 0.2 : 0.3
        }

    }

    Tip {
        id: tip

        visible: image._entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onClicked: root.clicked(mouse)
        onEntered: image._entered = true
        onExited: image._entered = false
        visible: root.enableMouseArea
    }

}

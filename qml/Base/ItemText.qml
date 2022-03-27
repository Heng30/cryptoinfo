import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property alias text: label.text
    property alias textColor: label.color
    property alias textFontBold: label.font.bold
    property alias textFontPixelSize: label.font.pixelSize
    property alias tipText: tip.text

    signal clicked()

    color: "transparent"
    implicitWidth: label.width + 20
    implicitHeight: label.height + 20

    Label {
        id: label

        anchors.centerIn: parent
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
    }

    Tip {
        property bool _entered: false
        id: tip
        visible: _entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: tip._entered = true
        onExited: tip._entered = false
        onClicked: root.clicked()
    }

}

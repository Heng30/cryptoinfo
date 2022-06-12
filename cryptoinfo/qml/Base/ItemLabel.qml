import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property real textVMargins: theme.itemPadding * 2
    property alias text: label.text
    property alias textColor: label.color
    property alias textFontBold: label.font.bold
    property alias textFontPixelSize: label.font.pixelSize
    property alias leftPadding: label.leftPadding
    property alias tipText: tip.text
    property alias elide: label.elide
    property alias wrapMode: label.wrapMode
    property alias label: label

    signal clicked()

    color: "transparent"
    implicitWidth: label.width
    implicitHeight: label.height + textVMargins

    Label {
        id: label

        anchors.verticalCenter: parent.verticalCenter
        leftPadding: theme.itemPadding
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
        elide: Text.ElideRight
    }

    Tip {
        property bool _entered: false
        id: tip
        visible: _entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        onClicked: root.clicked()
        hoverEnabled: true
        onEntered: tip._entered = true
        onExited: tip._entered = false
    }

}

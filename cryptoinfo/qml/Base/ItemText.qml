import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property alias text: label.text
    property alias textColor: label.color
    property alias textFontBold: label.font.bold
    property alias textFontPixelSize: label.font.pixelSize
    property alias tipText: tip.text
    property real horizontalPadding: 0
    property real verticalMargins: theme.itemMargins * 4
    property real horizontalMargins: theme.itemMargins * 2
    property alias wrapMode: label.wrapMode
    property alias label: label
    property bool enableEnterCursorShape: false

    signal clicked()

    color: "transparent"
    implicitHeight: label.height + verticalMargins
    implicitWidth: label.width + horizontalMargins
    clip: true

    Label {
        id: label

        anchors.centerIn: parent
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
        elide: Text.ElideMiddle
    }

    Tip {
        id: tip

        property bool _entered: false

        visible: _entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: tip._entered = true
        onExited: tip._entered = false
        onClicked: root.clicked()
        cursorShape: (enableEnterCursorShape && tip._entered) ? Qt.PointingHandCursor : Qt.ArrowCursor
    }

}

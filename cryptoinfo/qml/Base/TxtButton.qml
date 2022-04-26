import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: txtBtn

    property bool showBorder: true
    property alias text: label.text
    property alias textWidth: label.width
    property alias textColor: label.color
    property alias textFontBold: label.font.bold
    property alias textFontPixelSize: label.font.pixelSize
    property alias tipText: tip.text
    property bool checked: false
    property bool entered: false
    property real horizontalPadding: 20
    property real verticalPadding: 20

    signal clicked()

    border.width: showBorder ? 1 : 0
    border.color: theme.borderColor
    radius: theme.itemRadius * 2
    color: checked || entered ? theme.itemEnteredBG : theme.bgColor
    implicitWidth: label.width + horizontalPadding
    implicitHeight: label.height + verticalPadding

    Label {
        id: label

        anchors.centerIn: parent
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
    }

    Tip {
        id: tip

        visible: txtBtn.entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: txtBtn.entered = true
        onExited: txtBtn.entered = false
        onClicked: txtBtn.clicked()
    }

}

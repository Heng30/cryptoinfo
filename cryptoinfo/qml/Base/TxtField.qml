import QtQuick 2.15
import QtQuick.Controls 2.15

TextField {
    id: txtField

    property bool showBorder: true
    property color bgColor: "transparent"
    property alias tipText: tip.text
    property real borderWidth: 1
    property bool showTip: false

    padding: 0
    color: theme.fontColor
    verticalAlignment: TextInput.AlignVCenter
    font.pixelSize: theme.fontPixelNormal
    selectByMouse: true
    clip: true

    Tip {
        id: tip
        visible: txtField.showTip && text.length > 0
    }

    background: Rectangle {
        anchors.fill: parent
        border.width: txtField.showBorder ? txtField.borderWidth : 0
        border.color: theme.borderColor
        color: txtField.bgColor
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15

TextField {
    property bool showBorder: true
    property color bgColor: "transparent"
    id: textField

    padding: 0
    color: theme.fontColor
    verticalAlignment: TextInput.AlignVCenter
    font.pixelSize: theme.fontPixelNormal
    clip: true

    background: Rectangle {
        anchors.fill: parent
        border.width: textField.showBorder ? 1 : 0
        border.color: theme.borderColor
        color: textField.bgColor
    }

}

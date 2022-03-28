import QtQuick 2.15
import QtQuick.Controls 2.15

TextField {
    id: textField

    padding: 0
    color: theme.fontColor
    verticalAlignment: TextInput.AlignVCenter
    font.pixelSize: theme.fontPixelNormal
    clip: true

    background: Rectangle {
        anchors.fill: parent
        border.width: 1
        border.color: theme.borderColor
        color: "transparent"
    }

}

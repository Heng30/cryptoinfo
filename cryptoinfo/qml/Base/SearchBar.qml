import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property alias text: textInput.text

    signal editingFinished()
    signal accepted()

    function forceFocus() {
        textInput.forceActiveFocus();
    }

    implicitWidth: textInput.width + 10
    implicitHeight: textInput.height + 10
    radius: height / 2
    color: "transparent"

    TextInput {
        id: textInput

        width: 100
        anchors.verticalCenter: parent.verticalCenter
        verticalAlignment: TextInput.AlignVCenter
        leftPadding: parent.radius
        rightPadding: leftPadding
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
        selectByMouse: true
        clip: true
        onEditingFinished: root.editingFinished()
        onAccepted: root.accepted()
    }

}

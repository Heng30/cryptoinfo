import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: inputBar

    property color textColor: theme.fontColor
    property alias text: textInput.text
    property string underText: ""
    property bool showBorder: true
    property alias textInput: textInput

    signal editingFinished()
    signal accepted()

    function forceFocus() {
        textInput.forceActiveFocus();
    }

    implicitHeight: textInput.height + 10
    color: "transparent"
    border.color: showBorder ? theme.borderColor : "transparent"
    border.width: showBorder ? 1 : 0

    TextInput {
        id: textInput

        width: parent.width
        anchors.verticalCenter: parent.verticalCenter
        verticalAlignment: TextInput.AlignVCenter
        leftPadding: theme.itemPadding
        rightPadding: leftPadding
        color: inputBar.textColor
        font.pixelSize: theme.fontPixelNormal
        selectByMouse: true
        clip: true
        onEditingFinished: inputBar.editingFinished()
        onAccepted: inputBar.accepted()
    }

    ItemLabel {
        z: textInput.z - 1
        anchors.verticalCenter: parent.verticalCenter
        height: parent.height
        width: textInput.width
        text: textInput.text.length > 0 ? "" : underText
        textColor: theme.underFontColor
    }

}

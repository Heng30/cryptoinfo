import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property color textColor: theme.fontColor
    property color cursorColor: theme.invertBgColor
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
        color: root.textColor
        cursorVisible: true
        selectByMouse: true
        clip: true
        onEditingFinished: root.editingFinished()
        onAccepted: root.accepted()

        cursorDelegate: Rectangle {
            property bool _showCursor: false

            width: 2
            height: parent.height
            color: _showCursor ? root.cursorColor : "transparent"

            Timer {
                id: timer

                interval: 500
                running: parent.visible
                repeat: true
                onTriggered: parent._showCursor = !parent._showCursor
            }

        }

    }

}

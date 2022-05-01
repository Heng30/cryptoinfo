import QtQuick 2.15
import QtQuick.Controls 2.15

TextField {
    id: txtField

    property bool showBorder: true
    property color bgColor: "transparent"
    property alias tipText: tip.text
    property bool isUseTip: false

    padding: 0
    color: theme.fontColor
    verticalAlignment: TextInput.AlignVCenter
    font.pixelSize: theme.fontPixelNormal
    selectByMouse: true
    clip: true

    Tip {
        id: tip

        property bool _entered: false

        visible: _entered && text.length > 0 && isUseTip
    }

    MouseArea {
        enabled: isUseTip
        anchors.fill: parent
        hoverEnabled: true
        onEntered: tip._entered = true
        onExited: tip._entered = false
        onClicked: txtField.forceActiveFocus()
        onDoubleClicked: txtField.selectAll()
    }

    background: Rectangle {
        anchors.fill: parent
        border.width: txtField.showBorder ? 1 : 0
        border.color: theme.borderColor
        color: txtField.bgColor
    }

}

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

    signal clicked()

    border.width: showBorder ? 1 : 0
    border.color: theme.borderColor
    radius: theme.itemRadius * 2
    color: "transparent"
    implicitWidth: label.width + 20
    implicitHeight: label.height + 20

    Label {
        id: label

        anchors.centerIn: parent
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
    }

    Tip {
        id: tip

        property bool _entered: false

        visible: _entered && text.length > 0
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: {
            tip._entered = true;
            txtBtn.color = theme.itemEnteredBG;
        }
        onExited: {
            tip._entered = false;
            txtBtn.color = "transparent";
        }
        onClicked: txtBtn.clicked()
    }

}

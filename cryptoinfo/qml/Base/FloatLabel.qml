import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: floatLabel

    property alias model: repeater.model
    property color fontColor: theme.fontColor

    implicitHeight: content.height
    color: theme.invertBgColor
    opacity: 0.7

    Column {
        id: content

        anchors.centerIn: parent
        spacing: theme.itemSpacing
        width: parent.width

        Repeater {
            id: repeater

            Row {
                width: parent.width
                spacing: theme.itemSpacing
                anchors.horizontalCenter: parent.horizontalCenter

                Label {
                    width: parent.width / 2 - parent.spacing / 2
                    anchors.verticalCenter: parent.verticalCenter
                    color: floatLabel.fontColor
                    font.pixelSize: theme.fontPixelNormal
                    elide: Text.ElideRight
                    text: String(modelData.leftText)
                }

                Label {
                    width: parent.width / 2 - parent.spacing / 2
                    anchors.verticalCenter: parent.verticalCenter
                    color: floatLabel.fontColor
                    font.pixelSize: theme.fontPixelNormal
                    elide: Text.ElideRight
                    text: String(modelData.rightText)
                }

            }

        }

    }

}

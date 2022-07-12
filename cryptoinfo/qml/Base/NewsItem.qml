import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: newsItem

    property bool enableBG: false
    property bool _isEntered: false
    property string urlText: ""
    property alias titleText: title.text
    property alias contentText: content.text
    property alias addTimeText: addTime.text

    signal clicked()
    signal openUrlClicked()

    implicitWidth: 100
    height: column.height
    radius: theme.itemRadius
    color: enableBG && _isEntered ? theme.itemEnteredBG : "transparent"

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: _isEntered = true
        onExited: _isEntered = false
        onClicked: newsItem.clicked()
    }

    Column {
        id: column

        width: parent.width
        anchors.margins: theme.itemMargins * 2
        spacing: theme.itemSpacing

        Text {
            id: title

            width: parent.width
            color: theme.fontColor
            font.pixelSize: theme.fontPixelNormal + 5
            font.bold: true
            wrapMode: Text.Wrap
        }

        Text {
            id: content

            width: parent.width
            color: theme.fontColor
            font.pixelSize: theme.fontPixelNormal + 4
            wrapMode: Text.Wrap
        }

        Row {
            id: footer

            width: parent.width
            spacing: theme.itemSpacing * 2
            layoutDirection: Qt.RightToLeft

            ItemText {
                id: addTime

                anchors.verticalCenter: parent.verticalCenter
            }

            ItemText {
                id: openUrl

                anchors.verticalCenter: parent.verticalCenter
                text: translator.tr("阅读原文")
                implicitWidth: label.width + theme.itemMargins * 2
                onClicked: newsItem.openUrlClicked()
                enableEnterCursorShape: true
            }

        }

    }

}

import QtQuick 2.15

import "./Base" as Base

Item {
    id: root

    property alias model: repeater.model

    width: parent.width
    height: content.height
    visible: _isHide

    Column {
        id: content

        width: parent.width

        Repeater {
            id: repeater

            delegate: Row {
                Base.ItemLabel {
                    text: modelData.key
                    width: root.width / 2
                }

                Base.ItemLabel {
                    text: modelData.value
                    width: root.width / 2
                }

            }

        }

    }

    Rectangle {
        anchors.fill: parent
        color: theme.borderColor
        opacity: 0.2

        MouseArea {
            anchors.fill: parent
            onDoubleClicked: priceItem._itemChecked = false
        }

    }

    Rectangle {
        anchors.bottom: parent.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        width: parent.width
        height: 1
        color: theme.borderColor
    }

}

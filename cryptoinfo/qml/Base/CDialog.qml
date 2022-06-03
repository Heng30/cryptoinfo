import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Popup {
    id: dialog

    required property Component sourceComponent
    property string headerText: ""

    implicitWidth: 450
    implicitHeight: 400
    focus: true
    closePolicy: Popup.NoAutoClose
    padding: 0

    Column {
        anchors.centerIn: parent
        width: parent.width - theme.itemMargins
        height: parent.height - theme.itemMargins

        Rectangle {
            id: header

            width: parent.width
            height: theme.panelHeaderHeight
            color: theme.headerBG

            Base.ItemLabel {
                anchors.left: parent.left
                anchors.verticalCenter: parent.verticalCenter
                text: dialog.headerText
                textFontBold: true
            }

            Base.DragArea {
                id: dragArea

                anchors.fill: parent
                moveItem: dialog
            }

            Base.ImageButton {
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                anchors.margins: theme.itemMargins
                height: parent.height - anchors.margins * 2
                width: height
                onClicked: dialog.visible = false
                source: "qrc:/res/image/exit.png"
                tipText: translator.tr("关闭")
            }

        }

        Loader {
            id: loader

            width: parent.width
            height: parent.height - parent.padding - header.height
            sourceComponent: dialog.sourceComponent
        }

    }

    background: Rectangle {
        anchors.fill: parent
        border.width: 2
        border.color: theme.borderColor
        color: theme.bgColor
    }

}

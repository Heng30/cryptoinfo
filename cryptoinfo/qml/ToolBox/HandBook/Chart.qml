import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Popup {
    id: chart

    implicitWidth: 450
    implicitHeight: content.height + theme.itemMargins * 4
    anchors.centerIn: parent
    focus: true
    closePolicy: Popup.NoAutoClose
    padding: 0

    Column {
        id: content

        anchors.centerIn: parent
        width: parent.width - theme.itemMargins * 4
        spacing: theme.itemSpacing

        Base.ItemText {
            width: parent.width
            text: "资产分布图"
            textFontPixelSize: theme.fontPixelNormal + 4
        }

        Item {
            width: parent.width
            height: btn.height + theme.itemSpacing * 2

            Base.TxtButton {
                id: btn

                anchors.rightMargin: theme.itemMargins * 4
                anchors.right: parent.right
                text: translator.tr("关闭")
                onClicked: chart.visible = false
            }

        }

    }

    background: Rectangle {
        anchors.fill: parent
        border.width: 2
        border.color: theme.borderColor
        color: theme.bgColor
    }

}

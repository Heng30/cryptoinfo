import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: fundbook

    property real _imageIconSize: 32 - theme.itemMargins * 2
    property int _iconCount: 3
    property real _itemWidth: (fundbook.width - _imageIconSize * _iconCount - theme.itemSpacing * 2) / 5

    width: parent.width
    implicitHeight: 100

    Column {
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Rectangle {
            width: parent.width
            height: parent.height - footer.height - parent.spacing
            border.width: 1
            border.color: "steelblue"
            color: "transparent"

            Column {
                anchors.fill: parent

                Row {
                    id: row

                    width: parent.width

                    Repeater {
                        model: [translator.tr("时间"), translator.tr("加密货币"), translator.tr("股市基金"), translator.tr("储蓄"), translator.tr("其他")]

                        delegate: Base.ItemText {
                            width: fundbook._itemWidth
                            height: theme.fontPixelNormal + theme.itemMargins * 2
                            text: modelData
                        }

                    }

                }

                ListView {
                    id: listView

                    width: parent.width
                    height: parent.height - row.height
                    spacing: theme.itemSpacing
                    clip: true
                    model: fundbook_model

                    ScrollBar.vertical: Base.SBar {
                        policy: ScrollBar.AlwaysOff
                    }

                    delegate: DItem {
                    }

                }

            }

        }

        Footer {
            id: footer
        }

    }

}

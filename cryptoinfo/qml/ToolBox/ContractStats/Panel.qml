import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property var headerModel: [translator.tr("类型"), translator.tr("盈亏次数"), translator.tr("盈利占比"), translator.tr("盈亏(美元)"), translator.tr("加减利润(美元)")]
    property real iconSize: 32
    property real iconFieldWidth: iconSize * 4 - theme.itemSpacing * 5
    property real headerItemWidth: (width - iconFieldWidth - content.spacing) / headerModel.length

    width: parent.width
    implicitHeight: 100

    Column {
        id: content

        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Rectangle {
            id: dItemField

            width: parent.width
            height: dItemColumn.height
            color: "transparent"

            Column {
                id: dItemColumn

                anchors.fill: parent

                Row {
                    Repeater {
                        model: headerModel

                        delegate: Base.ItemText {
                            width: headerItemWidth
                            text: modelData
                        }

                    }

                }

                Repeater {
                    model: contract_stats_model

                    delegate: DItem {
                    }

                }

            }

        }

        Chart {
            id: chart

            height: parent.height - dItemField.height - parent.spacing
        }

    }

}

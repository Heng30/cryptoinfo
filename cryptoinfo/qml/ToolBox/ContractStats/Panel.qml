import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property var headerModel: [translator.tr("类型"), translator.tr("盈亏次数"), translator.tr("盈亏次数占比"), translator.tr("盈亏(美元)"), translator.tr("加减利润(美元)")]
    property real iconSize: 32
    property real iconFieldWidth: iconSize * 4 - theme.itemSpacing * 5
    property real headerItemWidth: (width - iconFieldWidth - content.spacing) / headerModel.length

    width: parent.width
    implicitHeight: 100

    Column {
        id: dItemField

        property alias wlValue: winLostValue

        width: parent.width

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

        Base.ItemText {
            id: winLostValue

            property real value: 0

            anchors.horizontalCenter: parent.horizontalCenter
            label.font.pixelSize: theme.fontPixelNormal + 2
            text: (value < 0 ? translator.tr("亏损:") : translator.tr("盈利")) + " $" + Math.abs(value).toFixed(2)
            textColor: value < 0 ? theme.priceDownFontColor : theme.priceUpFontColor

        }

    }

    Chart {
        width: parent.width
        height: root.height - dItemField.height
        anchors.bottom: parent.bottom
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: row

    width: parent.width

    Item {
        width: itemRow.width
        height: itemRow.height

        Row {
            id: itemRow

            property real _itemWidth: row.width / repeater.model.length
            property color _textColor: (modelData.tvl > 1000 * 1000 * 100) ? theme.priceUpFontColor : theme.priceDownFontColor

            Repeater {
                id: repeater
                model: [modelData.index + 1, modelData.name, modelData.symbol, utilityFn.toFixedPrice(modelData.tvl)]

                Base.ItemText {
                    text: modelData
                    width: itemRow._itemWidth
                    textColor: itemRow._textColor
                }

            }

        }

        Rectangle {
            property bool _entered: false

            anchors.fill: parent
            color: _entered ? theme.itemEnterColor : "transparent"
            opacity: 0.5

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onExited: parent._entered = false
                onEntered: parent._entered = true
            }

        }

    }

}

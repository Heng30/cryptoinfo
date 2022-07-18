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
            property color _textColor: modelData.apy > 10 ? theme.priceUpFontColor : theme.priceDownFontColor

            Repeater {
                id: repeater

                model: [modelData.index + 1, modelData.chain, modelData.symbol, modelData.project, modelData.pool, modelData.exposure, utilityFn.toFixedPrice(modelData.tvl), utilityFn.toPercentString(modelData.apy), (modelData.stablecoin ? translator.tr("是") : translator.tr("否"))]

                Base.ItemText {
                    text: modelData
                    width: itemRow._itemWidth
                    textColor: itemRow._textColor
                    label.width: width - theme.itemSpacing * 2
                    label.elide: Text.ElideMiddle
                    tipText: (index === 2 || index === 4) ? text : ""
                    onIsEnteredChanged: bg._entered = isEntered
                    onClicked: {
                        if (index === 2 || index === 4)
                            utility.copy_to_clipboard_qml(text);

                    }
                }

            }

        }

        Rectangle {
            id: bg

            property bool _entered: false

            anchors.fill: parent
            color: _entered ? theme.itemEnterColor : "transparent"
            opacity: 0.5
        }
    }

}

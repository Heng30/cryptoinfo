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

            property color _textColor: modelData.market_cap_usd > 1000 * 1000 * 1000 ? theme.priceUpFontColor : theme.priceDownFontColor
            property real _itemWidth: row.width / repeater.model.length

            Repeater {
                id: repeater

                model: !!modelData ? [index + 1, (modelData.pubdate === "0") ? "-" : utilityFn.prettyDateStr(modelData.pubdate), modelData.name, modelData.symbol, modelData.address, utilityFn.prettyNumStr(modelData.price_usd.toFixed(2)), utilityFn.toFixedPrice(modelData.market_cap_usd), utilityFn.toFixedPrice(modelData.volume_usd), utilityFn.toFixedPrice(modelData.circulation_quantity), utilityFn.toFixedPrice(modelData.issue_quantity)] : []

                Base.ItemText {
                    text: modelData
                    textColor: itemRow._textColor
                    width: itemRow._itemWidth
                    label.width: width - theme.itemSpacing * 2
                    label.elide: Text.ElideMiddle
                    tipText: index === 4 ? text : ""
                    onIsEnteredChanged: bg._entered = isEntered
                    onClicked: {
                        if (index === 4)
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

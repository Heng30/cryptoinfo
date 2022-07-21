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

            property color _textColor: (monitor_eth_model.total_tx_value > 0 && modelData.tx_value > monitor_eth_model.total_tx_value / monitor_eth_model.items_len_qml()) ? theme.priceUpFontColor : theme.priceDownFontColor
            property real _itemWidth: row.width / repeater.model.length

            Repeater {
                id: repeater

                model: !!modelData ? [index + 1, modelData.blocktime, modelData.tx_hash, modelData.from, modelData.to, utilityFn.prettyNumStr(modelData.tx_value.toFixed(0))] : []

                Base.ItemText {
                    text: modelData
                    textColor: itemRow._textColor
                    width: itemRow._itemWidth
                    label.width: width - theme.itemSpacing * 2
                    label.elide: Text.ElideMiddle
                    tipText: (index === 2 || index === 3 || index === 4) ? text : ""
                    onIsEnteredChanged: bg._entered = isEntered
                    onClicked: {
                        if (index === 2 || index === 3 || index === 4)
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

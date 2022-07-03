import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: row

    width: parent.width

    Item {
        id: markerField

        height: itemRow.height
        width: height

        Rectangle {
            id: marker

            anchors.centerIn: parent
            width: parent.width / 2
            height: width
            color: modelData.marked ? theme.priceMarkedColor : theme.priceUnmarkedColor
            radius: width / 2

            MouseArea {
                anchors.fill: parent
                onClicked: price_model.set_marked_qml(index, !modelData.marked)
            }

        }

    }

    Item {
        width: itemRow.width
        height: itemRow.height

        Row {
            id: itemRow

            property color _textColor: modelData.percent_change_24h > 0 ? theme.priceUpFontColor : theme.priceDownFontColor
            property real _itemWidth: (row.width - markerField.width) / (repeater.model.length + 1)

            Repeater {
                id: repeater

                model: [modelData.index + 1, modelData.symbol, utilityFn.toFixedPrice(modelData.price_usd), utilityFn.toPercentString(modelData.percent_change_24h), utilityFn.toPercentString(modelData.percent_change_7d), utilityFn.toFixedPrice(modelData.volume_24h_usd)]

                Base.ItemText {
                    text: modelData
                    textColor: itemRow._textColor
                    width: itemRow._itemWidth
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
                onClicked: {
                    item._itemChecked = !item._itemChecked;
                    if (item._itemChecked)
                        root.viewAtIndex(index);

                }
            }

        }

    }

    Item {
        anchors.margins: theme.itemMargins
        height: itemRow.height
        width: itemRow._itemWidth

        Base.TxtField {
            id: floorPriceField
            property string _precent: modelData.floor_price <= 0 || modelData.price_usd <= 0 ? "" : (100 * Math.abs(modelData.price_usd - modelData.floor_price) / (modelData.price_usd > modelData.floor_price ? modelData.price_usd : modelData.floor_price)).toFixed(0) + "%"

            anchors.centerIn: parent
            horizontalAlignment: TextInput.AlignHCenter
            height: itemRow.height - parent.anchors.margins * 2
            width: itemRow._itemWidth - parent.anchors.margins * 2
            text: modelData.floor_price <= 0 ? "N/A" : utilityFn.toFixedPrice(modelData.floor_price) + "(" + _precent + ")"
            bgColor: (modelData.floor_price <= 0 || modelData.floor_price < modelData.price_usd) ? "transparent" : theme.floorPriceBGColor
            onAccepted: {
                markerField.forceActiveFocus();
                if (text === "N/A")
                    return ;

                price_model.set_floor_price_qml(index, Number(text));
                msgTip.add(translator.tr("保存成功!"), false);
            }
        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: sItem

    property var _model: []
    property int _dItemIndex: index
    property real _imageIconSize: 32 - theme.itemMargins * 2

    function add(is_sell, time, total_price, count) {
        _model.push({
            "time": time,
            "is_sell": is_sell,
            "total_price": total_price,
            "count": count
        });
        repeater.model = _model;
        dItem._showSItem = true;
        handbook_model.add_sub_model_item_qml(index, is_sell, time, total_price, count);
    }

    function reload() {
        _model = [];
        var sub_model_len = handbook_model.sub_model_len(index);
        for (var i = 0; i < sub_model_len; i++) {
            var item = handbook_model.sub_model_item(index, i);
            _model.push({
                "time": item.time,
                "is_sell": item.is_sell,
                "total_price": item.total_price,
                "count": item.count
            });
        }
        repeater.model = _model;
    }

    height: column.height
    color: theme.bgColor
    visible: true
    Component.onCompleted: sItem.reload()

    Column {
        id: column

        property real _itemWidth: (row.width - _isSellIconSize - _imageIconSize * 4) / (repeater2.model.length - 1)
        property real _isSellIconSize: theme.fontPixelNormal * 2 + theme.itemMargins * 2

        anchors.centerIn: parent
        width: parent.width
        spacing: theme.itemSpacing

        Row {
            id: row

            width: parent.width

            Repeater {
                id: repeater2

                model: [translator.tr("卖出"), translator.tr("时间"), translator.tr("总资金"), translator.tr("数量"), translator.tr("单价")]

                delegate: Base.ItemText {
                    width: index === 0 ? column._isSellIconSize : column._itemWidth
                    height: theme.fontPixelNormal + theme.itemMargins * 2
                    text: modelData
                }

            }

        }

        Repeater {
            id: repeater

            delegate: SDItem {
            }

        }

    }

}

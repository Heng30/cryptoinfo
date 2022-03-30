import QtQuick 2.15
import QtQuick.Controls 2.15
import "./Base" as Base

Item {
    id: root

    property bool _isPriceUpdate: false
    property real _smallItemWidth: 65

    function updatePrice() {
        const url = "https://api.alternative.me/v1/ticker/";
        const Http = new XMLHttpRequest();
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function(e) {
            if (Http.responseText.length <= 0) {
                root._isPriceUpdate = false;
                return ;
            }
            pricer_model.update_all_price(Http.responseText);
            pricer_addtion.update_time();
            root._isPriceUpdate = true;
        };
    }

    function viewAtBeginning() {
        listView.positionViewAtBeginning();
    }

    function viewAtEnd() {
        listView.positionViewAtEnd();
    }

    function viewAtIndex(index) {
        listView.positionViewAtIndex(index, ListView.Beginning);
    }

    width: parent.width

    ListView {
        id: listView

        clip: true
        model: pricer_model
        anchors.fill: parent

        ScrollBar.vertical: Base.SBar {
        }

        header: Rectangle {
            width: ListView.view.width
            height: headerRow.height
            color: theme.priceHeaderBG

            Row {
                id: headerRow

                width: parent.width

                Base.ItemText {
                    id: markerHeaderField

                    width: height
                    text: "..."
                    onClicked: {
                        pricer_model.toggle_sort_dir();
                        pricer_model.sort_by_key("marked");
                    }
                }

                Repeater {
                    id: repeater

                    property var keyModel

                    keyModel: ["index", "symbol", "price", "24h%", "24h_volume"]
                    model: [translator.tr("市值"), translator.tr("代币"), translator.tr("价格"), translator.tr("24h行情"), translator.tr("24h交易量")]

                    delegate: Base.ItemText {
                        width: index === 0 ? root._smallItemWidth : (parent.width - root._smallItemWidth - markerHeaderField.width) / (repeater.model.length - 1)
                        text: modelData
                        onClicked: {
                            pricer_model.toggle_sort_dir();
                            pricer_model.sort_by_key(repeater.keyModel[index]);
                        }
                    }

                }

            }

        }

        delegate: Item {
            id: priceItem

            property bool _itemChecked: false

            width: ListView.view.width
            height: column.height

            Column {
                id: column

                width: parent.width

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
                                onClicked: pricer_model.set_marked(index, !pricer_model.get_marked(index))
                            }

                        }

                    }

                    Item {
                        width: parent.width
                        height: itemRow.height

                        Row {
                            id: itemRow

                            property color _textColor: modelData.percent_change_24h > 0 ? theme.priceUpFontColor : theme.priceDownFontColor

                            width: parent.width

                            Repeater {
                                id: repeater2

                                model: [modelData.index, modelData.symbol, utilityFn.toFixedPrice(modelData.price_usd), utilityFn.toPercentString(modelData.percent_change_24h), utilityFn.toFixedPrice(modelData.volume_24h_usd)]

                                Base.ItemText {
                                    text: modelData
                                    textColor: itemRow._textColor
                                    width: index === 0 ? root._smallItemWidth : (parent.width - root._smallItemWidth - markerField.width) / (repeater2.model.length - 1)
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
                                    priceItem._itemChecked = !priceItem._itemChecked;
                                    if (priceItem._itemChecked)
                                        root.viewAtIndex(index);

                                }
                            }

                        }

                    }

                }

                PriceDetail {
                    id: priceDetail

                    visible: priceItem._itemChecked
                    model: [{
                        "key": translator.tr("名称"),
                        "value": modelData.name
                    }, {
                        "key": translator.tr("1小时行情"),
                        "value": utilityFn.toPercentString(modelData.percent_change_1h)
                    }, {
                        "key": translator.tr("24小时行情"),
                        "value": utilityFn.toPercentString(modelData.percent_change_24h)
                    }, {
                        "key": translator.tr("7天小时行情"),
                        "value": utilityFn.toPercentString(modelData.percent_change_7d)
                    }, {
                        "key": translator.tr("24小时交易量(美元)"),
                        "value": utilityFn.toFixedPrice(modelData.volume_24h_usd)
                    }, {
                        "key": translator.tr("市值(美元)"),
                        "value": utilityFn.toFixedPrice(modelData.market_cap_usd)
                    }, {
                        "key": translator.tr("可用流通量"),
                        "value": utilityFn.toFixedPrice(modelData.available_supply)
                    }, {
                        "key": translator.tr("最大流通量"),
                        "value": utilityFn.toFixedPrice(modelData.max_supply)
                    }, {
                        "key": translator.tr("更新时间"),
                        "value": pricer_addtion.get_time_from_utc_seconds(modelData.last_updated)
                    }]
                }

            }

        }

    }

    Timer {
        interval: root._isPriceUpdate ? 1000 * config.price_refresh_interval : 1000 * 10
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: root.updatePrice()
    }

}

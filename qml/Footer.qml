import QtQuick 2.15

import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property bool _isMarketUpdate: false

    function updateMarket() {
        const url = "https://api.alternative.me/v1/global/";
        const Http = new XMLHttpRequest();
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function(e) {
            if (Http.responseText.length <= 0) {
                root._isMarketUpdate = false;
                return ;
            }
            pricer_addtion.update_market(Http.responseText);
            root._isMarketUpdate = true;
        };
    }

    width: parent.width
    height: content.height

    Timer {
        interval: root._isMarketUpdate ? 1000 * 60 * 60 : 1000 * 60
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: root.updateMarket()
    }

    Column {
        id: content

        width: parent.width

        Item {
            width: parent.width
            height: 2

            Rectangle {
                width: parent.width - 20
                height: parent.height
                anchors.centerIn: parent
                color: theme.sepColor
            }

        }

        Row {
            width: parent.width

            Repeater {
                id: repeater

                property color _bullPercentColor: pricer_model.bull_percent > 0.5 ? theme.priceUpFontColor : theme.priceDownFontColor

                model: [{
                    "text": utilityFn.toFixedPrice(pricer_addtion.total_market_cap_usd),
                    "tipText": translator.tr("加密货币总市值(美元)")
                }, {
                    "text": utilityFn.toFixedPrice(pricer_addtion.total_24h_volume_usd),
                    "tipText": translator.tr("24小时交易量(美元)")
                }, {
                    "text": utilityFn.toPercentString(pricer_addtion.bitcoin_percentage_of_market_cap),
                    "tipText": translator.tr("BTC市值占比")
                }, {
                    "text": utilityFn.toPercentString(pricer_model.bull_percent * 100),
                    "tipText": translator.tr("24小时上涨代币占比")
                }]

                Base.ItemText {
                    width: parent.width / repeater.model.length
                    text: modelData.text
                    textColor: index === 3 ? repeater._bullPercentColor : theme.fontColor
                    tipText: modelData.tipText
                }

            }

        }

    }

}

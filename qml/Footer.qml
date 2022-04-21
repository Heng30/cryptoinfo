import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    width: parent.width
    height: content.height

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
                    "text": utilityFn.toFixedPrice(pricer_addition.total_market_cap_usd),
                    "tipText": translator.tr("加密货币总市值(美元)")
                }, {
                    "text": utilityFn.toFixedPrice(pricer_addition.total_24h_volume_usd),
                    "tipText": translator.tr("24小时交易量(美元)")
                }, {
                    "text": pricer_addition.greed_tody,
                    "tipText": translator.tr("今天贪婪恐惧指数")
                }, {
                    "text": pricer_addition.greed_yestoday,
                    "tipText": translator.tr("昨天贪婪恐惧指数")
                }, {
                    "text": utilityFn.toPercentString(pricer_addition.bitcoin_percentage_of_market_cap),
                    "tipText": translator.tr("BTC市值占比")
                }, {
                    "text": utilityFn.toPercentString(pricer_model.bull_percent * 100),
                    "tipText": translator.tr("24小时上涨代币占比")
                }, {
                    "text": pricer_model.update_time,
                    "tipText": translator.tr("更新时间")
                }]

                Base.ItemText {
                    width: parent.width / repeater.model.length
                    text: modelData.text
                    textColor: index === 5 ? repeater._bullPercentColor : theme.fontColor
                    tipText: modelData.tipText
                }

            }

        }

    }

}

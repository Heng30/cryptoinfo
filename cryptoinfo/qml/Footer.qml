import QtQuick 2.15
import PanelType 1.0
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

                property real _bull_percent: config.panel_type === PanelType.DefiProtocol ? defi_protocol_model.bull_percent : price_model.bull_percent
                property color _bullPercentColor: _bull_percent > 0.5 ? theme.priceUpFontColor : theme.priceDownFontColor

                model: [{
                    "text": utilityFn.toFixedPrice(price_addition.total_market_cap_usd),
                    "tipText": translator.tr("加密货币总市值(美元)")
                }, {
                    "text": utilityFn.toFixedPrice(price_addition.total_24h_volume_usd),
                    "tipText": translator.tr("24小时交易量(美元)")
                }, {
                    "text": price_addition.average + "(" + utilityFn.seconds2FixedTime(price_addition.average_wait) + ")",
                    "tipText": translator.tr("ETH标准油费(等待时间)") + utilityFn.paddingSpace(4) + translator.tr("慢") + ": " + price_addition.low +  "(" + utilityFn.seconds2FixedTime(price_addition.low_wait) + ")" + " " + translator.tr("快") + ": " + price_addition.fast + "(" + utilityFn.seconds2FixedTime(price_addition.fast_wait) + ")",
                    "color": price_addition.average_wait < 60 * 5 ? theme.priceUpFontColor : theme.priceDownFontColor
                }, {
                    "text": price_addition.otc_usdt.toFixed(2),
                    "tipText": price_addition.otc_datetime + utilityFn.paddingSpace(2)+ translator.tr("场外USDT价格") + utilityFn.paddingSpace(2) + translator.tr("溢价比") + ": "  + utilityFn.toPercentString(price_addition.otc_usd > 0 ? (price_addition.otc_usd - price_addition.otc_usdt) * 100 / price_addition.otc_usd : 0),
                    "color": price_addition.otc_usd < price_addition.otc_usdt ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": price_addition.greed_tody + utilityFn.paddingSpace(4) + price_addition.greed_yestoday,
                    "tipText": translator.tr("今天/昨天贪婪恐惧指数"),
                    "color": price_addition.greed_tody < 50 ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": String(price_addition.bitcoin_next_halving_days_left),
                    "tipText": translator.tr("BTC下次减半时间(天)"),
                    "color": price_addition.bitcoin_next_halving_days_left < 365 ? theme.priceDownFontColor : theme.fontColor
                }, {
                    "text": price_addition.ahr999.toFixed(2),
                    "tipText": translator.tr("AHR999 BTC 定投指数: 抄底区间(小于0.45) 定投区间(0.45-1.2) 起飞区间(大于5)"),
                    "color": price_addition.ahr999 < 0.45 ? theme.priceUpFontColor : (price_addition.ahr999 < 1.2 ? theme.fontColor : theme.priceDownFontColor)
                }, {
                    "text": utilityFn.toPercentString(price_addition.long_rate),
                    "tipText": translator.tr("24小时") + price_addition.long_short_symbol + translator.tr("多空比") + utilityFn.paddingSpace(2) + translator.tr("多仓位") + ": " + utilityFn.toFixedPrice(price_addition.long_vol_usd) + utilityFn.paddingSpace(2) + translator.tr("空仓位") + ": " + utilityFn.toFixedPrice(price_addition.short_vol_usd),
                    "color": price_addition.long_rate > 50 ? theme.priceUpFontColor : theme.priceDownFontColor
                }, {
                    "text": utilityFn.toPercentString(price_addition.bitcoin_percentage_of_market_cap),
                    "tipText": translator.tr("BTC市值占比"),
                    "color": price_addition.bitcoin_percentage_of_market_cap < 0.5 ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": utilityFn.toPercentString(_bull_percent * 100),
                    "tipText": translator.tr("24小时上涨比率"),
                    "color": repeater._bullPercentColor
                }, {
                    "text": config.panel_type === PanelType.DefiProtocol ? defi_protocol_model.update_time : (config.panel_type === PanelType.DefiChain ? defi_chain_model.update_time: (config.panel_type === PanelType.News ? news_model.update_time : (config.panel_type === PanelType.Price ? price_model.update_time : "N/A"))),
                    "tipText": translator.tr("更新时间")
                }]

                Base.ItemText {
                    width: parent.width / repeater.model.length
                    text: modelData.text
                    textColor: !!modelData.color ? modelData.color : theme.fontColor
                    tipText: modelData.tipText
                }

            }

        }

    }

}

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
                // }, {
                //     "text": price_addition.ahr999.toFixed(2),
                //     "tipText": translator.tr("AHR999 BTC 定投指数: 抄底区间(小于0.45) 定投区间(0.45-1.2) 起飞区间(大于5)"),
                //     "color": price_addition.ahr999 < 0.45 ? theme.priceUpFontColor : (price_addition.ahr999 < 1.2 ? theme.fontColor : theme.priceDownFontColor)

                id: repeater

                property real _bull_percent: _bull_percent_cal()

                function _bull_percent_cal() {
                    if (config.panel_type === PanelType.Chain) {
                        if (_chainProtocolTabIsChecked)
                            return chain_protocol_model.bull_percent;

                    } else if (config.panel_type === PanelType.Exchange) {
                        if (_exchangeBtcTabIsChecked)
                            return exchange_btc_model.bull_percent;

                    } else if (config.panel_type === PanelType.StableCoin) {
                        if (_stableCoinMcapTabIsChecked)
                            return stable_coin_mcap_model.bull_percent;

                    } else if (config.panel_type === PanelType.Price) {
                        return price_model.bull_percent;
                    }
                    return -1;
                }

                function _updateTime() {
                    if (config.panel_type === PanelType.Chain) {
                        if (_chainProtocolTabIsChecked)
                            return chain_protocol_model.update_time;
                        else if (_chainTvlTabIsChecked)
                            return chain_tvl_model.update_time;
                    } else if (config.panel_type === PanelType.News) {
                        return news_model.update_time;
                    } else if (config.panel_type === PanelType.Price) {
                        return price_model.update_time;
                    } else if (config.panel_type == PanelType.Exchange) {
                        if (_exchangeBtcTabIsChecked)
                            return exchange_btc_model.update_time;

                    } else if (config.panel_type === PanelType.Monitor) {
                        if (_monitorBtcTabIsChecked)
                            return monitor_btc_model.update_time;

                    } else if (config.panel_type === PanelType.StableCoin) {
                        if (_stableCoinMcapTabIsChecked)
                            return stable_coin_mcap_model.update_time;
                        else if (_stableCoinChainTabIsChecked)
                            return stable_coin_chain_model.update_time;

                    }
                    return "N/A";
                }

                model: [{
                    "text": utilityFn.toFixedPrice(price_addition.total_market_cap_usd),
                    "tipText": translator.tr("加密货币总市值(美元)")
                }, {
                    "text": utilityFn.toFixedPrice(price_addition.total_24h_volume_usd),
                    "tipText": translator.tr("24小时交易量(美元)")
                }, {
                    "text": utilityFn.toFixedPrice(price_addition.total_blast_24h),
                    "tipText": utility.get_time_from_utc_seconds_qml(price_addition.total_blast_update_time) + utilityFn.paddingSpace(2) + translator.tr("24小时爆仓量(美元)") + utilityFn.paddingSpace(2) + translator.tr("1小时爆仓量") + ": " + utilityFn.toFixedPrice(price_addition.total_blast_1h) + utilityFn.paddingSpace(2) + translator.tr("24小时爆仓合约数") + ": " + utilityFn.toFixedPrice(price_addition.total_blast_num_24h)
                }, {
                    "text": price_addition.average,
                    "tipText": translator.tr("ETH标准油费") + "(" + utilityFn.seconds2FixedTime(price_addition.average_wait) + ")" + utilityFn.paddingSpace(2) + translator.tr("慢") + ": " + price_addition.low + "(" + utilityFn.seconds2FixedTime(price_addition.low_wait) + ")" + " " + translator.tr("快") + ": " + price_addition.fast + "(" + utilityFn.seconds2FixedTime(price_addition.fast_wait) + ")",
                    "color": price_addition.average_wait < 60 * 5 ? theme.priceUpFontColor : theme.priceDownFontColor
                }, {
                    "text": price_addition.eth_burned_rate_1h.toFixed(2) + utilityFn.paddingSpace(2) + price_addition.eth_burned_rate_24h.toFixed(2),
                    "tipText": translator.tr("1小时ETH燃烧速率") + utilityFn.paddingSpace(2) + translator.tr("24小时ETH燃烧速率") + utilityFn.paddingSpace(2) + translator.tr("总ETH燃烧量") + ": " + price_addition.eth_burned_total.toFixed(2) + "ETH",
                    "color": price_addition.eth_burned_rate_1h > price_addition.eth_burned_rate_24h ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": price_addition.otc_usdt.toFixed(2),
                    "tipText": price_addition.otc_datetime + utilityFn.paddingSpace(2) + translator.tr("场外USDT价格") + utilityFn.paddingSpace(2) + translator.tr("溢价比") + ": " + utilityFn.toPercentString(price_addition.otc_usd > 0 ? (price_addition.otc_usd - price_addition.otc_usdt) * 100 / price_addition.otc_usd : 0),
                    "color": price_addition.otc_usd < price_addition.otc_usdt ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": price_addition.greed_tody + utilityFn.paddingSpace(2) + price_addition.greed_yestoday,
                    "tipText": translator.tr("今天/昨天贪婪恐惧指数"),
                    "color": price_addition.greed_tody < 50 ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": String(price_addition.bitcoin_next_halving_days_left),
                    "tipText": translator.tr("BTC下次减半时间(天)"),
                    "color": price_addition.bitcoin_next_halving_days_left < 365 ? theme.priceDownFontColor : theme.fontColor
                }, {
                    "text": (price_addition.btc_ma730 <= 0 || price_addition.btc_ma730_price <= 0 || price_addition.btc_ma730_mu5 <= 0) ? "N/A" : (price_addition.btc_ma730_price < price_addition.btc_ma730 ? utilityFn.toPercentString(100 * (price_addition.btc_ma730 - price_addition.btc_ma730_price) / price_addition.btc_ma730) : (price_addition.btc_ma730_price < price_addition.btc_ma730_mu5 ? utilityFn.toPercentString(100 * (price_addition.btc_ma730_price - price_addition.btc_ma730) / (price_addition.btc_ma730_mu5 / price_addition.btc_ma730)) : utilityFn.toPercentString((price_addition.btc_ma730_price - price_addition.btc_ma730_mu5) / price_addition.btc_ma730_mu5))),
                    "tipText": utility.get_time_from_utc_seconds_qml(price_addition.btc_ma730_create_time) + utilityFn.paddingSpace(2) + translator.tr("BTC mA730逃顶/抄底指数(底部 当前 顶部)") + ": " + price_addition.btc_ma730.toFixed(0) + utilityFn.paddingSpace(2) + price_addition.btc_ma730_price.toFixed(0) + utilityFn.paddingSpace(2) + price_addition.btc_ma730_mu5.toFixed(0),
                    "color": price_addition.btc_ma730_price < price_addition.btc_ma730 ? theme.priceUpFontColor : (price_addition.btc_ma730_price < price_addition.btc_ma730_mu5 ? theme.fontColor : theme.priceDownFontColor)
                }, {
                    "text": utilityFn.toPercentString(price_addition.long_rate),
                    "tipText": translator.tr("24小时") + price_addition.long_short_symbol + translator.tr("多空比") + utilityFn.paddingSpace(2) + translator.tr("多仓位") + ": " + utilityFn.toFixedPrice(price_addition.long_vol_usd) + utilityFn.paddingSpace(2) + translator.tr("空仓位") + ": " + utilityFn.toFixedPrice(price_addition.short_vol_usd),
                    "color": price_addition.long_rate > 50 ? theme.priceUpFontColor : theme.priceDownFontColor
                }, {
                    "text": utilityFn.toPercentString(price_addition.btc_hash_percent_24h),
                    "tipText": translator.tr("24小时BTC算力") + (price_addition.btc_hash_percent_24h > 0 ? translator.tr("上升") : translator.tr("下降")) + utilityFn.paddingSpace(2) + translator.tr("BTC全球算力") + ": " + price_addition.btc_hash,
                    "color": price_addition.btc_hash_percent_24h < 0 ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": utilityFn.toPercentString(price_addition.bitcoin_percentage_of_market_cap),
                    "tipText": translator.tr("BTC市值占比"),
                    "color": price_addition.bitcoin_percentage_of_market_cap < 0.5 ? theme.priceDownFontColor : theme.priceUpFontColor
                }, {
                    "text": _bull_percent >= 0 ? utilityFn.toPercentString(_bull_percent * 100) : "N/A",
                    "tipText": translator.tr("24小时上涨比率"),
                    "color": _bull_percent > 0.5 ? theme.priceUpFontColor : (_bull_percent > 0 ? theme.priceDownFontColor : theme.fontColor)
                }, {
                    "text": _updateTime(),
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

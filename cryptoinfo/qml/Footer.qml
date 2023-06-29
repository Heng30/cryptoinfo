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

                property real _bull_percent: _bull_percent_cal()

                function _bull_percent_cal() {
                    if (config.panel_type === PanelType.Chain) {
                        if (_chainProtocolTabIsChecked)
                            return chain_protocol_model.bull_percent;

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
                        else if (_chainYieldTabIsChecked)
                            return chain_yield_model.update_time;
                        else if (_chainCryptoFeeTabIsChecked)
                            return crypto_fee_model.update_time;
                    } else if (config.panel_type === PanelType.Intel) {
                        if (_macroNewsTabIsChecked)
                            return macro_news_model.update_time;
                        else if (_macroEventTabIsChecked)
                            return macro_event_model.update_time;
                    } else if (config.panel_type === PanelType.Price) {
                        return price_model.update_time;
                    } else if (config.panel_type === PanelType.StableCoin) {
                        if (_stableCoinMcapTabIsChecked)
                            return stable_coin_mcap_model.update_time;
                        else if (_stableCoinChainTabIsChecked)
                            return stable_coin_chain_model.update_time;
                    } else if (config.panel_type === PanelType.Account) {
                        if (_accountChanTabIsChecked)
                            return okex_account.update_time;
                        else if (_accountMainRestTabIsChecked)
                            return okex_main_account_rest_model.update_time;
                        else if (_accountDepositTabIsChecked)
                            return okex_deposit_rest_model.update_time;
                        else if (_accountWithdrawalTabIsChecked)
                            return okex_withdrawal_rest_model.update_time;
                        else if (_accountBillTabIsChecked)
                            return okex_bill_rest_model.update_time;
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
                    "tipText": utility.get_time_from_utc_seconds_qml(price_addition.total_blast_update_time) + utilityFn.paddingSpace(2) + translator.tr("24小时爆仓量(美元)") + utilityFn.paddingSpace(2) + translator.tr("1小时爆仓量") + ": " + utilityFn.toFixedPrice(price_addition.total_blast_1h) + utilityFn.paddingSpace(2) + translator.tr("24小时爆仓合约数") + ": " + utilityFn.prettyNumStr(price_addition.total_blast_num_24h.toFixed(0))
                }, {
                    "text": price_addition.eth_burned_rate_1h.toFixed(2) + utilityFn.paddingSpace(2) + price_addition.eth_burned_rate_24h.toFixed(2),
                    "tipText": translator.tr("1小时ETH燃烧速率") + utilityFn.paddingSpace(2) + translator.tr("24小时ETH燃烧速率") + utilityFn.paddingSpace(2) + translator.tr("总ETH燃烧量") + ": " + utilityFn.prettyNumStr(price_addition.eth_burned_total.toFixed(0)) + "ETH",
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

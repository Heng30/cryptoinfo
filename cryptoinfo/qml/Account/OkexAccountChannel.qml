import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

Item {
    width: parent.width

    Column {
        anchors.fill: parent
        spacing: theme.itemSpacing

        Rectangle {
            width: parent.width
            height: parent.height - itemLabel.height - parent.spacing
            border.width: 1
            border.color: "steelblue"
            color: "transparent"

            BaseItemPanel.Panel {
                anchors.fill: parent
                showSbar: false
                listModel: okex_account_channel_model
                itemTipTextShowModel: []
                headerSortKeyModel: []
                headerModel: [translator.tr("..."), translator.tr("代币"), translator.tr("币价"), translator.tr("权益"), translator.tr("可用余额"), translator.tr("可用保证金"), translator.tr("价值(美元)"), translator.tr("折算权益(美元)"), translator.tr("逐仓权益"), translator.tr("逐仓未实现盈亏"), translator.tr("未实现盈亏")]
                itemModel: (function(index, modelData) {
                    return !!modelData ? [index + 1, modelData.ccy, utilityFn.toFixedPrice(modelData.coin_usd_price), utilityFn.toFixedPrice(modelData.eq), utilityFn.toFixedPrice(modelData.cash_bal), utilityFn.toFixedPrice(modelData.avail_eq), utilityFn.toFixedPrice(modelData.eq_usd), utilityFn.toFixedPrice(modelData.dis_eq), utilityFn.toFixedPrice(modelData.iso_eq), utilityFn.toFixedPrice(modelData.iso_upl), utilityFn.toFixedPrice(modelData.upl)] : [];
                })
                itemTextColor: (function(modelData) {
                    return Number(modelData.upl) >= 0 ? theme.priceUpFontColor : theme.priceDownFontColor;
                })
            }

        }

        Base.ItemLabel {
            id: itemLabel

            anchors.horizontalCenter: parent.horizontalCenter
            text: translator.tr("更新时间") + ": " + okex_account_channel_model.utime + utilityFn.paddingSpace(4) + translator.tr("总资产(美元)") + ": " + Number(okex_account_channel_model.total_eq).toFixed(2) + utilityFn.paddingSpace(4) + translator.tr("逐仓仓位(美元)") + ": " + Number(okex_account_channel_model.iso_eq).toFixed(2)
        }

    }

}

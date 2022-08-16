import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    showSbar: false
    listModel: nft_genie_model
    itemTipTextShowModel: [true, true, false, false, false, false, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("名称"), translator.tr("地址"), translator.tr("24h交易量(ETH)"), translator.tr("24h交易量"),  translator.tr("在售占比"), translator.tr("市值(ETH)"), translator.tr("总数量(个)"),translator.tr("持有者数"), translator.tr("地板价(ETH)")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [modelData.name, modelData.address, utilityFn.toFixedPrice(modelData.volume), utilityFn.toPercentString(modelData.volume_change), modelData.percent_listed, utilityFn.toFixedPrice(modelData.market_cap), utilityFn.prettyNumStr(modelData.supply), utilityFn.prettyNumStr(modelData.owners), utilityFn.toFixedPrice(modelData.floor) ] : [];
    })
    itemTextColor: (function(modelData) {
        return Number(modelData.volume_change) > 0 ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import NFTGemSortKey 1.0
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    listModel: nft_gem_model
    itemTipTextShowModel: [false, true, false, false, false, false, false, false, false, false]
    headerSortKeyModel: [NFTGemSortKey.Unknown, NFTGemSortKey.Name, NFTGemSortKey.OneDayVolume, NFTGemSortKey.OneDayChange, NFTGemSortKey.SevenDayChange, NFTGemSortKey.TotalVolume, NFTGemSortKey.TotalSales, NFTGemSortKey.TotalSupply, NFTGemSortKey.NumOwners, NFTGemSortKey.FloorPrice]
    headerModel: [translator.tr("..."), translator.tr("名称"), translator.tr("24h交易量(ETH)"), translator.tr("24行情"), translator.tr("7d行情"), translator.tr("总交易量(ETH)"), translator.tr("总售出(个)"), translator.tr("总供应量(个)"), translator.tr("持有者数"), translator.tr("地板价(ETH)")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.name, utilityFn.toFixedPrice(modelData.one_day_volume), utilityFn.toPercentString(modelData.one_day_change * 100), utilityFn.toPercentString(modelData.seven_day_change * 100), utilityFn.toFixedPrice(modelData.total_volume), Number(modelData.total_sales).toFixed(0), Number(modelData.total_supply).toFixed(0), Number(modelData.num_owners).toFixed(0), utilityFn.toFixedPrice(modelData.floor_price)] : [];
    })
    itemTextColor: (function(modelData) {
        return Number(modelData.one_day_change) > 0 ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

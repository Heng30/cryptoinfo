import QtQuick 2.15
import QtQuick.Controls 2.15
import NFTSudoSwapSortKey 1.0
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    listModel: nft_sudoswap_model
    itemTipTextShowModel: [false, true, true, false, false, false, false, false]
    headerSortKeyModel: [NFTSudoSwapSortKey.Unknown, NFTSudoSwapSortKey.Address, NFTSudoSwapSortKey.Name, NFTSudoSwapSortKey.SellQuote, NFTSudoSwapSortKey.BuyQuote, NFTSudoSwapSortKey.OfferTvl, NFTSudoSwapSortKey.PoolCount, NFTSudoSwapSortKey.ItemCount]
    headerModel: [translator.tr("..."), translator.tr("地址"), translator.tr("名称"), translator.tr("地板价(ETH)"), translator.tr("最高出价(ETH)"), translator.tr("总价值(ETH)"), translator.tr("池子数量"), translator.tr("NFT数量")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.address, modelData.name, utilityFn.toFixedPrice(modelData.buy_quote), utilityFn.toFixedPrice(modelData.sell_quote), utilityFn.toFixedPrice(modelData.offer_tvl ), Number(modelData.pool_count).toFixed(0), Number(modelData.item_count).toFixed(0)] : [];
    })
    itemTextColor: (function(modelData) {
        return Number(modelData.buy_quote) > 1 ? theme.priceUpFontColor : theme.priceDownFontColor;
        return theme.priceUpFontColor;
    })
}

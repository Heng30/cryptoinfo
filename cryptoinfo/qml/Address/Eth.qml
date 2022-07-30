import QtQuick 2.15
import QtQuick.Controls 2.15
import AddressEthSortKey 1.0
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    listModel: address_eth_model
    itemTipTextShowModel: [false, true, false, false, false]
    headerSortKeyModel: [AddressEthSortKey.Unknown, AddressEthSortKey.Unknown, AddressEthSortKey.Balance, AddressEthSortKey.Percentage, AddressEthSortKey.Transactions]
    headerModel: [translator.tr("..."), translator.tr("地址"), translator.tr("数量"), translator.tr("占比"), translator.tr("转帐次数")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.address, utilityFn.prettyNumStr(modelData.balance.toFixed(0)), utilityFn.toPercentString(modelData.percentage * 100), utilityFn.prettyNumStr(modelData.transactions)] : [];
    })
    itemTextColor: (function(modelData) {
        return modelData.percentage > 0.01 ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

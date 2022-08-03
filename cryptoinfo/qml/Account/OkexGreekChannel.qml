import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    width: parent.width
    showSbar: false
    listModel: okex_greek_channel_model
    itemTipTextShowModel: []
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("代币"), translator.tr("数量"), translator.tr("价值(美元)"), translator.tr("更新时间")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.ccy, utilityFn.toFixedPrice(modelData.delta_bs), Number(modelData.delta_pa) > 0 ? utilityFn.toFixedPrice(modelData.delta_pa) : "-", modelData.ts] : [];
    })
    itemTextColor: (function(modelData) {
        return Number(modelData.delta_pa) >= okex_greek_channel_model.avg_value ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

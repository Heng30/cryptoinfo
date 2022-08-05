import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    width: parent.width
    showSbar: false
    listModel: okex_greek_channel_model
    itemTipTextShowModel: []
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("代币"), translator.tr("数量"), translator.tr("更新时间")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.ccy, utilityFn.prettyNumStr(Number(modelData.delta_bs).toFixed(2)), modelData.ts] : [];
    })
    itemTextColor: (function(modelData) {
        return theme.priceUpFontColor;
    })
}

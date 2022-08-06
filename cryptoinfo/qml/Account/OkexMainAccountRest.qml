import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    width: parent.width
    showSbar: false
    listModel: okex_main_account_rest_model
    itemTipTextShowModel: []
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("代币"), translator.tr("数量"), translator.tr("可用数量"), translator.tr("冻结数量")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.ccy, utilityFn.prettyNumStr(Number(modelData.bal).toFixed(2)), utilityFn.prettyNumStr(Number(modelData.avail_bal).toFixed(2)), utilityFn.prettyNumStr(Number(modelData.frozen_bal).toFixed(2))] : [];
    })
    itemTextColor: (function(modelData) {
        return theme.priceUpFontColor;
    })
}

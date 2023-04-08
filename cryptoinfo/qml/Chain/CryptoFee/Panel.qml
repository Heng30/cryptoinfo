import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    listModel: crypto_fee_model
    itemTipTextShowModel: []
    headerSortKeyModel: []
    itemWidthList: ["100px", "expand", "250px", "250px"]
    headerModel: ["...", translator.tr("名称"), translator.tr("1天油费(美元)"), translator.tr("7天平均油费(美元)")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.name, modelData.fee_1day, modelData.fee_7day_avg] : [];
    })
}

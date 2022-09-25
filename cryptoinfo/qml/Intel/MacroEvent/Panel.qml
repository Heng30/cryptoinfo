import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    function _importance(value) {
        if (value === 1)
            return translator.tr("低");
        else if (value === 2)
            return translator.tr("中");
        else if (value === 3)
            return translator.tr("高");
        else
            return "N/A";
    }

    listModel: macro_event_model
    itemTipTextShowModel: [false, false, true, false, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("时间"), translator.tr("国家"), translator.tr("事件"), translator.tr("重要程度"), translator.tr("今值"), translator.tr("预期"), translator.tr("前值")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [modelData.public_date, modelData.country, modelData.title, _importance(modelData.importance), !!modelData.actual ? (modelData.actual + modelData.unit) : "--", !!modelData.forecast ? (modelData.forecast + modelData.unit) : "--", modelData.previous + modelData.unit] : [];
    })
    itemTextColor: (function(modelData) {
        return modelData.importance === 2 ? theme.priceUpFontColor : (modelData.importance === 3 ? theme.priceDownFontColor : theme.fontColor);
    })
}

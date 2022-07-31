import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    showSbar: false
    listModel: okex_subscribe_status_model
    itemTipTextShowModel: [false, true, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("WebSocket 地址"), translator.tr("频道名称"), translator.tr("频道类型"), translator.tr("订阅状态")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.url, modelData.channel, modelData.is_pub ? translator.tr("公有") : translator.tr("私有"), modelData.is_ok ? translator.tr("成功") : translator.tr("失败")] : [];
    })
    itemTextColor: (function(modelData) {
        return modelData.is_ok ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

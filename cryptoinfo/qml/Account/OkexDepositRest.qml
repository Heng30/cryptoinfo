import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    function _tx_state(modelData) {
        let state = Number(modelData.state);
        if (state === 0)
            return translator.tr("等待确认");
        else if (state === 1)
            return translator.tr("确认到账");
        else if (state === 2)
            return translator.tr("充值成功");
        else if (state === 8)
            return translator.tr("暂停充值");
        else if (state === 12)
            return translator.tr("被冻结");
        else if (state === 13)
            return translator.tr("被拦截");
        else
            return translator.tr("未知");
    }

    width: parent.width
    showSbar: false
    listModel: okex_deposit_rest_model
    itemTipTextShowModel: [false, true, true, true, true, false, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("区块ID"), translator.tr("发送者"), translator.tr("接受者"), translator.tr("公链"), translator.tr("代币"), translator.tr("数量"), translator.tr("状态(确认数)"), translator.tr("到账时间")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.tx_id, modelData.from, modelData.to, modelData.chain, modelData.ccy, utilityFn.prettyNumStr(Number(modelData.amt).toFixed(2)), _tx_state(modelData) + "("+ modelData.actual_dep_blk_confirm + ")", modelData.ts] : [];
    })
    itemTextColor: (function(modelData) {
        return modelData.state === "2" ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

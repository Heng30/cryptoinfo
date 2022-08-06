import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    function _tx_state(modelData) {
        let state = Number(modelData.state);
        if (state === -3)
            return translator.tr("正在撤销");
        else if (state === -2)
            return translator.tr("已撤销");
        else if (state === -1)
            return translator.tr("提现失败");
        else if (state === 0)
            return translator.tr("等待提现");
        else if (state === 1)
            return translator.tr("正在提现");
        else if (state === 2)
            return translator.tr("提现成功");
        else if (state === 8)
            return translator.tr("邮箱确认");
        else if (state === 12)
            return translator.tr("人工审核");
        else if (state === 13)
            return translator.tr("身份认证");
        else
            return translator.tr("未知");
    }

    width: parent.width
    showSbar: false
    listModel: okex_withdrawal_rest_model
    itemTipTextShowModel: [false, true, true, true, false, false, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("区块ID"), translator.tr("接受者"), translator.tr("公链"), translator.tr("代币"), translator.tr("数量"), translator.tr("手续费"), translator.tr("状态"), translator.tr("申请时间")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.tx_id, modelData.to, modelData.chain, modelData.ccy, utilityFn.prettyNumStr(Number(modelData.amt).toFixed(2)), utilityFn.prettyNumStr(Number(modelData.fee).toFixed(2)), _tx_state(modelData), modelData.ts] : [];
    })
    itemTextColor: (function(modelData) {
        return modelData.state === "2" ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

BaseItemPanel.Panel {
    function _inst_type(modelData) {
        var type = modelData.inst_type;
        if (type === "SPOT")
            return translator.tr("币币");
        else if (type === "MARGIN")
            return translator.tr("币币杠杆");
        else if (type === "SWAP")
            return translator.tr("永续合约");
        else if (type === "FUTURES")
            return translator.tr("交割合约");
        else if (type === "OPTION")
            return translator.tr("期权");
        else
            return translator.tr("其他");
    }

    function _bill_type(modelData) {
        var bill_type = Number(modelData.bill_type);
        var sub_type = Number(modelData.sub_type);
        var bill_type_list = ["未知", "划转", "交易", "交割", "自动换币", "强平", "保证金划转", "扣息", "资金费", "自动减仓", "穿仓补偿", "系统换币", "策略划拨", "对冲减仓"];
        var sub_type_list = [[1, "买入"], [2, "卖出"], [3, "开多"], [4, "开空"], [5, "平多"], [6, "平空"], [9, "市场借币扣息"], [11, "转入"], [12, "转出"], [14, "尊享借币扣息"], [160, "手动追加保证金"], [161, "手动减少保证金"], [162, "自动追加保证金"], [114, "自动换币买入"], [115, "自动换币卖出"], [118, "系统换币转入"], [119, "系统换币转出"], [100, "强减平多"], [101, "强减平空"], [102, "强减买入"], [103, "强减卖出"], [104, "强平平多"], [105, "强平平空"], [106, "强平买入"], [107, "强平卖出"], [110, "强平换币转入"], [111, "强平换币转出"], [125, "自动减仓平多"], [126, "自动减仓平空"], [127, "自动减仓买入"], [128, "自动减仓卖出"], [131, "对冲买入"], [132, "对冲卖出"], [170, "到期行权"], [171, "到期被行权"], [172, "到期作废"], [112, "交割平多"], [113, "交割平空"], [117, "交割 / 期权穿仓补偿"], [173, "资金费支出"], [174, "资金费收入"], [200, "系统转入"], [201, "手动转入"], [202, "系统转出"], [203, "手动转出"]];
        var res = "";
        var found = false;
        if (bill_type >= bill_type_list.length)
            res += translator.tr("未知");
        else
            res += translator.tr(bill_type_list[bill_type]);
        for (var i = 0; i < sub_type_list.length; i++) {
            if (sub_type_list[i][0] === sub_type) {
                res += "/" + sub_type_list[i][1];
                found = true;
                break;
            }
        }
        if (!found)
            res += "/" + translator.tr("未知");

        return res;
    }

    function _bal(modelData) {
        return utilityFn.prettyNumStr(Number(modelData.bal).toFixed(2)) + "(" + utilityFn.prettyNumStr(Number(modelData.bal_chg).toFixed(2)) + ")";
    }

    function _pos_bal(modelData) {
        return utilityFn.prettyNumStr(Number(modelData.pos_bal).toFixed(2)) + "(" + utilityFn.prettyNumStr(Number(modelData.pos_bal_chg).toFixed(2)) + ")";
    }

    width: parent.width
    showSbar: false
    listModel: okex_bill_rest_model
    itemTipTextShowModel: [false, false, true, true, true, true, true, false, false, false]
    headerSortKeyModel: []
    headerModel: [translator.tr("..."), translator.tr("创建时间"), translator.tr("产品"), translator.tr("类型(代币)"), translator.tr("帐单类型"), translator.tr("账户余额(变动)"), translator.tr("仓位余额(变动)"), translator.tr("数量(张数)"), translator.tr("收益(数量)"), translator.tr("手续费")]
    itemModel: (function(index, modelData) {
        return !!modelData ? [index + 1, modelData.ts, modelData.inst_id, _inst_type(modelData) + "(" + modelData.ccy + ")", _bill_type(modelData), _bal(modelData), _pos_bal(modelData), utilityFn.prettyNumStr(Number(modelData.sz).toFixed(2)), utilityFn.prettyNumStr(Number(modelData.pnl).toFixed(2)), utilityFn.toFixedPrice(modelData.fee)] : [];
    })
    itemTextColor: (function(modelData) {
        return Number(modelData.pnl) >= 0 ? theme.priceUpFontColor : theme.priceDownFontColor;
    })
}

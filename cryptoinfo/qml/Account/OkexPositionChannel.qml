import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Base/ItemPanel" as BaseItemPanel

Item {
    width: parent.width

    Column {
        anchors.fill: parent
        spacing: theme.itemSpacing

        Rectangle {
            width: parent.width
            height: parent.height - itemLabel.height - parent.spacing
            border.width: 1
            border.color: "steelblue"
            color: "transparent"

            BaseItemPanel.Panel {
                function _posSide(modelData) {
                    var side = modelData.pos_side;
                    var pos = Number(modelData.pos);
                    if (side === "long") {
                        return translator.tr("双向多头");
                    } else if (side === "short") {
                        return translator.tr("双向空头");
                    } else {
                        if (pos > 0)
                            return translator.tr("单向多头");
                        else
                            return translator.tr("单向空头");
                    }
                }

                anchors.fill: parent
                showSbar: false
                listModel: okex_position_channel_model
                itemTipTextShowModel: []
                headerSortKeyModel: []
                headerModel: [translator.tr("创建时间"), translator.tr("合约"), translator.tr("模式"), translator.tr("方向(倍数)"), translator.tr("持仓数量"), translator.tr("仓位(美元)"), translator.tr("开仓价"), translator.tr("标记价"), translator.tr("预估强平价"), translator.tr("保证金率"), translator.tr("未实现收益")]
                itemModel: (function(index, modelData) {
                    return !!modelData ? [modelData.ctime, modelData.inst_id, (modelData.mgn_mode === "cross") ? translator.tr("全仓") : translator.tr("逐仓"), _posSide(modelData) + "(" + modelData.lever + translator.tr("倍") + ")", utilityFn.toFixedPrice(modelData.notional_usd/modelData.mark_px) + "($" + Number(modelData.notional_usd).toFixed(0) + ")", utilityFn.toFixedPrice(modelData.margin), utilityFn.toFixedPrice(modelData.avg_px), utilityFn.toFixedPrice(modelData.mark_px), utilityFn.toFixedPrice(modelData.liq_px), utilityFn.toPercentString(Number(modelData.mgn_ratio) * 100), utilityFn.toFixedPrice(modelData.upl) + "(" + (Number(modelData.upl_ratio) * 100).toFixed(0) + "%)"] : [];
                })
                itemTextColor: (function(modelData) {
                    return Number(modelData.upl) >= 0 ? theme.priceUpFontColor : theme.priceDownFontColor;
                })
            }

        }

        Base.ItemLabel {
            id: itemLabel

            anchors.horizontalCenter: parent.horizontalCenter
            textColor: okex_position_channel_model.iso_eq > 0 ? theme.priceUpFontColor : theme.priceDownFontColor
            text: translator.tr("更新时间") + ": " + okex_position_channel_model.utime + utilityFn.paddingSpace(4) + translator.tr("仓位(美元)") + ": " + Number(okex_position_channel_model.total_eq).toFixed(2) + utilityFn.paddingSpace(4) + translator.tr("未实现收益(美元)") + ": " + Number(okex_position_channel_model.iso_eq).toFixed(2) + "(" + utilityFn.toPercentString(okex_position_channel_model.total_eq === 0 ? 0 : 100 * okex_position_channel_model.iso_eq / okex_position_channel_model.total_eq) + ")"
        }

    }

}

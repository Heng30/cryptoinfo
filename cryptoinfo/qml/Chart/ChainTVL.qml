import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    anchors.fill: parent

    Base.TimeLineSeriesChart {
        id: chartView

        property bool _isAsBillionFlag: utilityFn.isAsBillion(chart_chain_tvl_model.max_tvl)

        function _update(count) {
            if (count <= 0)
                return ;

            chartView.series.clear();
            chartView.title = chart_chain_tvl_model.name_qml() + translator.tr("锁仓量");
            var firstItem = chart_chain_tvl_model.item_qml(0);
            var endItem = chart_chain_tvl_model.item_qml(count - 1);
            chartView.xMin = new Date(utilityFn.seconds2milliseconds(firstItem.second));
            chartView.xMax = new Date(utilityFn.seconds2milliseconds(endItem.second));
            for (var i = 0; i < count; i++) {
                var item = chart_chain_tvl_model.item_qml(i);
                var x = utilityFn.seconds2milliseconds(item.second);
                var y = chartView._isAsBillionFlag ? utilityFn.asBillion(item.tvl, 9) : utilityFn.asMillion(item.tvl, 9);
                series.append(new Date(x), y);
            }
        }

        function _calcValueXY(x, y) {
            let count = chart_chain_tvl_model.count;
            if (count <= 0)
                return ;

            var minSecond = chart_chain_tvl_model.item_qml(0).second;
            var maxSceond = chart_chain_tvl_model.item_qml(count - 1).second;
            if (maxSceond < minSecond)
                return ;

            var xSecond = minSecond + (x - plotArea.x) * (maxSceond - minSecond) / plotArea.width;
            xSecond = Math.floor(xSecond);
            var item = chart_chain_tvl_model.likely_item_qml(xSecond);
            if (item.second <= 0)
                return ;

            var yMaxTVL = chartView._isAsBillionFlag ? utilityFn.billionAsNum(chartView.yMax) : utilityFn.millionAsNum(chartView.yMax);
            chartView.posY = chartView.plotArea.height * (1 - item.tvl / yMaxTVL) + chartView.plotArea.y;
            return {
                "x": new Date(item.second * 1000),
                "y": item.tvl
            };
        }

        anchors.fill: parent
        xTitleText: translator.tr("时间")
        yTitleText: translator.tr("美元") + "(" + (utilityFn.isAsBillion(chart_chain_tvl_model.max_tvl) ? translator.tr("十亿") : translator.tr("百万")) + ")"
        yMax: utilityFn.asMillionOrBillion(chart_chain_tvl_model.max_tvl, 2) + 0.1
        calcValueXY: _calcValueXY
        isShowVDashLish: true
        Component.onCompleted: {
            chart_chain_tvl_model.updated.connect(function() {
                chartView._update(chart_chain_tvl_model.count);
            });
        }

        Timer {
            interval: 5000
            repeat: false
            running: true
            triggeredOnStart: false
            onTriggered: {
                chart_chain_tvl_model.set_name_qml("Chains");
                chart_chain_tvl_model.use_cache_data_qml();
                chartView._update(chart_chain_tvl_model.count);
            }
        }

    }

    Base.ComBox {
        id: comBox

        anchors.right: parent.right
        anchors.margins: theme.itemMargins * 4
        width: 100
        popupHeight: parent.height / 2
        onActivated: {
            if (chart_chain_tvl_model.is_updating_qml()) {
                msgTip.add(translator.tr("正在下载数据, 请等待!"), false);
                return;
            }
            chart_chain_tvl_model.set_name_qml(model[index]);
            chart_chain_tvl_model.use_cache_data_qml();
            chartView._update(chart_chain_tvl_model.count);
        }
        Component.onCompleted: {
            var _model = ["Chains"];
            for (var i = 0; i < chain_name_model.count; i++) {
                _model.push(chain_name_model.item_qml(i).name);
            }
            model = _model;
        }
    }

    FloatTip {
        visible: chartView.isMouseInPlotArea
        fontColor: theme.bgColor
        plotArea: chartView.plotArea
        timeLabelText: utility.utc_seconds_to_local_string_qml(chartView.valueX.getTime() / 1000, "%y-%m-%d")
        tvlLabelText: chartView._isAsBillionFlag ? utilityFn.toBillion(chartView.valueY, 2) : utilityFn.toMillion(chartView.valueY, 2)
    }

}

import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    anchors.fill: parent

    Base.TimeLineSeriesChart {
        id: chartView

        property bool _isAsBillionFlag: utilityFn.isAsBillion(defi_chain_tvl_model.max_tvl)

        function _update(count) {
            if (count <= 0)
                return ;

            chartView.series.clear();
            chartView.title = defi_chain_tvl_model.name + translator.tr("锁仓量");
            var firstItem = defi_chain_tvl_model.item(0);
            var endItem = defi_chain_tvl_model.item(count - 1);
            chartView.xMin = new Date(utilityFn.seconds2milliseconds(firstItem.second));
            chartView.xMax = new Date(utilityFn.seconds2milliseconds(endItem.second));
            for (var i = 0; i < count; i++) {
                var item = defi_chain_tvl_model.item(i);
                var x = utilityFn.seconds2milliseconds(item.second);
                var y = chartView._isAsBillionFlag ? utilityFn.asBillion(item.tvl, 9) : utilityFn.asMillion(item.tvl, 9);
                series.append(new Date(x), y);
            }
        }

        function _calcValueXY(x, y) {
            let count = defi_chain_tvl_model.count;
            if (count <= 0)
                return ;

            var minSecond = defi_chain_tvl_model.item(0).second;
            var maxSceond = defi_chain_tvl_model.item(count - 1).second;
            if (maxSceond < minSecond)
                return ;

            var xSecond = minSecond + (x - plotArea.x) * (maxSceond - minSecond) / plotArea.width;
            xSecond = Math.floor(xSecond);
            var item = defi_chain_tvl_model.likely_item_qml(xSecond);
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
        yTitleText: translator.tr("美元") + "(" + (utilityFn.isAsBillion(defi_chain_tvl_model.max_tvl) ? translator.tr("十亿") : translator.tr("百万")) + ")"
        yMax: utilityFn.asMillionOrBillion(defi_chain_tvl_model.max_tvl, 2) + 0.1
        calcValueXY: _calcValueXY
        isShowVDashLish: true

        Connections {
            function onUpdated() {
                chartView._update(defi_chain_tvl_model.count);
            }

            target: defi_chain_tvl_model
        }

        Timer {
            interval: 2000
            repeat: false
            running: true
            triggeredOnStart: false
            onTriggered: defi_chain_tvl_model.update_text_qml("Chains")
        }

    }

    Base.ComBox {
        id: comBox

        anchors.right: parent.right
        anchors.margins: theme.itemMargins * 4
        width: 100
        popupHeight: parent.height / 2
        onActivated: defi_chain_tvl_model.update_text_qml(model[index])
        Component.onCompleted: {
            var _model = ["Chains"];
            for (var i = 0; i < defi_chain_name_model.count; i++) {
                _model.push(defi_chain_name_model.item(i).name);
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

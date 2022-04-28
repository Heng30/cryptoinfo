import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: totalTVL

    anchors.fill: parent

    Base.TimeLineSeriesChart {
        id: chartView

        function _update(count) {
            if (count <= 0)
                return ;

            chartView.series.clear();
            var firstItem = defi_total_tvl_model.item(0);
            var endItem = defi_total_tvl_model.item(count - 1);
            chartView.xMin = new Date(utilityFn.seconds2milliseconds(firstItem.second));
            chartView.xMax = new Date(utilityFn.seconds2milliseconds(endItem.second));
            for (var i = 0; i < count; i++) {
                var item = defi_total_tvl_model.item(i);
                var x = utilityFn.seconds2milliseconds(item.second);
                var y = utilityFn.asBillion(item.tvl, 9);
                series.append(new Date(x), y);
            }
        }

        function _calcValueXY(x, y) {
            let count = defi_total_tvl_model.count;
            if (count <= 0)
                return ;

            var minSecond = defi_total_tvl_model.item(0).second;
            var maxSceond = defi_total_tvl_model.item(count - 1).second;
            if (maxSceond < minSecond)
                return ;

            var xSecond = minSecond + (x - plotArea.x) * (maxSceond - minSecond) / plotArea.width;
            xSecond = Math.floor(xSecond);
            var item = defi_total_tvl_model.likely_item(xSecond);
            if (item.second <= 0)
                return ;

            chartView.posY = chartView.plotArea.height * (1 - item.tvl / utilityFn.billionAsNum(chartView.yMax)) + chartView.plotArea.y;
            return {
                "x": new Date(item.second * 1000),
                "y": item.tvl
            };
        }

        anchors.fill: parent
        title: translator.tr("总锁仓量")
        xTitleText: translator.tr("时间")
        yTitleText: translator.tr("美元(十亿)")
        yMax: utilityFn.asBillion(defi_total_tvl_model.max_tvl, 0) + 10
        calcValueXY: _calcValueXY
        isShowVDashLish: true
        Component.onCompleted: _update(defi_total_tvl_model.count)

        Connections {
            function onUpdated() {
                _update(defi_total_tvl_model.count);
            }

            target: defi_total_tvl_model
        }

    }

    FloatTip {
        visible: chartView.isMouseInPlotArea
        fontColor: theme.bgColor
        plotArea: chartView.plotArea
        timeLabelText: utility.utc_seconds_to_local_string(chartView.valueX.getTime() / 1000, "%y-%m-%d")
        tvlLabelText: utilityFn.toBillion(chartView.valueY, 2)
    }

}

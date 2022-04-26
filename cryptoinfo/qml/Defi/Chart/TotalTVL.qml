import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Base.TimeLineSeries {
    id: totalTVL

    function _update(count) {
        if (count <= 0)
            return ;

        totalTVL.series.clear();
        var firstItem = defi_total_tvl_model.item(0);
        var endItem = defi_total_tvl_model.item(count - 1);
        totalTVL.xMin = new Date(utilityFn.seconds2milliseconds(firstItem.second));
        totalTVL.xMax = new Date(utilityFn.seconds2milliseconds(endItem.second));
        for (var i = 0; i < count; i++) {
            var item = defi_total_tvl_model.item(i);
            var x = utilityFn.seconds2milliseconds(item.second);
            var y = utilityFn.asBillion(item.tvl, 9);
            series.append(new Date(x), y);
        }
    }

    anchors.fill: parent
    title: translator.tr("总锁仓量")
    xTitleText: translator.tr("时间")
    yTitleText: translator.tr("美元(十亿)")
    yMax: utilityFn.asBillion(defi_total_tvl_model.max_tvl, 0) + 10
    Component.onCompleted: _update(defi_total_tvl_model.count)

    Connections {
        function onUpdated() {
            _update(defi_total_tvl_model.count);
        }

        target: defi_total_tvl_model
    }

}

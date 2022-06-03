import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.CPieChart {
    id: chartView

    function update() {
        var data = [];
        for (var i = 0; i < handbook_model.count; i++) {
            var stats = handbook_model.pie_chart_stats_qml(i);
            var statsList = stats.split(",");
            if (statsList.length !== 3)
                return ;

            var name = String(statsList[0]);
            var payment = Number(statsList[1]);
            var income = Number(statsList[2]);
            data.push({
                "label": name,
                "value": income
            });
        }
        chartView.add(data);
    }

    appTheme: main.theme
    onVisibleChanged: {
        if (!chartView.visible)
            return ;

        chartView.series.clear();
        chartView.update();
    }
    Component.onCompleted: chartView.update()
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtCharts 2.2
import "qrc:/res/qml/Base" as Base

Item {
    id: chartView

    function update() {
        chartViewLeftUp.series.clear();
        chartViewLeftDown.series.clear();
        chartViewRight.series.clear();
        if (contract_stats_model.win_lose_counts <= 0)
            return ;

        var data = [];
        var winCount = 0;
        var loseCount = 0;
        var winFloatValue = 0;
        var loseFloatValue = 0;
        for (var i = 0; i < contract_stats_model.count; i++) {
            var item = contract_stats_model.item_qml(i);
            if (i < 2) {
                winCount += item.win_lose_count;
                winFloatValue += item.float_value;
            } else {
                loseCount += item.win_lose_count;
                loseFloatValue += Math.abs(item.float_value);
            }
            if (Math.abs(item.float_value) > 0)
                data.push({
                "label": item.ctype,
                "value": Math.abs(item.float_value)
            });

        }
        if (winCount > 0 || loseCount > 0)
            chartViewLeftUp.add([{
            "label": translator.tr("盈利次数"),
            "value": winCount
        }, {
            "label": translator.tr("亏损次数"),
            "value": loseCount
        }]);

        if (winFloatValue > 0 || loseFloatValue > 0)
            chartViewLeftDown.add([{
            "label": translator.tr("盈利金额"),
            "value": winFloatValue
        }, {
            "label": translator.tr("亏损金额"),
            "value": loseFloatValue
        }]);

        chartViewRight.add(data);
    }

    Component.onCompleted: {
        chartView.update();
        contract_stats_model.updated.connect(function() {
            chartView.update();
        });
    }

    Row {
        anchors.fill: parent
        spacing: theme.itemSpacing

        Column {
            width: parent.width / 2
            height: parent.height

            Item {
                width: parent.width
                height: parent.height / 2

                Base.CPieChart {
                    id: chartViewLeftUp

                    width: Math.min(parent.width, parent.height)
                    height: width
                    anchors.centerIn: parent
                    appTheme: main.theme
                    showValueOnTip: false
                    legend.alignment: Qt.AlignBottom
                }

            }

            Item {
                width: parent.width
                height: parent.height / 2

                Base.CPieChart {
                    id: chartViewLeftDown

                    width: Math.min(parent.width, parent.height)
                    height: width
                    anchors.centerIn: parent
                    appTheme: main.theme
                    legend.alignment: Qt.AlignBottom
                }

            }

        }

        Item {
            width: parent.width / 2
            height: parent.height

            Base.CPieChart {
                id: chartViewRight

                width: Math.min(parent.width, parent.height)
                height: width
                anchors.centerIn: parent
                appTheme: main.theme
            }

        }

    }

}

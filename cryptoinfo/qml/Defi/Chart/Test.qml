import QtQuick 2.15
import QtCharts 2.2

ChartView {
    id: chartView

    anchors.fill: parent

    title: "ChartView"
    antialiasing: true
    backgroundColor: Qt.rgba(0, 0, 1, 0.1)
    animationOptions: ChartView.SeriesAnimations
    animationDuration: 1
    titleColor: Qt.rgba(0, 0, 0, 0.8)
    titleFont.bold: true
    titleFont.pointSize: 15
    legend.visible: false

    ValueAxis {
        id: myAxisX

        min: 0
        max: 6000
        tickCount: 20
        labelsColor: Qt.rgba(0, 0, 0, 0.9)
        labelsFont.pointSize: 13
        labelsFont.bold: true
        labelFormat: ' '
        color: Qt.rgba(0, 0, 1, 0.9)
    }

    ValueAxis {
        id: myAxisY

        min: 0
        max: 60000
        tickCount: 6
        labelsColor: Qt.rgba(0, 0, 0, 0.9)
        labelsFont.pointSize: 13
        labelsFont.bold: true
        labelFormat: '%d'
        color: Qt.rgba(0, 0, 1, 0.9)
    }

    LineSeries {
        id: lineSeries

        name: "LineSeries"
        axisX: myAxisX
        axisY: myAxisY
        color: Qt.rgba(1, 0, 0, 1)
        width: 1
    }

    Timer {
        property int current: 0
        property int xValue: 0
        readonly property var valueList: [10000, 15000, 25000, 30000, 35000, 40000, 45000, 49700, 49800, 49900, 49950, 50000, 49950, 49900, 49800, 49700, 45000, 40000, 35000, 30000, 25000, 15000, 10000]

        running: panel.checkedTabIndex === 1
        interval: chartView.animationDuration
        repeat: true
        onTriggered: {
            var y = valueList[current];
            current = current + 1;
            if (current >= valueList.length)
                current = 0;

            var x = xValue;
            xValue += 10;
            if (xValue >= myAxisX.max) {
                lineSeries.clear();
                xValue = 0;
                return ;
            }
            lineSeries.append(x, y);
        }
    }

}

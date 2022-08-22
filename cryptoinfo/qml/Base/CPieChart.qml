import QtQuick 2.15
import QtQuick.Controls 2.15
import QtCharts 2.2
import "qrc:/res/qml/Base" as Base

ChartView {
    id: chartView

    required property var appTheme
    property alias series: pieSeries
    property bool showValueOnTip: true

    function add(data) {
        for (var i = 0; i < data.length; i++) {
            var slice = pieSeries.append(data[i].label, data[i].value);
            slice.labelPosition = PieSlice.LabelInsideNormal;
            slice.label = data[i].label;
        }
        for (var i = 0; i < data.length; i++) {
            var slice = pieSeries.at(i);
            if (slice.angleSpan > 10)
                slice.labelVisible = true;

        }
    }

    function setTheme() {
        if (appTheme.darkTheme)
            chartView.theme = ChartView.ChartThemeDark;
        else
            chartView.theme = ChartView.ChartThemeBrownSand;
        chartView.backgroundColor = "transparent";
        chartView.plotAreaColor = "transparent";
    }

    legend.alignment: Qt.AlignRight
    legend.labelColor: appTheme.fontColor
    antialiasing: true
    Component.onCompleted: {
        setTheme();
        appTheme.themeSig.connect(setTheme);
    }

    Rectangle {
        id: floatTip

        visible: label.text.length > 0
        anchors.right: parent.right
        anchors.rightMargin: appTheme.itemMargins * 2
        width: label.width + appTheme.itemMargins
        height: label.height + appTheme.itemMargins
        color: appTheme.invertBgColor
        opacity: 0.7

        Label {
            id: label

            anchors.centerIn: parent
            color: appTheme.bgColor
            font.pixelSize: appTheme.fontPixelNormal
        }

    }

    PieSeries {
        id: pieSeries

        size: 1
        holeSize: 0.3
        onHovered: {
            if (state)
                label.text = slice.label + (chartView.showValueOnTip? " ($" + Number(slice.value).toFixed(0) : "") + utilityFn.paddingSpace(4) + Number(slice.percentage * 100).toFixed(2) + "%)";
            else
                slice.exploded = false;
        }
        onClicked: slice.exploded = true
    }

}

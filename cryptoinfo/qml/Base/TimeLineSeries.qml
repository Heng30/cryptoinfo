import QtQuick 2.15
import QtCharts 2.2

ChartView {
    id: chartView

    property var xMin: (new Date(0))
    property var xMax: (new Date(0))
    property real yMin: 0
    property real yMax: 0
    property string xTitleText: ""
    property string yTitleText: ""
    property var appTheme: window.theme
    property alias series: lineSeries
    property string timeFormat: "yy.MM.dd"

    implicitWidth: 100
    implicitHeight: 100
    antialiasing: true
    backgroundColor: "transparent"
    plotAreaColor: "transparent"
    titleColor: appTheme.fontColor
    titleFont.bold: true
    titleFont.pixelSize: appTheme.fontPixelNormal + 5
    legend.visible: false

    DateTimeAxis {
        id: axisX

        min: xMin
        max: xMax
        tickCount: 5
        gridVisible: false
        labelsColor: appTheme.fontColor
        labelsFont.pixelSize: appTheme.fontPixelNormal
        titleVisible: xTitleText.length > 0
        titleText: xTitleText
        format: timeFormat
        color: appTheme.invertBgColor
    }

    ValueAxis {
        id: axisY

        min: yMin
        max: yMax
        tickCount: 10
        gridVisible: false
        labelsColor: appTheme.fontColor
        labelsFont.pixelSize: appTheme.fontPixelNormal
        titleVisible: yTitleText.length > 0
        titleText: yTitleText
        labelFormat: '%d'
        color: appTheme.invertBgColor
    }

    LineSeries {
        id: lineSeries

        name: "TimeLineSeries"
        width: 1
        axisX: axisX
        axisY: axisY
        color: appTheme.lineSeriesColor
    }

}

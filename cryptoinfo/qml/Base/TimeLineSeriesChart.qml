import QtQuick 2.15
import QtCharts 2.2
import "qrc:/res/qml/Base" as Base

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
    property string yLabelFormat: "%.2f"
    property var calcValueXY: null
    property var valueX: new Date(0)
    property real valueY: 0
    property real posY: -1
    property bool isShowHDashLish: false
    property bool isShowVDashLish: false
    property bool isMouseInPlotArea: false
    property alias mouseX: mouseArea.mouseX
    property alias mouseY: mouseArea.mouseY

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
        labelFormat: chartView.yLabelFormat
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

    MouseArea {
        id: mouseArea

        function _isInPlotArea() {
            var minX = chartView.plotArea.x;
            var maxX = chartView.plotArea.x + chartView.plotArea.width;
            var minY = chartView.plotArea.y;
            var maxY = chartView.plotArea.y + chartView.plotArea.height;
            if ((minX < mouseX && mouseX < maxX) && (minY < mouseY && mouseY < maxY))
                return true;

            return false;

        }

        anchors.fill: parent
        hoverEnabled: true
        onPositionChanged: {
            isMouseInPlotArea = _isInPlotArea();

            if (isMouseInPlotArea && chartView.calcValueXY) {
                var valueXY = chartView.calcValueXY(mouseX, mouseY);
                chartView.valueX = valueXY.x;
                chartView.valueY = valueXY.y;
            }

            if (!isMouseInPlotArea)
                chartView.posY = -1;

            vDashLine.moved(mouseX);
            hDashLine.moved(mouseY);
        }
    }

    Base.DashLine {
        id: vDashLine

        color: appTheme.invertBgColor
        isVertical: true
        height: chartView.plotArea.height
        visible: isShowVDashLish && isMouseInPlotArea
        y: chartView.plotArea.y
    }

    Base.DashLine {
        id: hDashLine

        color: appTheme.invertBgColor
        isVertical: false
        width: chartView.plotArea.width
        visible: isShowHDashLish && isMouseInPlotArea
        x: chartView.plotArea.x
    }

    Rectangle {
        x: chartView.mouseX - width / 2
        y: chartView.posY - height / 2
        width: 10
        height: width
        radius: height / 2
        visible: chartView.posY >= 0
        color: appTheme.invertBgColor
    }
}

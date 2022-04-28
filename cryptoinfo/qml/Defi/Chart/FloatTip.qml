import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: floatTip

    property color fontColor: theme.fontColor
    property string timeLabelText: "N/A"
    property string tvlLabelText: "N/A"
    property var plotArea: null
    property bool _isEntered: false

    x: _isEntered ? plotArea.x + plotArea.width - theme.itemSpacing - width : plotArea.x + theme.itemSpacing
    y: chartView.plotArea.y + theme.itemSpacing
    implicitWidth: content.width + theme.itemMargins
    implicitHeight: content.height + theme.itemMargins
    color: theme.invertBgColor
    opacity: 0.7

    Column {
        id: content

        anchors.centerIn: parent
        spacing: theme.itemSpacing

        Label {
            id: timeLabel

            color: floatTip.fontColor
            font.pixelSize: theme.fontPixelNormal
            text: translator.tr("时间") + ": " + timeLabelText
        }

        Label {
            id: tvlLabel

            color: floatTip.fontColor
            font.pixelSize: theme.fontPixelNormal
            text: translator.tr("锁仓量") + ": " + tvlLabelText
        }

    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: floatTip._isEntered = !floatTip._isEntered
    }

}

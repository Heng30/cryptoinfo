import QtQuick 2.15
import QtQuick.Shapes 1.15

Shape {
    id: dashLine

    property alias color: shapePath.strokeColor
    property bool isVertical: true
    width: 1
    height: 1
    visible: true

    function moved(pos) {
        if (!dashLine.visible) return;
        if (dashLine.isVertical)
            dashLine.x = pos;
        else
            dashLine.y = pos;
    }

    ShapePath {
        id: shapePath

        strokeColor: theme.invertBgColor
        strokeWidth: dashLine.isVertical ? dashLine.width : dashLine.height
        strokeStyle: ShapePath.DashLine
        startX: 0
        startY: 0

        PathLine {
            x: dashLine.isVertical ? 0 : dashLine.width
            y: dashLine.isVertical ? dashLine.height : 0
        }

    }

}

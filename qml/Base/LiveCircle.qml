import QtQuick 2.15
import QtGraphicalEffects 1.12

Item {
    id: root

    property real _maxWidth: Math.min(width, height)
    property int interval: timer.interval
    property alias circleColor: circle.color
    property alias circleOpacity: circle.opacity
    property alias blur: fastBlur.visible

    implicitWidth: 100
    implicitHeight: 100

    Rectangle {
        id: circle

        anchors.centerIn: parent
        width: 0
        height: width
        color: theme.invertBgColor
        radius: width / 2
    }

    FastBlur {
        id: fastBlur

        visible: false
        anchors.fill: circle
        source: circle
        radius: circle.radius
        transparentBorder: true
    }

    Timer {
        id: timer

        interval: 50
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: circle.width = (circle.width + 1) % root._maxWidth
    }

}

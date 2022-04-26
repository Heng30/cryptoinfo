import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: splash

    anchors.fill: parent
    color: theme.bgColor

    Image {
        id: image

        anchors.fill: parent
        source: "qrc:/res/image/splash.png"
    }

    Rectangle {
        id: progressBar

        property real maxWidth: parent.width

        anchors.bottom: parent.bottom
        width: Math.min(timer.intervalCount * timer.interval / config.splash_interval, 1) * maxWidth
        height: 10
        color: theme.splashBarColor
    }

    Timer {
        id: timer

        property int intervalCount: 0

        interval: 10
        repeat: true
        running: main._isPowerOn
        triggeredOnStart: true
        onTriggered: {
            intervalCount += 1;
            if (!config.show_splash || interval * intervalCount > config.splash_interval) {
                _isPowerOn = false;
                main.close();
            }
        }
    }

}

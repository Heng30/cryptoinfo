import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: splash

    width: theme.splashWitdh
    height: theme.splashHeight
    color: theme.bgColor
    visible: true

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
        running: splash.visible
        triggeredOnStart: true
        onTriggered: {
            intervalCount += 1;
            if (!config.show_splash || interval * intervalCount > config.splash_interval) {
                // 这里使用调用一次，会有窗口高度为开机动画页面高度的可能。
                // 调用三次保证main.height会被触发
                splash.visible = !splash.visible;
                splash.visible = !splash.visible;
                splash.visible = !splash.visible;
            }
        }
    }

}

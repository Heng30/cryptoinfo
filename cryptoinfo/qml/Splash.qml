import QtQuick 2.15
import QtMultimedia 5.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: splash

    width: theme.splashWitdh
    height: theme.splashHeight
    color: theme.bgColor
    Component.onCompleted: visible = config.show_splash

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

    MediaPlayer {
        id: playMusic

        source: "qrc:/res/sound/splash.wav"
        autoPlay: config.show_splash && config.use_splash_sound
        loops: 0
        volume: 0.5
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
                splash.visible = false;
                playMusic.stop();
            }
        }
    }

}

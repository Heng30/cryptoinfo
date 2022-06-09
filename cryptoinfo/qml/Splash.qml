import QtQuick 2.15
import QtQuick.Controls 2.15
import QtMultimedia 5.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: splash

    function showLogin() {
        splash.visible = true;
        login.visible = true;
        login.canLogin = false;
    }

    width: theme.splashWitdh
    height: content.height
    color: theme.bgColor
    Component.onCompleted: visible = (config.show_splash || config.enable_login_password)

    Column {
        id: content

        width: parent.width

        Image {
            id: image

            width: parent.width
            height: login.visible ? theme.splashHeight * 4 / 5 : theme.splashHeight
            fillMode: login.visible ? Image.PreserveAspectCrop : Image.Stretch
            source: "qrc:/res/image/splash.png"
        }

        Base.Sep {
            width: parent.width
            height: 2
            color: theme.darkTheme ? Qt.darker("lightgray") : "lightgray"
            visible: login.visible
        }

        Login {
            id: login

            width: parent.width
        }

        Rectangle {
            id: progressBar

            property real maxWidth: parent.width

            width: Math.min(0.001 + timer.intervalCount * timer.interval / config.splash_interval, 1) * maxWidth
            height: 10
            color: theme.splashBarColor
        }

    }

    MediaPlayer {
        id: playMusic

        source: "qrc:/res/sound/login.wav"
        autoLoad: true
        volume: 0.5
    }

    Timer {
        id: timer

        property int intervalCount: 0

        interval: 10
        repeat: true
        running: splash.visible && login.canLogin
        triggeredOnStart: true
        onTriggered: {
            intervalCount += 1;
            if (intervalCount === 1 && config.use_login_sound && login.visible)
                playMusic.play();

            if (interval * intervalCount > config.splash_interval) {
                splash.visible = false;
                intervalCount = 0;
                playMusic.stop();
            }
        }
    }

}

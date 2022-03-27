import QtQuick 2.15
import QtQuick.Window 2.15
import "./Base" as Base

Window {
    id: window

    property var prevMouse: ({
        "x": null,
        "y": null
    })
    property bool _rightButtonClicked: false
    property bool _isShowSettingPanel: false
    property bool isShowPopupWindow: false

    function rightClicked() {
        window._isShowSettingPanel = !window.isShowPopupWindow || (window.isShowPopupWindow && !window._isShowSettingPanel);
        popupPanel.showSettingPanel(window._isShowSettingPanel);
    }

    function leftClicked() {
    }

    function doubleClicked() {
        if (popupPanel.isOpen())
            window.isShowPopupWindow = false;
        else
            window.isShowPopupWindow = true;
        windowMouseArea.released(null);
    }

    function hideIntoEdge() {
        if (window.x == 0)
            window.x = -window.width / 2;

        if (window.y == 0)
            window.y = -window.height / 2;

    }

    function showFromEdge() {
        if (window.x < 0)
            window.x = 0;

        if (window.y < 0)
            window.y = 0;

    }

    function showWhenEntered() {
        if (window.x <= 0 || window.x >= Screen.desktopAvailableWidth - window.width)
            window.isShowPopupWindow = true;

    }

    x: theme.startupX
    y: theme.startupY
    width: theme.windowWidth
    height: width
    visible: true
    flags: Qt.Dialog | Qt.FramelessWindowHint | Qt.NoDropShadowWindowHint
    title: "cryptoinfo"
    color: "transparent"

    Base.Theme {
        id: theme
    }

    ShortKey {
        id: shortKey
    }

    Connecter {
        id: connecter
    }

    UtilityFn {
        id: utilityFn
    }

    PopupPanel {
        id: popupPanel

        visible: window.isShowPopupWindow
    }

    Base.LiveCircle {
        id: liveCircle

        z: circle.z + 1
        anchors.fill: parent
        visible: config.show_live_circle
        opacity: theme.liveCircleOpacity
        blur: true
    }

    Rectangle {
        id: circle

        property bool _entered: false

        anchors.fill: parent
        color: theme.bgColor
        radius: width / 2
        opacity: _entered ? theme.enteredOpacity : theme.exitedOpacity
        border.color: _entered ? theme.windowBorderEnterColor : "transparent"
        border.width: _entered ? 1 : 0

        MouseArea {
            id: windowMouseArea

            anchors.fill: parent
            hoverEnabled: true
            acceptedButtons: Qt.LeftButton | Qt.RightButton
            onEntered: {
                parent._entered = true;
                window.showFromEdge();
                window.showWhenEntered();
            }
            onExited: {
                parent._entered = false;
                window.hideIntoEdge();
            }
            onPressed: {
                if (mouse.button === Qt.RightButton)
                    window._rightButtonClicked = true;

                window.prevMouse.x = mouse.x;
                window.prevMouse.y = mouse.y;
            }
            onPositionChanged: {
                if (window._rightButtonClicked)
                    return ;

                if (window.prevMouse.x === null || window.prevMouse.y === null)
                    return ;

                window.x = window.x + (mouse.x - window.prevMouse.x);
                window.y = window.y + (mouse.y - window.prevMouse.y);
                if (window.x < 0)
                    window.x = 0;

                if (window.y < 0)
                    window.y = 0;

                if (window.x > Screen.desktopAvailableWidth - window.width)
                    window.x = Screen.desktopAvailableWidth - window.width;

                if (window.y > Screen.desktopAvailableHeight - window.height)
                    window.y = Screen.desktopAvailableHeight - window.height;

            }
            onReleased: {
                window.prevMouse.x = null;
                window.prevMouse.y = null;
                window._rightButtonClicked = false;
            }
            onClicked: {
                if (mouse.button === Qt.RightButton)
                    window.rightClicked();

                if (mouse.button === Qt.LeftButton)
                    window.leftClicked();

            }
            onDoubleClicked: window.doubleClicked()
        }

    }

}

import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Item {
    anchors.fill: parent

    property var prevMouse: ({
        "x": null,
        "y": null
    })

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onPressed: {
            prevMouse.x = mouse.x;
            prevMouse.y = mouse.y;
        }
        onPositionChanged: {
            if (prevMouse.x === null || prevMouse.y === null)
                return ;

            main.x = main.x + (mouse.x - prevMouse.x);
            main.y = main.y + (mouse.y - prevMouse.y);
            if (main.x < 0)
                main.x = 0;

            if (main.y < 0)
                main.y = 0;

            if (main.x > Screen.desktopAvailableWidth - main.width)
                main.x = Screen.desktopAvailableWidth - main.width;

            if (main.y > Screen.desktopAvailableHeight - main.height)
                main.y = Screen.desktopAvailableHeight - main.height;

        }
        onReleased: {
            prevMouse.x = null;
            prevMouse.y = null;
        }

        onDoubleClicked: homePage.isMaxHeight = !homePage.isMaxHeight
    }

}

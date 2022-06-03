import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    required property var moveItem
    property real bgWidth: moveItem.parent.width
    property real bgHeight: moveItem.parent.height
    property var prevMouse: ({
        "x": null,
        "y": null
    })

    signal clicked()
    signal doubleClicked()

    MouseArea {
        anchors.fill: parent
        onPressed: {
            prevMouse.x = mouse.x;
            prevMouse.y = mouse.y;
        }
        onPositionChanged: {
            if (prevMouse.x === null || prevMouse.y === null)
                return ;

            moveItem.x = moveItem.x + (mouse.x - prevMouse.x);
            moveItem.y = moveItem.y + (mouse.y - prevMouse.y);
            if (moveItem.x < 0)
                moveItem.x = 0;

            if (moveItem.y < 0)
                moveItem.y = 0;

            if (moveItem.x > root.bgWidth - moveItem.width)
                moveItem.x = root.bgWidth - moveItem.width;

            if (moveItem.y > root.bgHeight - moveItem.height)
                moveItem.y = root.bgHeight - moveItem.height;

        }
        onReleased: {
            prevMouse.x = null;
            prevMouse.y = null;
        }
        onDoubleClicked: root.doubleClicked()
        onClicked: root.clicked()
    }

}

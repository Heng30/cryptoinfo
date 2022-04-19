import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: txtArea

    property alias text: area.text
    property alias areaBackground: area.background
    property alias areaHeight: flick.contentHeight

    property int scrollBarPolicy: ScrollBar.AlwaysOff

    signal saved()

    function forceFocus() {
        area.forceActiveFocus();
    }

    implicitWidth: 100
    implicitHeight: 100
    color: "transparent"
    border.width: 1
    border.color: "steelblue"
    clip: true

    Flickable {
        id: flick

        function ensureVisible(r) {
            if (contentX >= r.x)
                contentX = r.x;
            else if (contentX + width <= r.x + r.width)
                contentX = r.x + r.width - width;
            if (contentY >= r.y)
                contentY = r.y;
            else if (contentY + height <= r.y + r.height)
                contentY = r.y + r.height - height;
        }

        anchors.fill: parent
        contentWidth: width
        contentHeight: area.height
        anchors.margins: theme.itemMargins
        clip: true

        TextArea {
            id: area

            padding: 0
            rightPadding: vbar.width
            width: parent.width
            background: null
            selectByMouse: true
            wrapMode: TextEdit.Wrap
            mouseSelectionMode: TextEdit.SelectWords
            onCursorRectangleChanged: flick.ensureVisible(cursorRectangle)
            color: theme.fontColor
            font.pixelSize: theme.fontPixelNormal
        }

        ScrollBar.vertical: Base.SBar {
            id: vbar

            policy: txtArea.scrollBarPolicy
        }

    }

}

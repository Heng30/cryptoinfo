import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: rightField

    property int checkedIndex: -1

    function reload() {
    }

    implicitWidth: 100
    height: parent.height
    border.width: 1
    border.color: "steelblue"
    color: "transparent"

    Column {
        anchors.fill: parent

        RightHeaderBar {
            id: headerBar
        }

        ListView {
            id: listView

            width: parent.width - theme.itemMargins
            height: parent.height - headerBar.height - parent.spacing
            anchors.horizontalCenter: parent.horizontalCenter
            clip: true
            model: 30

            ScrollBar.vertical: Base.SBar {
                policy: ScrollBar.AlwaysOff
            }

            delegate: RightDItem {
            }

        }

    }

}

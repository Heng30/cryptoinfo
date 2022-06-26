import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: leftField

    property int checkedIndex: 0

    implicitWidth: 100
    height: parent.height
    border.width: 1
    border.color: "steelblue"
    color: "transparent"

    Column {
        anchors.fill: parent

        LeftHeaderBar {
            id: headerBar
        }

        ListView {
            id: listView

            width: parent.width - theme.itemMargins
            height: parent.height - headerBar.height - parent.spacing
            anchors.horizontalCenter: parent.horizontalCenter
            clip: true
            model: bookmark_model

            ScrollBar.vertical: Base.SBar {
                policy: ScrollBar.AlwaysOff
            }

            delegate: LeftDItem {
            }

        }

    }

}

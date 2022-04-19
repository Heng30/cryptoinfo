import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: todo

    width: parent.width
    implicitHeight: 100

    Column {
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Rectangle {
            width: parent.width
            height: parent.height - row.height - parent.spacing
            border.width: 1
            border.color: "steelblue"
            color: "transparent"

            ListView {
                id: listView

                anchors.fill: parent
                anchors.margins: theme.itemMargins
                spacing: theme.itemSpacing
                clip: true
                model: todo_model

                ScrollBar.vertical: Base.SBar {
                    policy: ScrollBar.AlwaysOff
                }

                delegate: DItem {
                }

            }

        }

        Row {
            id: row

            anchors.rightMargin: theme.itemMargins * 5
            width: parent.width - anchors.rightMargin
            height: addBtn.height + anchors.margins * 2
            spacing: theme.itemSpacing * 5
            layoutDirection: Qt.RightToLeft

            Base.TxtButton {
                id: addBtn

                text: translator.tr("添加")
                anchors.verticalCenter: parent.verticalCenter
                onClicked: {
                    todo_model.add(false, "");
                    listView.positionViewAtEnd();
                }
            }

        }

    }

}

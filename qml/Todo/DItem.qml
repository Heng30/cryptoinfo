import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: dItem

    property bool isFinished: modelData.is_finished
    property string text: modelData.text

    width: ListView.view.width
    height: content.height
    implicitHeight: 100
    border.width: 1
    border.color: theme.borderColor
    color: isFinished ? theme.todoItemBGColor : "transparent"

    Column {
        id: content

        width: parent.width
        spacing: theme.itemSpacing

        Base.TxtArea {
            id: txtArea

            height: Math.max(areaHeight, 50)
            width: parent.width
            text: modelData.text
            border.color: "transparent"
        }

        Row {
            property list<QtObject> btnModel

            anchors.leftMargin: theme.itemMargins * 2
            width: parent.width - anchors.leftMargin
            spacing: theme.itemSpacing * 2
            layoutDirection: Qt.RightToLeft
            btnModel: [
                QtObject {
                    property string text: dItem.isFinished ? translator.tr("未完成") : translator.tr("完成")
                    property var clicked: function() {
                        dItem.isFinished = !dItem.isFinished;
                        todo_model.set(index, dItem.isFinished, txtArea.text);
                        todo_model.save();
                    }
                },
                QtObject {
                    property string text: translator.tr("保存")
                    property var clicked: function() {
                        todo_model.set(index, dItem.isFinished, txtArea.text);
                        todo_model.save();
                    }
                },
                QtObject {
                    property string text: translator.tr("删除")
                    property var clicked: function() {
                        todo_model.remove_rows(index, 1);
                        todo_model.save();
                    }
                }
            ]

            Repeater {
                model: parent.btnModel

                delegate: Item {
                    width: btn.width
                    height: btn.height + theme.itemMargins * 2

                    Base.TxtButton {
                        id: btn

                        anchors.centerIn: parent
                        onClicked: modelData.clicked()
                        text: modelData.text

                        Rectangle {
                            anchors.bottom: parent.bottom
                            anchors.bottomMargin: theme.itemMargins * 2
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: parent.textWidth
                            height: 2
                            color: "red"
                            visible: index === 1 && txtArea.text !== dItem.text
                        }

                    }

                }

            }

        }

    }

}
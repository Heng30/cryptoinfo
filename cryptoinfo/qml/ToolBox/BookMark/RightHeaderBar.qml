import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: rhBar

    property int _checkedIndex: -1

    width: parent.width
    height: row.height + theme.itemMargins * 2
    color: theme.inputBarBgColor

    Row {
        id: row

        property list<QtObject> imageModel

        anchors.verticalCenter: parent.verticalCenter
        width: parent.width
        height: Math.max(btnField.height, name.height)
        spacing: theme.itemSpacing
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/add.png"
                property string tipText: translator.tr("添加")
                property bool checked: rhBar._checkedIndex === 0
                property var clicked: function() {
                    name.forceFocus();
                    if (rhBar._checkedIndex === 0)
                        rhBar._checkedIndex = -1;
                    else
                        rhBar._checkedIndex = 0;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/edit.png"
                property string tipText: translator.tr("编辑")
                property bool checked: rhBar._checkedIndex === 1
                property var clicked: function() {
                    name.forceFocus();
                    if (rhBar._checkedIndex === 1)
                        rhBar._checkedIndex = -1;
                    else
                        rhBar._checkedIndex = 1;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property bool checked: false
                property var clicked: function() {
                    rhBar._checkedIndex = -1;
                }
            }
        ]

        Row {
            id: btnField

            spacing: theme.itemSpacing
            anchors.verticalCenter: parent.verticalCenter

            Repeater {
                model: row.imageModel

                delegate: Base.ImageButton {
                    anchors.margins: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    height: 32 - anchors.margins * 2
                    width: height
                    source: modelData.source
                    tipText: modelData.tipText
                    checked: modelData.checked
                    onClicked: modelData.clicked()
                }

            }

        }

        Base.InputBar {
            id: name

            anchors.verticalCenter: parent.verticalCenter
            width: (parent.width - btnField.width - parent.spacing) * 0.4
            underText: translator.tr("请输入名称")
            visible: rhBar._checkedIndex === 0 || rhBar._checkedIndex === 1
        }

        Base.InputBar {
            id: url

            anchors.verticalCenter: parent.verticalCenter
            width: parent.width - btnField.width - parent.spacing * 2 - name.width
            underText: translator.tr("请输入网址")
            visible: name.visible
        }

    }

}

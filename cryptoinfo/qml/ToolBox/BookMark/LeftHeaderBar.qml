import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: lhBar

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
                property bool checked: lhBar._checkedIndex === 0
                property var clicked: function() {
                    name.forceFocus();
                    name.text = "";
                    if (lhBar._checkedIndex === 0)
                        lhBar._checkedIndex = -1;
                    else
                        lhBar._checkedIndex = 0;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/edit.png"
                property string tipText: translator.tr("编辑")
                property bool checked: lhBar._checkedIndex === 1
                property var clicked: function() {
                    name.forceFocus();
                    if (lhBar._checkedIndex === 1) {
                        lhBar._checkedIndex = -1;
                    } else {
                        lhBar._checkedIndex = 1;
                        if (leftField.checkedIndex >= 0)
                            name.text = bookmark_model.item(leftField.checkedIndex).name;

                    }
                }
            },
            QtObject {
                property string source: "qrc:/res/image/up.png"
                property string tipText: translator.tr("上移")
                property bool checked: false
                property var clicked: function() {
                    if (leftField.checkedIndex <= 0)
                        return ;

                    bookmark_model.up_item_qml(leftField.checkedIndex);
                    bookmark_model.save();
                    leftField.checkedIndex -= 1;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/down.png"
                property string tipText: translator.tr("下移")
                property bool checked: false
                property var clicked: function() {
                    if (leftField.checkedIndex >= bookmark_model.count - 1)
                        return ;

                    bookmark_model.down_item_qml(leftField.checkedIndex);
                    bookmark_model.save();
                    leftField.checkedIndex += 1;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property bool checked: false
                property var clicked: function() {
                    if (leftField.checkedIndex < 0)
                        return ;

                    msgBox.add(translator.tr("是否要删除!"), true, function() {
                        bookmark_model.remove_item_qml(leftField.checkedIndex);
                        bookmark_model.save();
                        leftField.checkedIndex -= 1;
                    }, function() {
                    });
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
            width: parent.width - btnField.width - parent.spacing
            underText: translator.tr("请输入名称")
            visible: lhBar._checkedIndex === 0 || lhBar._checkedIndex === 1
            onAccepted: {
                if (name.text.length <= 0)
                    return ;

                if (lhBar._checkedIndex === 0)
                    bookmark_model.add_item_qml(name.text);
                else if (lhBar._checkedIndex === 1)
                    bookmark_model.set_item_qml(leftField.checkedIndex, name.text);
                bookmark_model.save();
                name.text = "";
            }
        }

    }

}

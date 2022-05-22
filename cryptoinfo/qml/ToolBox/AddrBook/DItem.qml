import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    width: ListView.view.width
    height: row.height + theme.itemMargins * 2
    color: mouseArea._isEntered ? theme.itemEnterColor : "transparent"
    border.width: 1
    border.color: theme.borderColor
    radius: theme.itemRadius

    Row {
        id: row

        anchors.centerIn: parent
        spacing: theme.itemSpacing
        width: parent.width - anchors.margins * 2

        Base.ItemLabel {
            text: modelData.name
            anchors.verticalCenter: parent.verticalCenter
            label.width: parent.width - btnRow.width - parent.spacing
            wrapMode: Text.Wrap

            MouseArea {
                id: mouseArea

                property bool _isEntered: false

                anchors.fill: parent
                hoverEnabled: true
                onEntered: _isEntered = true
                onExited: _isEntered = false
                onDoubleClicked: qrcode.show(index, modelData.name, modelData.addr, modelData.image_path)
            }

        }

        Row {
            id: btnRow

            property list<QtObject> imageModel

            anchors.verticalCenter: parent.verticalCenter
            spacing: theme.itemSpacing
            imageModel: [
                QtObject {
                    property string source: "qrc:/res/image/qrcode.png"
                    property string tipText: translator.tr("二维码")
                    property var clicked: function() {
                        qrcode.show(index, modelData.name, modelData.addr, modelData.image_path);
                    }
                },
                QtObject {
                    property string source: "qrc:/res/image/copy.png"
                    property string tipText: translator.tr("复制地址")
                    property var clicked: function() {
                        if (utility.copy_to_clipboard(modelData.addr))
                            msgTip.add(translator.tr("复制成功"), false);
                        else
                            msgTip.add(translator.tr("复制失败"), false);
                    }
                },
                QtObject {
                    property string source: "qrc:/res/image/edit.png"
                    property string tipText: translator.tr("编辑")
                    property var clicked: function() {
                        addItem.edit(index, modelData.name, modelData.addr);
                    }
                },
                QtObject {
                    property string source: "qrc:/res/image/clear.png"
                    property string tipText: translator.tr("删除")
                    property var clicked: function() {
                        msgBox.add(translator.tr("确定删除"), true, function() {
                            addrbook_model.remove_item_qml(index);
                            addrbook_model.save();
                        }, function() {
                        });
                    }
                }
            ]

            Repeater {
                model: parent.imageModel

                delegate: Base.ImageButton {
                    anchors.margins: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    height: 32 - anchors.margins * 2
                    width: height
                    source: modelData.source
                    tipText: modelData.tipText
                    onClicked: modelData.clicked()
                }

            }

        }

    }

}

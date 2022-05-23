import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: sdItem

    property bool _isSellIconChecked: modelData.is_sell
    property string _time: modelData.time
    property real _total_price: modelData.total_price
    property real _count: modelData.count

    width: parent.width
    height: row.height

    Row {
        id: row

        property list<QtObject> imageModel

        width: parent.width
        spacing: 0
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/up.png"
                property string tipText: translator.tr("上移")
                property var clicked: function() {
                    handbook_model.up_sub_model_item_qml(sItem._dItemIndex, index);
                    sItem.reload();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/down.png"
                property string tipText: translator.tr("下移")
                property var clicked: function() {
                    handbook_model.down_sub_model_item_qml(sItem._dItemIndex, index);
                    sItem.reload();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/save.png"
                property string tipText: translator.tr("保存")
                property var clicked: function() {
                    sdItem.forceActiveFocus(); // 为了触发txtField.editingFinished 信号
                    if (sdItem._time <= 0 || sdItem._total_price <= 0 || sdItem._count <= 0) {
                        msgTip.add(translator.tr("保存失败!"), true);
                        return ;
                    }
                    handbook_model.set_sub_model_item_qml(sItem._dItemIndex, index, sdItem._isSellIconChecked, sdItem._time, sdItem._total_price, sdItem._count);
                    handbook_model.save();
                    msgTip.add(translator.tr("保存成功!"), false);
                    sItem.reload();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property var clicked: function() {
                    msgBox.add(translator.tr("是否删除"), true, function() {
                        handbook_model.remove_sub_model_item_qml(sItem._dItemIndex, index);
                        handbook_model.save();
                        sItem.reload();
                    }, function() {
                    });
                }
            }
        ]

        Item {
            height: parent.height
            width: column._isSellIconSize

            Rectangle {
                width: Math.min(parent.height, parent.width) - theme.itemMargins * 4
                height: width
                anchors.centerIn: parent
                color: sdItem._isSellIconChecked ? theme.markedColor : theme.unmarkedColor
                radius: width / 2

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        sdItem._isSellIconChecked = !sdItem._isSellIconChecked;
                    }
                }

            }

        }

        Repeater {
            model: [modelData.time, utilityFn.toFixedPrice(modelData.total_price), modelData.count.toFixed(2), modelData.count <= 0 ? "N/A" : utilityFn.toFixedPrice(modelData.total_price / modelData.count)]

            delegate: Item {
                height: txtField.height + theme.itemMargins * 2
                width: column._itemWidth - 2 // 为什么-2?

                Base.TxtField {
                    id: txtField

                    anchors.centerIn: parent
                    horizontalAlignment: TextInput.AlignHCenter
                    height: theme.fontPixelNormal + theme.itemMargins * 2
                    width: parent.width - theme.itemMargins
                    text: modelData
                    readOnly: index === 3
                    onEditingFinished: {
                        focus = false;
                        if (index === 0)
                            sdItem._time = text;
                        else if (index === 1)
                            sdItem._total_price = Number(text);
                        else if (index === 2)
                            sdItem._count = Number(text);
                    }
                }

            }

        }

        Repeater {
            model: parent.imageModel

            delegate: Item {
                height: imgBtn.height
                width: height
                anchors.verticalCenter: parent.verticalCenter

                Base.ImageButton {
                    id: imgBtn

                    anchors.centerIn: parent
                    height: sItem._imageIconSize
                    width: height
                    source: modelData.source
                    tipText: modelData.tipText
                    onClicked: modelData.clicked()
                }

            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: dItem

    property string _time: modelData.time
    property real _crypto: modelData.crypto
    property real _stock: modelData.stock
    property real _saving: modelData.saving
    property real _other: modelData.other

    width: listView.width
    height: row.height

    Row {
        id: row

        property list<QtObject> imageModel

        width: parent.width
        spacing: 0
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/up-join.png"
                property string tipText: translator.tr("向上合并")
                property var clicked: function() {
                    if (index === 0) {
                        msgTip.add(translator.tr("没有可以向上合并的条目"), false);
                        return ;
                    }
                    msgBox.add(translator.tr("是否合并"), true, function() {
                        if (fundbook_model.up_join_item_qml(index))
                            fundbook_model.save_qml();
                        else
                            msgTip.add(translator.tr("合并失败! 原因：买卖类型不一致"), true);
                    }, function() {
                    });
                }
            },
            QtObject {
                property string source: "qrc:/res/image/save.png"
                property string tipText: translator.tr("保存")
                property var clicked: function() {
                    dItem.forceActiveFocus();
                    fundbook_model.set_item_qml(index, dItem._time, dItem._crypto, dItem._stock, dItem._saving, dItem._other);
                    fundbook_model.save_qml();
                    footer.updateText();
                    msgTip.add(translator.tr("保存成功!"), false);
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property var clicked: function() {
                    msgBox.add(translator.tr("是否删除"), true, function() {
                        fundbook_model.remove_item_qml(index);
                        fundbook_model.save_qml();
                        footer.updateText();
                    }, function() {
                    });
                }
            }
        ]

        Repeater {
            model: [modelData.time, utilityFn.toFixedPrice(modelData.crypto), utilityFn.toFixedPrice(modelData.stock), utilityFn.toFixedPrice(modelData.saving), utilityFn.toFixedPrice(modelData.other)]

            delegate: Item {
                height: txtField.height + theme.itemMargins * 2
                width: fundbook._itemWidth

                Base.TxtField {
                    id: txtField

                    anchors.centerIn: parent
                    horizontalAlignment: TextInput.AlignHCenter
                    height: theme.fontPixelNormal + theme.itemMargins * 2
                    width: parent.width - theme.itemMargins * 2
                    text: modelData
                    onEditingFinished: {
                        focus = false;
                        if (index === 0) {
                            dItem._time = text;
                        } else {
                            if (isNaN(Number(text)))
                                text = "0";

                            if (index === 1)
                                dItem._crypto = Number(text);
                            else if (index === 2)
                                dItem._stock = Number(text);
                            else if (index === 3)
                                dItem._saving = Number(text);
                            else if (index === 4)
                                dItem._other = Number(text);
                        }
                    }
                }

            }

        }

        Repeater {
            model: parent.imageModel

            delegate: Base.ImageButton {
                anchors.verticalCenter: parent.verticalCenter
                height: fundbook._imageIconSize
                width: height
                source: modelData.source
                tipText: modelData.tipText
                onClicked: modelData.clicked()
            }

        }

    }

}

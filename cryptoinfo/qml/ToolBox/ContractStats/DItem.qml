import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: dItem

    property bool _isEntered: false
    property bool _isEditing: false
    property string _ctype: modelData.ctype
    property int _win_lose_count: modelData.win_lose_count
    property real _win_precent: contract_stats_model.win_lose_counts <= 0 ? 0 : modelData.win_lose_count * 100 / contract_stats_model.win_lose_counts
    property real _float_value: modelData.float_value
    property int _index: index
    property color _textColor: index < 2 ? theme.priceUpFontColor : theme.priceDownFontColor

    width: parent.width
    height: row.height + theme.itemMargins * 2
    color: _isEntered ? theme.itemEnteredBG : "transparent"

    Row {
        id: row

        property list<QtObject> imageModel

        anchors.verticalCenter: parent.verticalCenter
        width: parent.width
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/like.png"
                property string tipText: translator.tr("增加")
                property var clicked: function() {
                    msgBox.add(translator.tr("确定增加盈亏次数?"), false, function() {
                        dItem._win_lose_count += 1;
                    }, function() {
                    });
                }
            },
            QtObject {
                property string source: "qrc:/res/image/unlike.png"
                property string tipText: translator.tr("减少")
                property var clicked: function() {
                    msgBox.add(translator.tr("确定减少盈亏次数?"), false, function() {
                        dItem._win_lose_count -= 1;
                    }, function() {
                    });
                }
            },
            QtObject {
                property string source: "qrc:/res/image/edit.png"
                property string tipText: translator.tr("编辑")
                property bool checked: dItem._isEditing
                property var clicked: function() {
                    dItem._isEditing = !dItem._isEditing;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/save.png"
                property string tipText: translator.tr("保存")
                property var clicked: function() {
                    dItem._isEditing = false;
                    dItem.forceActiveFocus();
                    contract_stats_model.set_item_qml(index, dItem._ctype, dItem._win_lose_count, dItem._float_value);
                    contract_stats_model.save_qml();
                    msgTip.add(translator.tr("保存成功!"), false);
                }
            }
        ]

        Row {
            Repeater {
                model: [dItem._ctype, dItem._win_lose_count, utilityFn.toPercentString(dItem._win_precent)]

                Base.ItemText {
                    anchors.verticalCenter: parent.verticalCenter
                    width: root.headerItemWidth
                    text: modelData
                    textColor: dItem._textColor
                    onIsEnteredChanged: dItem._isEntered = isEntered
                }

            }

            Repeater {
                model: [utilityFn.prettyNumStr(Math.abs(dItem._float_value).toFixed(2)), 0]

                delegate: Item {
                    anchors.verticalCenter: parent.verticalCenter
                    height: txtField.height
                    width: root.headerItemWidth

                    Base.TxtField {
                        id: txtField

                        anchors.centerIn: parent
                        horizontalAlignment: TextInput.AlignHCenter
                        height: theme.fontPixelNormal + theme.itemMargins * 2
                        width: parent.width - theme.itemMargins * 2
                        text: modelData
                        readOnly: index === 1 ? false : !dItem._isEditing
                        color: dItem._textColor
                        onEditingFinished: {
                            let num = Number(text);
                            if (isNaN(num) || num < 0) {
                                if (index === 1)
                                    text = 0;

                                msgTip.add(translator.tr("请输入正数!"), false);
                                return ;
                            }
                            if (num === 0)
                                return ;

                            if (index === 0) {
                                dItem._float_value = dItem._index < 2 ? num : -num;
                            } else if (index === 1) {
                                dItem._win_lose_count += 1;
                                if (dItem._index < 2)
                                    dItem._float_value += num;
                                else
                                    dItem._float_value -= num;
                            }
                        }
                    }

                }

            }

        }

        Row {
            id: btnField

            spacing: theme.itemSpacing
            anchors.verticalCenter: parent.verticalCenter

            Repeater {
                model: row.imageModel

                delegate: Base.ImageButton {
                    anchors.margins: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    height: root.iconSize - theme.itemSpacing * 2
                    width: height
                    checked: modelData.checked === undefined ? false : modelData.checked
                    source: modelData.source
                    tipText: modelData.tipText
                    onClicked: modelData.clicked()
                }

            }

        }

    }

}

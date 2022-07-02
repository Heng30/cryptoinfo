import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: dItem

    property bool _isEntered: false
    property bool _isChecked: index === rightField.checkedIndex

    width: ListView.view.width
    height: row.height + theme.itemMargins * 2
    color: _isChecked ? theme.itemCheckedBG : (_isEntered ? theme.itemEnteredBG : "transparent")

    Row {
        id: row

        property list<QtObject> imageModel

        function openInBrowser() {
            if (modelData.url.length <= 0) {
                msgTip.add(translator.tr("网址为空，无法打开!"), true);
                return ;
            }
            var ret = utility.process_cmd_qml(config.browser, modelData.url);
            msgTip.add(translator.tr("打开") + modelData.name + (ret ? translator.tr("成功") : translator.tr("失败")) + "!", !ret);
        }

        anchors.verticalCenter: parent.verticalCenter
        width: parent.width
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/browser.png"
                property string tipText: translator.tr("打开网址")
                property var clicked: row.openInBrowser
            },
            QtObject {
                property string source: "qrc:/res/image/copy.png"
                property string tipText: translator.tr("复制网址")
                property var clicked: function() {
                    utility.copy_to_clipboard_qml(modelData.url);
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property var clicked: function() {
                    msgBox.add(translator.tr("是否删除!"), true, function() {
                        bookmark_model.remove_sub_model_item_qml(leftField.checkedIndex, index);
                        bookmark_model.save_qml();
                        rightField.reload();
                    }, function() {
                    });
                }
            },
            QtObject {
                property string source: "qrc:/res/image/up.png"
                property string tipText: translator.tr("上移")
                property bool checked: false
                property var clicked: function() {
                    bookmark_model.up_sub_model_item_qml(leftField.checkedIndex, index);
                    bookmark_model.save_qml();
                    rightField.reload();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/down.png"
                property string tipText: translator.tr("下移")
                property bool checked: false
                property var clicked: function() {
                    bookmark_model.down_sub_model_item_qml(leftField.checkedIndex, index);
                    bookmark_model.save_qml();
                    rightField.reload();
                }
            }
        ]

        Base.ItemLabel {
            id: label

            width: parent.width - btnField.width
            anchors.verticalCenter: parent.verticalCenter
            text: modelData.name
            onEntered: dItem._isEntered = true
            onExited: dItem._isEntered = false
            onDoubleClicked: row.openInBrowser()
            onClicked: {
                if (headerBar._checkedIndex === 1) {
                    if (rightField.checkedIndex === index) {
                        headerBar.nameInput.text = "";
                        headerBar.urlInput.text = "";
                    } else {
                        headerBar.nameInput.text = modelData.name;
                        headerBar.urlInput.text = modelData.url;
                    }
                } else {
                    headerBar.nameInput.text = "";
                    headerBar.urlInput.text = "";
                }
                if (rightField.checkedIndex === index)
                    rightField.checkedIndex = -1;
                else
                    rightField.checkedIndex = index;
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

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: dItem

    function _level(value) {
        if (value === 0)
            return translator.tr("调试");
        else if (value === 1)
            return translator.tr("警告");
        else if (value === 2)
            return translator.tr("错误");
        else
            return translator.tr("未知");
    }

    width: listView.width
    height: row.height

    Row {
        id: row

        property list<QtObject> imageModel

        width: parent.width
        spacing: 0
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("删除")
                property var clicked: function() {
                    notify_model.remove_item_qml(index);
                }
            }
        ]

        Repeater {
            id: rep

            property int _mlevel: modelData.level

            model: [modelData.timestamp, modelData.module, _level(modelData.level), modelData.content]

            delegate: Base.ItemText {
                width: index < 3 ? notfiyPanel._itemWidth : notfiyPanel._contentWidth
                height: theme.fontPixelNormal + theme.itemMargins * 2
                label.elide: Text.ElideMiddle
                text: modelData
                tipText: index < 3 ? "" : modelData
                textColor: rep._mlevel === 0 ? theme.priceUpFontColor : theme.priceDownFontColor
            }

        }

        Repeater {
            model: parent.imageModel

            delegate: Base.ImageButton {
                anchors.verticalCenter: parent.verticalCenter
                height: notifyPanel._imageIconSize
                width: height
                source: modelData.source
                tipText: modelData.tipText
                onClicked: modelData.clicked()
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: notfiyPanel

    property real _imageIconSize: 32 - theme.itemMargins * 2
    property int _iconCount: 1
    property real _itemWidth: Math.max((notfiyPanel.width - _imageIconSize * _iconCount - theme.itemSpacing) * 0.1, 80)
    property real _contentWidth: notfiyPanel.width - _imageIconSize * _iconCount - _itemWidth * 3 - theme.itemSpacing * 3

    width: parent.width
    implicitHeight: 100

    Column {
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Row {
            id: row

            width: parent.width

            Repeater {
                model: [translator.tr("时间"), translator.tr("模块"), translator.tr("级别"), translator.tr("内容")]

                delegate: Base.ItemText {
                    width: index < 3 ? notfiyPanel._itemWidth : notfiyPanel._contentWidth
                    height: theme.fontPixelNormal + theme.itemMargins * 2
                    text: modelData
                }

            }

        }

        ListView {
            id: listView

            width: parent.width
            height: parent.height - row.height
            spacing: theme.itemSpacing
            clip: true
            model: notify_model

            ScrollBar.vertical: Base.SBar {
                policy: ScrollBar.AlwaysOff
            }

            delegate: DItem {
            }

        }

    }

    // Component.onCompleted: {
    //     for(var i = 0; i < 5; i++) {
    //         notify_model.add_item_qml(
    //             123232 * i,
    //             "foo-" + i,
    //             i % 5,
    //             "2321 323".repeat(10),
    //         );
    //     }
    // }

}

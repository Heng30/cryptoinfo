import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: row

    width: parent.width

    Item {
        width: itemRow.width
        height: itemRow.height

        Row {
            id: itemRow

            property real _itemWidth: row.width / repeater.model.length
            property color _textColor: itemPanel.itemTextColor(modelData)

            Repeater {
                id: repeater
                model: itemPanel.itemModel(index, modelData)

                Base.ItemText {
                    text: modelData
                    textColor: itemRow._textColor
                    width: itemRow._itemWidth
                    label.width: width - theme.itemSpacing * 2
                    label.elide: Text.ElideMiddle
                    tipText: (itemPanel.itemTipTextShowModel.length > index && !!itemPanel.itemTipTextShowModel[index]) ? text : ""
                    onIsEnteredChanged: bg._entered = isEntered
                    onClicked: {
                        if (!!itemPanel.itemTipTextShowModel[index])
                            utility.copy_to_clipboard_qml(text);

                    }
                }

            }

        }

        Rectangle {
            id: bg

            property bool _entered: false

            anchors.fill: parent
            color: _entered ? theme.itemEnterColor : "transparent"
            opacity: 0.5
        }

    }

}

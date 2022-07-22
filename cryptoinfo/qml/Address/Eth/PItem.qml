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

            property color _textColor: modelData.balance > 60000 ? theme.priceUpFontColor : theme.priceDownFontColor
            property real _itemWidth: row.width / repeater.model.length

            Repeater {
                id: repeater

                model: !!modelData ? [index + 1, modelData.address, utilityFn.prettyNumStr(modelData.balance.toFixed(0)), utilityFn.toPercentString(modelData.percentage * 100), utilityFn.prettyNumStr(modelData.transactions)] : []

                Base.ItemText {
                    text: modelData
                    textColor: itemRow._textColor
                    width: itemRow._itemWidth
                    label.width: width - theme.itemSpacing * 2
                    label.elide: Text.ElideMiddle
                    tipText: index === 1 ? text : ""
                    onIsEnteredChanged: bg._entered = isEntered
                    onClicked: {
                        if (index === 1)
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

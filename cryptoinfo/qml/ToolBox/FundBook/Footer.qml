import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Column {
    id: footer

    property int _spaceCount: 2

    function updateText() {
        var text = fundbook_model.stats_qml();
        var items = text.split(",");
        if (items.length !== 5)
            return ;

        if (Number(items[0]) <= 0) {
            itemLabel.text = "";
            return ;
        }
        itemLabel.text = translator.tr("总资金") + ": " + items[0] + utilityFn.paddingSpace(_spaceCount) + translator.tr("加密货币") + ": " + Number(items[1]).toFixed(0) + "(" + utilityFn.toPercentString(Number(items[1]) * 100 / Number(items[0])) + ")" + utilityFn.paddingSpace(_spaceCount) + translator.tr("股票基金") + ": " + Number(items[2]).toFixed(0) + "(" + utilityFn.toPercentString(Number(items[2]) * 100 / Number(items[0])) + ")" + utilityFn.paddingSpace(_spaceCount) + translator.tr("储蓄") + ": " + Number(items[3]).toFixed(0) + "(" + utilityFn.toPercentString(Number(items[3]) * 100 / Number(items[0])) + ")" + utilityFn.paddingSpace(_spaceCount) + translator.tr("其他") + ": " + Number(items[4]).toFixed(0) + "(" + utilityFn.toPercentString(Number(items[4]) * 100 / Number(items[0])) + ")";
    }

    width: parent.width
    Component.onCompleted: updateText()

    Row {
        anchors.rightMargin: theme.itemMargins
        width: parent.width - anchors.rightMargin
        layoutDirection: Qt.RightToLeft
        spacing: theme.itemSpacing * 2

        Base.TxtButton {
            id: addBtn

            text: translator.tr("添加")
            anchors.verticalCenter: parent.verticalCenter
            onClicked: fundbook_model.add_item_qml("", 0, 0, 0, 0)
        }

        Item {
            width: theme.itemSpacing * 8
            height: parent.height
        }

        Base.ItemLabel {
            id: itemLabel

            anchors.verticalCenter: parent.verticalCenter
        }

    }

}

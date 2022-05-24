import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Column {
    id: addItem

    property bool _isEditMode: false

    function edit(index, nameText) {
        addBtn.clicked(null);
        name.text = nameText;
        finishedBtn._editIndex = index;
        _isEditMode = true;
    }

    function updateBalance() {
        var balance = handbook_model.balance_qml();
        var blist = balance.split(",");
        if (blist.length !== 2)
            return ;

        var payment = Number(blist[0]);
        var income = Number(blist[1]);
        itemLabel.text = translator.tr("总支出") + ": " + utilityFn.toFixedPrice(payment) + utilityFn.paddingSpace(8) + translator.tr("总收入") + ": " + utilityFn.toFixedPrice(income) + utilityFn.paddingSpace(8) + (payment > income ? translator.tr("亏损") : translator.tr("盈利")) + ": " + utilityFn.toFixedPrice(Math.abs(payment - income)) + "(" + Math.abs((payment - income) * 100 / payment).toFixed(0) + "%)";
        itemLabel.textColor = payment > income ? theme.priceDownFontColor : theme.priceUpFontColor;
    }

    width: parent.width

    Row {
        anchors.horizontalCenter: parent.horizontalCenter
        width: parent.width
        visible: finishedBtn.visible
        spacing: theme.itemSpacing

        Base.ItemLabel {
            id: nameLabel

            anchors.verticalCenter: parent.verticalCenter
            text: translator.tr("名称") + ": "
        }

        Base.InputBar {
            id: name

            anchors.verticalCenter: parent.verticalCenter
            width: parent.width - nameLabel.width - parent.spacing
        }

    }

    Row {
        anchors.rightMargin: theme.itemMargins * 5
        width: parent.width - anchors.rightMargin
        layoutDirection: Qt.RightToLeft

        Base.TxtButton {
            id: addBtn

            text: translator.tr("添加")
            anchors.verticalCenter: parent.verticalCenter
            onClicked: {
                name.text = "";
                addBtn.visible = false;
                name.forceFocus();
            }
        }

        Base.TxtButton {
            id: finishedBtn

            property int _editIndex: 0

            text: translator.tr("完成")
            anchors.verticalCenter: parent.verticalCenter
            visible: !addBtn.visible
            onClicked: {
                addBtn.visible = true;
                if (name.text.length <= 0)
                    return ;

                if (!_isEditMode)
                    handbook_model.add_item_qml(name.text);
                else
                    handbook_model.set_item_qml(_editIndex, name.text);
                handbook_model.save();
                _isEditMode = false;
                handbook.addItemSig = !handbook.addItemSig;
            }
        }

        Item {
            width: addItem.width
            height: parent.height

            Base.ItemLabel {
                id: itemLabel

                anchors.centerIn: parent
            }

        }

    }

}

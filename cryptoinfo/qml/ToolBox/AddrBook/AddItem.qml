import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Column {
    id: addItem

    property bool _isEditMode: false

    function edit(index, nameText, addrText) {
        addBtn.clicked(null);
        name.text = nameText;
        addr.text = addrText;
        finishedBtn._editIndex = index;
        _isEditMode = true;
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
        anchors.horizontalCenter: parent.horizontalCenter
        width: parent.width
        visible: finishedBtn.visible
        spacing: theme.itemSpacing

        Base.ItemLabel {
            id: addrLabel

            anchors.verticalCenter: parent.verticalCenter
            text: translator.tr("地址") + ": "
        }

        Base.InputBar {
            id: addr

            anchors.verticalCenter: parent.verticalCenter
            width: parent.width - addrLabel.width - parent.spacing
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
                addr.text = "";
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
                if (name.text.length <= 0 || addr.text.length <= 0)
                    return ;

                if (!_isEditMode)
                    addrbook_model.add_item_qml(name.text, addr.text);
                else
                    addrbook_model.set_item_qml(_editIndex, name.text, addr.text);
                addrbook_model.save();
                _isEditMode = false;
            }
        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias model: comBox.model
    property alias labelText: label.text
    property alias labelWidth: label.width
    property alias text: txtField.text
    property alias txtFieldVisible: txtField.visible
    property alias boxCurrentIndex: comBox.currentIndex
    property alias validator: txtField.validator
    property real boxWidth: 50
    property real txtFieldWidth: 50
    property real itemSpacing: theme.itemSpacing
    property alias readOnly: txtField.readOnly

    signal boxActived(int index)
    signal editingFinished()
    signal textAccepted()

    width: row.width
    height: row.height

    Row {
        id: row

        height: comBox.height
        spacing: root.itemSpacing

        ItemLabel {
            id: label

            height: parent.height
            visible: text.length > 0
        }

        TxtField {
            id: txtField

            height: parent.height
            width: root.txtFieldWidth
            onEditingFinished: root.editingFinished()
            onAccepted: {
                txtField.focus = false;
                root.textAccepted()
            }
        }

        ComBox {
            id: comBox

            width: root.boxWidth
            onActivated: root.boxActived(index)
        }

    }

}

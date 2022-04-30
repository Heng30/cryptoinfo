import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias labelText: label.text
    property alias labelTipText: label.tipText
    property alias labelWidth: label.width
    property alias text: txtField.text
    property alias txtFieldVisible: txtField.visible
    property alias validator: txtField.validator
    property alias readOnly: txtField.readOnly
    property real txtFieldWidth: 50
    property real itemSpacing: theme.itemSpacing

    signal editingFinished()
    signal textAccepted()
    signal labelClicked()

    width: row.width
    height: row.height

    Row {
        id: row

        height: label.height
        spacing: root.itemSpacing

        ItemLabel {
            id: label

            anchors.verticalCenter: parent.verticalCenter
            textVMargins: theme.itemMargins
            visible: text.length > 0
            onClicked: root.labelClicked()
        }

        TxtField {
            id: txtField

            anchors.verticalCenter: parent.verticalCenter
            width: root.txtFieldWidth
            height: parent.height
            onEditingFinished: root.editingFinished()
            onAccepted: {
                txtField.focus = false;
                root.textAccepted()
            }
        }
    }

}

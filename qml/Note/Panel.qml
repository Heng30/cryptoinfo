import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: panel

    function forceFocus() {
        txtArea.forceFocus();
    }

    function saveText() {
        saveBtn.clicked(null);
    }

    width: parent.width
    implicitWidth: 100
    implicitHeight: 100
    Component.onCompleted: window.noteSaved.connect(saveText)

    Column {
        id: content

        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing * 2

        Base.TxtArea {
            id: txtArea

            width: parent.width
            height: parent.height - row.height - parent.spacing
            text: private_note.text
        }

        Row {
            id: row

            anchors.rightMargin: theme.itemMargins * 5
            width: parent.width - anchors.rightMargin
            spacing: theme.itemSpacing * 5
            layoutDirection: Qt.RightToLeft

            Base.TxtButton {
                id: saveBtn

                text: translator.tr("保存")
                onClicked: private_note.save(txtArea.text)

                Rectangle {
                    anchors.bottom: parent.bottom
                    anchors.bottomMargin: theme.itemMargins * 2
                    anchors.horizontalCenter: parent.horizontalCenter
                    width: parent.textWidth
                    height: 2
                    color: "red"
                    visible: txtArea.text !== private_note.text
                }

            }

            Base.TxtButton {
                height: saveBtn.height
                text: translator.tr("丢弃")
                onClicked: txtArea.text = private_note.text
            }

        }

    }

}

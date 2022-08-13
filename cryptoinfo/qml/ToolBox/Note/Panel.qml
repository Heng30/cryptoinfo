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

    Column {
        id: content

        property bool isEdited: false
        property string tmpText: private_note.text

        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing * 2

        Base.TxtArea {
            id: txtArea

            width: parent.width
            height: parent.height - row.height - parent.spacing
            text: private_note.text
            visible: content.isEdited
        }

        Base.TxtArea {
            id: txtAreaMD

            width: parent.width
            height: parent.height - row.height - parent.spacing
            text: private_note.text
            tarea.textFormat: TextEdit.MarkdownText
            readOnly: true
            visible: !content.isEdited
        }

        Row {
            id: row

            anchors.rightMargin: theme.itemMargins
            width: parent.width - anchors.rightMargin
            spacing: theme.itemSpacing * 5
            layoutDirection: Qt.RightToLeft

            Base.TxtButton {
                id: saveBtn

                text: translator.tr("保存")
                onClicked: {
                    private_note.save_qml(txtArea.text);
                    msgTip.add(translator.tr("保存成功!"), false);
                }

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

            Base.TxtButton {
                height: saveBtn.height
                text: content.isEdited ? translator.tr("预览") : translator.tr("编辑")
                onClicked: {
                    content.isEdited = !content.isEdited;
                    if (!content.isEdited) {
                        txtAreaMD.text = "";
                        txtAreaMD.text = txtArea.text;
                    } else {
                        msgTip.add(translator.tr("请用Markdown格式编辑"), false);
                    }
                }
            }

        }

    }

}

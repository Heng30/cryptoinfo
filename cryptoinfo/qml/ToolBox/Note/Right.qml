import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: right

    function forceFocus() {
        txtArea.forceFocus();
    }

    function switchFile(index) {
        if (txtArea.text === note_model.text) {
            leftField.checkedIndex = index;
            return ;
        }
        msgBox.add(translator.tr("文件没有保存，是否保存?"), true, function() {
            right.save();
            leftField.checkedIndex = index;
        }, function() {
            leftField.checkedIndex = index;
        });
    }

    function save() {
        note_model.save_qml(leftField.checkedIndex, txtArea.text);
        msgTip.add(translator.tr("保存成功!"), false);
    }

    function recovery() {
        txtArea.text = note_model.text;
    }

    function edit() {
        content.isEdited = true;
        msgTip.add(translator.tr("请用Markdown格式编辑"), false);
    }

    function reload() {
        content.isEdited = false;
        note_model.load_qml(leftField.checkedIndex);
        txtArea.text = note_model.text;
        txtAreaMD.text = note_model.text;
    }

    function viewMD() {
        content.isEdited = false;
        txtAreaMD.text = "";
        txtAreaMD.text = txtArea.text;
    }

    height: parent.height

    Column {
        id: content

        property bool isEdited: false
        property string tmpText: note_model.text

        anchors.fill: parent
        spacing: theme.itemSpacing

        Base.TxtArea {
            id: txtArea

            width: parent.width
            height: parent.height
            text: note_model.text
            visible: content.isEdited
            scrollBarPolicy: ScrollBar.AsNeeded
            border.color: text === note_model.text ? "steelblue" : "red"
        }

        Base.TxtArea {
            id: txtAreaMD

            width: parent.width
            height: parent.height
            text: note_model.text
            tarea.textFormat: TextEdit.MarkdownText
            readOnly: true
            visible: !content.isEdited
            scrollBarPolicy: ScrollBar.AsNeeded
            border.color: txtArea.text === note_model.text ? "steelblue" : "red"
        }

    }

}

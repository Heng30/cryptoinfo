import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: rightField

    property int checkedIndex: -1
    property var _model: []

    function reload() {
        rightField._model = [];
        if (leftField.checkedIndex >= 0) {
            var len = bookmark_model.sub_model_len_qml(leftField.checkedIndex);
            for (var i = 0; i < len; i++) {
                rightField._model[i] = bookmark_model.sub_model_item_qml(leftField.checkedIndex, i);
            }
        }
        listView.model = rightField._model;
        headerBar.clearInput();
    }

    implicitWidth: 100
    height: parent.height
    border.width: 1
    border.color: "steelblue"
    color: "transparent"

    Column {
        anchors.fill: parent

        RightHeaderBar {
            id: headerBar
        }

        ListView {
            id: listView

            width: parent.width - theme.itemMargins
            height: parent.height - headerBar.height - parent.spacing
            anchors.horizontalCenter: parent.horizontalCenter
            clip: true

            ScrollBar.vertical: Base.SBar {
                policy: ScrollBar.AlwaysOff
            }

            delegate: RightDItem {
            }

        }

    }

}

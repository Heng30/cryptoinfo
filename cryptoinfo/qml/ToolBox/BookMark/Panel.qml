import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: bookmark

    width: parent.width
    implicitHeight: 100

    Row {
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Left {
            id: leftField
            width: parent.width * 0.3
        }

        Right {
            id: rightField
            width: parent.width - leftField.width - parent.spacing;
        }
    }

}

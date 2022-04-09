import QtQuick 2.15
import QtQuick.Controls 2.15
import "./Base" as Base

Flickable {
    id: root
    width: parent.width
    implicitHeight: 100
    contentWidth: width
    contentHeight: content.height
    clip: true

    Column {
        id: content

        width: parent.width
        spacing: theme.itemSpacing
    }

}

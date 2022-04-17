import QtQuick 2.15
import QtQuick.Controls 2.15

ScrollBar {
    id: root

    property color contentItemColor: theme.scrollBarColor
    property real contentItemWidth: 6
    property real contentItemHeight: 100

    active: true
    orientation: Qt.Vertical
    policy: ScrollBar.AsNeeded

    contentItem: Rectangle {
        implicitWidth: root.contentItemWidth
        implicitHeight: root.contentItemHeight
        radius: width / 2
        color: root.contentItemColor
    }

}

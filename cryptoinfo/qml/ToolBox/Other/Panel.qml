import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: panel

    anchors.fill: parent
    implicitWidth: 100
    implicitHeight: 100

    Column {
        anchors.fill: parent
        spacing: theme.itemSpacing

        ExchangeRate {
        }
    }

}

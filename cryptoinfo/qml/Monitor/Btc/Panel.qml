import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    width: parent.width

    Column {
        anchors.fill: parent

        Header {
            id: header
        }

        ListView {
            id: listView

            clip: true
            model: monitor_btc_model
            width: parent.width
            height: parent.height - header.height

            delegate: DItem {
            }

        }

    }

}

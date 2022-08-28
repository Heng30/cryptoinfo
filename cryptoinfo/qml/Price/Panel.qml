import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    function viewAtBeginning() {
        listView.positionViewAtBeginning();
    }

    function viewAtEnd() {
        listView.positionViewAtEnd();
    }

    function viewAtIndex(index) {
        listView.positionViewAtIndex(index, ListView.Beginning);
    }

    width: parent.width

    Column {
        anchors.fill: parent

        Header {
            id: header
        }

        ListView {
            id: listView

            clip: true
            model: price_model
            width: parent.width
            height: parent.height - header.height
            maximumFlickVelocity: height

            ScrollBar.vertical: Base.SBar {
            }

            delegate: DItem {
            }

        }

    }

}

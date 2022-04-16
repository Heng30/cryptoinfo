import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property real _smallItemWidth: 65

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

    ListView {
        id: listView

        clip: true
        model: pricer_model
        anchors.fill: parent

        ScrollBar.vertical: Base.SBar {
        }

        header: Header {
        }

        delegate: DItem {
        }

    }

}

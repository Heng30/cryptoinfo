import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property bool _isPriceUpdate: false
    property real _smallItemWidth: 65

    function updatePrice() {
        const url = "https://api.alternative.me/v1/ticker/?limit=" + config.price_item_count;
        const Http = new XMLHttpRequest();
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function(e) {
            if (Http.responseText.length <= 0) {
                root._isPriceUpdate = false;
                return ;
            }
            pricer_model.update_all_price(Http.responseText);
            pricer_addtion.update_time();
            root._isPriceUpdate = true;
        };
    }

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

    Timer {
        interval: root._isPriceUpdate ? 1000 * config.price_refresh_interval : 1000 * 10
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: root.updatePrice()
    }

}

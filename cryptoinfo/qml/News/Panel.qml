import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    width: parent.width

    ListView {
        id: listView

        property var _refreshTime: Date.now()

        clip: true
        model: news_model
        anchors.fill: parent
        spacing: theme.itemSpacing
        anchors.margins: theme.itemMargins
        anchors.rightMargin: 0
        onContentYChanged: {
            if (contentY + listView.height >= contentHeight + originY) {
                if (Date.now() - _refreshTime > 5000) {
                    news_model.update_now = true;
                    _refreshTime = Date.now();
                }
            }
        }

        ScrollBar.vertical: Base.SBar {
        }

        delegate: DItem {
        }

    }

}

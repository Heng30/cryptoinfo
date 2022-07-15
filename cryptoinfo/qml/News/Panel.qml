import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    width: parent.width

    ListView {
        id: listView

        property var _refreshTime: Date.now()
        property bool _up_drag_refresh: false

        Component.onCompleted: {
            news_model.up_refresh_ok.connect(function() {
                if (!_up_drag_refresh)
                    return ;

                msgTip.add(translator.tr("刷新成功!"), false);
                _up_drag_refresh = false;
            });
        }
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
            } else if (contentY <= -200) {
                if (Date.now() - _refreshTime > 5000) {
                    msgTip.add(translator.tr("正在刷新, 请等待!"), false);
                    news_model.page_index = 1;
                    news_model.update_now = true;
                    _up_drag_refresh = true;
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

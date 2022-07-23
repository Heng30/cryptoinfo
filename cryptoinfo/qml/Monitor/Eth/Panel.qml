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

            property var _refreshTime: Date.now()
            property bool _up_drag_refresh: false

            Component.onCompleted: {
                monitor_eth_model.refresh_ok.connect(function() {
                    if (!_up_drag_refresh)
                        return ;

                    msgTip.add(translator.tr("刷新成功!"), false);
                    _up_drag_refresh = false;
                });
            }
            clip: true
            model: monitor_eth_model
            width: parent.width
            height: parent.height - header.height
            onContentYChanged: {
                if (contentY + listView.height >= contentHeight + originY) {
                    if (Date.now() - _refreshTime > 5000) {
                        monitor_eth_model.down_refresh_qml();
                        _refreshTime = Date.now();
                    }
                } else if (contentY <= -200) {
                    if (Date.now() - _refreshTime > 5000) {
                        msgTip.add(translator.tr("正在刷新, 请等待!"), false);
                        _up_drag_refresh = true;
                        _refreshTime = Date.now();
                        monitor_eth_model.up_refresh_qml();
                    }
                }
            }

            ScrollBar.vertical: Base.SBar {
            }

            delegate: DItem {
            }

        }

    }

}

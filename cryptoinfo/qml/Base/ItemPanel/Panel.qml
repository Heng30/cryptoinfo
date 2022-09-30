import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: itemPanel

    required property var listModel
    required property var headerSortKeyModel
    required property var headerModel
    required property var itemTipTextShowModel
    property alias headerBG: header.color
    property var itemWidthList: []
    property var itemModel: (function(index, modelData) {
        return [];
    })
    property var itemTextColor: (function(modelData) {
        return theme.fontColor;
    })

    property alias showSbar: sbar.visible

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
                if (!listModel.refresh_ok)
                    return ;

                listModel.refresh_ok.connect(function() {
                    if (!_up_drag_refresh)
                        return ;

                    msgTip.add(translator.tr("刷新成功!"), false);
                    _up_drag_refresh = false;
                });
            }
            clip: true
            model: listModel
            width: parent.width
            height: parent.height - header.height
            maximumFlickVelocity: height
            onContentYChanged: {
                if (contentY + listView.height >= contentHeight + originY) {
                    if (!listModel.down_refresh_qml)
                        return ;

                    if (Date.now() - _refreshTime > 3000) {
                        _refreshTime = Date.now();
                        listModel.down_refresh_qml();
                    }
                } else if (contentY <= -200) {
                    if (Date.now() - _refreshTime > 3000) {
                        if (!listModel.up_refresh_qml)
                            return ;

                        msgTip.add(translator.tr("正在刷新, 请等待!"), false);
                        _up_drag_refresh = true;
                        _refreshTime = Date.now();
                        listModel.up_refresh_qml();
                    }
                }
            }

            ScrollBar.vertical: Base.SBar {
                id: sbar
            }

            delegate: DItem {
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15
import "./Base" as Base

Rectangle {
    id: root

    property bool settingIschecked: false
    property bool _isGreedUpdate: false
    property bool _isSearchChecked: false

    signal refresh()
    signal editingFinished()

    function updateGreed() {
        const url = "https://api.alternative.me/fng/?limit=2";
        const Http = new XMLHttpRequest();
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function(e) {
            if (Http.responseText.length <= 0) {
                root._isGreedUpdate = false;
                return ;
            }
            pricer_addtion.update_greed(Http.responseText);
            root._isGreedUpdate = true;
        };
    }

    width: parent.width
    height: theme.popupPanelHeaderHeight
    color: theme.headerBG
    radius: theme.itemRadius

    Timer {
        interval: root._isGreedUpdate ? 1000 * 60 * 60 : 1000 * 60
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: root.updateGreed()
    }

    Shortcut {
        sequence: shortKey.pricePanelRefresh
        onActivated: root.refresh()
    }

    Row {
        property list<QtObject> imageModel

        anchors.left: parent.left
        height: parent.height
        anchors.leftMargin: theme.itemSpacing
        spacing: theme.itemSpacing
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/setting.png"
                property string tipText: translator.tr("设置")
                property bool visible: true
                property var clicked: function() {
                    root.settingIschecked = !root.settingIschecked;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/theme.png"
                property string tipText: translator.tr("皮肤")
                property bool visible: true
                property var clicked: function() {
                    config.is_dark_theme = !config.is_dark_theme;
                    config.save_config();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/max-height.png"
                property string tipText: translator.tr("缩放")
                property bool visible: true
                property var clicked: function() {
                    popupPanel.isPopupPanelMaxHeight = !popupPanel.isPopupPanelMaxHeight;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("清除")
                property bool visible: !root.settingIschecked
                property var clicked: function() {
                    pricer_model.clear();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/refresh.png"
                property string tipText: translator.tr("刷新")
                property bool visible: !root.settingIschecked
                property var clicked: function() {
                    root.refresh();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/search.png"
                property string tipText: translator.tr("搜索")
                property bool visible: !root.settingIschecked
                property var clicked: function() {
                    root._isSearchChecked = !root._isSearchChecked;
                    if (root._isSearchChecked)
                        searchBar.forceFocus();

                }
            }
        ]

        Repeater {
            model: parent.imageModel

            delegate: Item {
                height: parent.height
                width: height

                Base.ImageButton {
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.margins: theme.itemMargins + 1
                    height: parent.height - anchors.margins * 2
                    width: height
                    onClicked: modelData.clicked()
                    source: modelData.source
                    visible: modelData.visible
                    tipText: modelData.tipText
                }

            }

        }

    }

    Row {
        id: greed_rate

        anchors.centerIn: parent
        height: parent.height
        spacing: theme.itemSpacing
        visible: !root._isSearchChecked

        Repeater {
            model: [{
                "greed": pricer_addtion.greed_tody,
                "tipText": translator.tr("今天贪婪恐惧指数")
            }, {
                "greed": pricer_addtion.greed_yestoday,
                "tipText": translator.tr("昨天贪婪恐惧指数")
            }]

            Item {
                height: parent.height
                width: root.width / 8

                Base.ItemText {
                    anchors.centerIn: parent
                    text: modelData.greed
                    tipText: modelData.tipText
                }

            }

        }

    }

    Base.SearchBar {
        id: searchBar

        anchors.centerIn: parent
        width: root.width / 4
        height: parent.height / 4 * 3
        visible: root._isSearchChecked
        color: theme.searchBarColor
        Keys.onTabPressed: root._isSearchChecked = !root._isSearchChecked
        text: root._isSearchChecked ? text : ""
        onEditingFinished: {
            pricer_model.search_and_show_at_beginning(text)
            root.editingFinished();
        }

        Shortcut {
            sequence: shortKey.search
            onActivated: {
                root._isSearchChecked = true;
                searchBar.forceFocus();
            }
        }

    }

    Row {
        anchors.right: exitButton.left
        height: parent.height
        spacing: theme.itemSpacing

        Item {
            height: parent.height
            width: theme.itemSpacing
        }

        Item {
            height: parent.height
            width: timeLabel.width

            Base.ItemText {
                id: timeLabel

                anchors.verticalCenter: parent.verticalCenter
                text: pricer_addtion.system_time
                tipText: translator.tr("更新时间")
            }

        }

        Item {
            height: parent.height
            width: theme.itemSpacing
        }

    }

    Base.ImageButton {
        id: exitButton

        anchors.verticalCenter: parent.verticalCenter
        anchors.margins: theme.itemMargins
        anchors.right: parent.right
        height: parent.height - anchors.margins * 2
        width: height
        onClicked: popupPanel.close()
        source: "qrc:/res/image/exit.png"
        tipText: translator.tr("关闭")
    }

}

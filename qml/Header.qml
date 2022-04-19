import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: root

    property bool _settingIsChecked: config.panel_type === PanelType.Setting
    property bool _homeIsChecked: config.panel_type === PanelType.Price
    property bool _noteIsChecked: config.panel_type === PanelType.Note
    property bool _notifyIsChecked: config.panel_type === PanelType.Todo
    property bool _isSearchChecked: false

    signal refresh()
    signal searchEditingFinished()
    signal noteClicked()

    width: parent.width
    height: theme.popupPanelHeaderHeight
    color: theme.headerBG
    radius: theme.itemRadius

    Shortcut {
        sequence: shortKey.pricePanelRefresh
        onActivated: root.refresh()
    }

    Row {
        id: leftIconBtn

        property list<QtObject> imageModel

        anchors.left: parent.left
        height: parent.height
        anchors.leftMargin: theme.itemSpacing
        spacing: theme.itemSpacing
        imageModel: [
            QtObject {
                property string source: "qrc:/res/image/home.png"
                property string tipText: translator.tr("主页")
                property bool visible: !root._homeIsChecked
                property var clicked: function() {
                    config.panel_type = PanelType.Price;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/setting.png"
                property string tipText: translator.tr("设置")
                property bool visible: !root._settingIsChecked
                property var clicked: function() {
                    config.panel_type = PanelType.Setting;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/note.png"
                property string tipText: translator.tr("笔记")
                property bool visible: !root._noteIsChecked
                property var clicked: function() {
                    config.panel_type = PanelType.Note;
                    root.noteClicked();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/todo-list.png"
                property string tipText: translator.tr("代办事项")
                property bool visible: !root._notifyIsChecked
                property var clicked: function() {
                    config.panel_type = PanelType.Todo;
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
                    homePage.isPopupPanelMaxHeight = !homePage.isPopupPanelMaxHeight;
                }
            },
            QtObject {
                property string source: "qrc:/res/image/clear.png"
                property string tipText: translator.tr("清除")
                property bool visible: root._homeIsChecked
                property var clicked: function() {
                    pricer_model.clear();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/refresh.png"
                property string tipText: translator.tr("刷新")
                property bool visible: root._homeIsChecked
                property var clicked: function() {
                    root.refresh();
                }
            },
            QtObject {
                property string source: "qrc:/res/image/search.png"
                property string tipText: translator.tr("搜索")
                property bool visible: root._homeIsChecked
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
                visible: modelData.visible

                Base.ImageButton {
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.margins: theme.itemMargins + 1
                    height: parent.height - anchors.margins * 2
                    width: height
                    onClicked: modelData.clicked()
                    source: modelData.source
                    tipText: modelData.tipText
                }

            }

        }

    }

    Item {
        anchors.right: uptime.left
        anchors.rightMargin: theme.itemMargins
        height: parent.height
        width: parent.width / 6

        Row {
            id: greed_rate

            anchors.fill: parent
            height: parent.height
            width: parent.width
            spacing: theme.itemSpacing
            visible: !root._isSearchChecked

            Repeater {
                model: [{
                    "greed": pricer_addition.greed_tody,
                    "tipText": translator.tr("今天贪婪恐惧指数")
                }, {
                    "greed": pricer_addition.greed_yestoday,
                    "tipText": translator.tr("昨天贪婪恐惧指数")
                }]

                Item {
                    height: parent.height
                    width: parent.width / 2

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

            anchors.verticalCenter: parent.verticalCenter
            width: parent.width
            height: parent.height / 4 * 3
            visible: root._isSearchChecked
            color: theme.searchBarColor
            Keys.onTabPressed: root._isSearchChecked = !root._isSearchChecked
            text: root._isSearchChecked ? text : ""
            onEditingFinished: {
                pricer_model.search_and_show_at_beginning(text);
                root.searchEditingFinished();
            }

            Shortcut {
                sequence: shortKey.search
                onActivated: {
                    root._isSearchChecked = true;
                    searchBar.forceFocus();
                }
            }

        }

    }

    Row {
        id: uptime

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
                text: pricer_addition.system_time
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
        onClicked: window.isShowPopupWindow = false
        source: "qrc:/res/image/exit.png"
        tipText: translator.tr("关闭")
    }

}

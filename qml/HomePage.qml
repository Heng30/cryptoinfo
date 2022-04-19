import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Note" as Note
import "qrc:/res/qml/Price" as Price
import "qrc:/res/qml/Todo" as Todo
import "qrc:/res/qml/Setting" as Setting
import "qrc:/res/qml/Base" as Base

Window {
    id: root

    property bool isPopupPanelMaxHeight: false
    readonly property real _popupPanelMaxHeight: theme.popupPanelMaxHeight

    function isOpen() {
        return root.visible;
    }

    function showSettingPanel(checked) {
        window.isShowPopupWindow = true;
        config.panel_type = checked ? PanelType.Setting : PanelType.Price;
    }

    x: window.x > Screen.desktopAvailableWidth / 2 ? window.x - width : window.x + window.width
    y: window.y + window.height - height / 2
    width: content.width
    height: content.height
    color: "transparent"
    flags: Qt.Dialog | Qt.FramelessWindowHint | Qt.NoDropShadowWindowHint
    onIsPopupPanelMaxHeightChanged: {
        if (!root.isPopupPanelMaxHeight)
            return ;

        window.y = theme.startupY;
    }

    Base.MsgBox {
        id: msgBox
        anchors.centerIn: parent
    }

    Rectangle {
        id: bgField

        anchors.fill: parent
        focus: true
        radius: 5
        color: theme.bgColor
        opacity: theme.enteredOpacity

        Shortcut {
            sequence: shortKey.popupPanelClose
            onActivated: {
                window.isShowPopupWindow = false;
            }
        }

        Shortcut {
            sequence: shortKey.pricePanelViewAtBeginning
            onActivated: pricePanel.viewAtBeginning()
        }

        Shortcut {
            sequence: shortKey.pricePanelViewAtEnd
            onActivated: pricePanel.viewAtEnd()
        }

        Shortcut {
            sequence: shortKey.popupPanelMax
            context: Qt.ApplicationShortcut
            onActivated: root.isPopupPanelMaxHeight = !root.isPopupPanelMaxHeight
        }

        Column {
            id: content

            width: theme.popupPanelWidth

            Header {
                id: header

                onRefresh: pricer_model.update_now = true
                onSearchEditingFinished: pricePanel.viewAtBeginning()
                onNoteClicked: notePanel.forceFocus()
            }

            Price.Panel {
                id: pricePanel

                height: root.isPopupPanelMaxHeight ? root._popupPanelMaxHeight : theme.popupPanelHeight
                visible: config.panel_type === PanelType.Price
            }

            Setting.Panel {
                id: settingPanel

                height: pricePanel.height
                visible: config.panel_type === PanelType.Setting
            }

            Note.Panel {
                id: notePanel

                height: pricePanel.height
                visible: config.panel_type === PanelType.Note
            }

            Todo.Panel {
                id: notifyPanel

                height: pricePanel.height
                visible: config.panel_type === PanelType.Todo
            }

            Footer {
                id: footer
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Note" as Note
import "qrc:/res/qml/Price" as Price
import "qrc:/res/qml/Defi/Protocol" as DefiProtocol
import "qrc:/res/qml/Defi/Chain" as DefiChain
import "qrc:/res/qml/Defi/Chart" as DefiChart
import "qrc:/res/qml/Todo" as Todo
import "qrc:/res/qml/Setting" as Setting
import "qrc:/res/qml/ToolBox" as ToolBox
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Header" as Header

Item {
    id: homepage

    property bool _isMaxHeight: false
    property bool _settingIsChecked: config.panel_type === PanelType.Setting
    property bool _toolBoxIsChecked: config.panel_type === PanelType.ToolBox
    property bool _todoIsChecked: config.panel_type === PanelType.Todo
    property bool _homeIsChecked: config.panel_type === PanelType.Price
    property bool _noteIsChecked: config.panel_type === PanelType.Note
    property bool _defiProtocolIsChecked: config.panel_type === PanelType.DefiProtocol
    property bool _defiChainIsChecked: config.panel_type === PanelType.DefiChain
    property bool _defiChartIsChecked: config.panel_type === PanelType.DefiChart
    property real _bodyHeight: _isMaxHeight ? theme.panelMaxHeight : theme.panelHeight

    function _show_msg_box() {
        msgBox.add(translator.tr("程序已经在运行, 请勿重新启动!"), true, function() {
            utilityFn.quit();
        }, null);
    }

    width: content.width
    height: content.height
    on_IsMaxHeightChanged: {
        if (homepage._isMaxHeight)
            main.y = Screen.desktopAvailableHeight / 2 - main.height / 2;

    }
    Component.onCompleted: {
        if (config.single_ins && !config.can_open_pidlock) {
            if (window.visible)
                _show_msg_box();
            else
                window.visibleChanged.connect(_show_msg_box);
        }
    }

    About {
        id: about

        anchors.centerIn: parent
    }

    Base.MsgBox {
        id: msgBox

        anchors.centerIn: parent
    }

    Base.MsgTip {
        id: msgTip

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
            sequence: shortKey.homepageHide
            onActivated: main.hide()
        }

        Shortcut {
            sequence: shortKey.panelViewAtBeginning
            onActivated: {
                if (_homeIsChecked)
                    pricePanel.viewAtBeginning();
                else if (_defiProtocolIsChecked)
                    defiProtocolPanel.viewAtBeginning();
                else if (_defiChainIsChecked)
                    defiChainPanel.viewAtBeginning();
            }
        }

        Shortcut {
            sequence: shortKey.panelViewAtEnd
            onActivated: {
                if (_homeIsChecked)
                    pricePanel.viewAtEnd();
                else if (_defiProtocolIsChecked)
                    defiProtocolPanel.viewAtEnd();
                else if (_defiChainIsChecked)
                    defiChainPanel.viewAtEnd();
            }
        }

        Shortcut {
            sequence: shortKey.panelMax
            context: Qt.ApplicationShortcut
            onActivated: homepage._isMaxHeight = !homepage._isMaxHeight
        }

        Column {
            id: content

            width: theme.panelWidth

            Header.Field {
                id: header

                onPriceRefresh: price_model.update_now = true
                onDefiProtocolRefresh: defi_protocol_model.update_now = true
                onDefiChainRefresh: defi_chain_model.update_now = true
                onNoteClicked: notePanel.forceFocus()
                onDefiChartRefresh: {
                    if (defiChartPanel.checkedTabIndex === 0)
                        defi_chain_tvl_model.update_now = true;

                }
                onSearchEditingFinished: {
                    if (_homeIsChecked)
                        pricePanel.viewAtBeginning();
                    else if (_defiProtocolIsChecked)
                        defiProtocolPanel.viewAtBeginning();
                    else if (_defiChainIsChecked)
                        defiChainPanel.viewAtBeginning();
                }
            }

            Price.Panel {
                id: pricePanel

                height: _bodyHeight
                visible: _homeIsChecked
            }

            DefiProtocol.Panel {
                id: defiProtocolPanel

                height: _bodyHeight
                visible: _defiProtocolIsChecked
            }

            DefiChain.Panel {
                id: defiChainPanel

                height: _bodyHeight
                visible: _defiChainIsChecked
            }

            DefiChart.Panel {
                id: defiChartPanel

                height: _bodyHeight
                visible: _defiChartIsChecked
            }

            Setting.Panel {
                id: settingPanel

                height: _bodyHeight
                visible: _settingIsChecked
            }

            ToolBox.Panel {
                id: toolBoxPanel

                height: _bodyHeight
                visible: _toolBoxIsChecked
            }

            Note.Panel {
                id: notePanel

                height: _bodyHeight
                visible: _noteIsChecked
            }

            Todo.Panel {
                id: notifyPanel

                height: _bodyHeight
                visible: _todoIsChecked
            }

            Footer {
                id: footer
            }

        }

    }

}

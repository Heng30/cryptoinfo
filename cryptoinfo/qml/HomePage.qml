import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Price" as Price
import "qrc:/res/qml/Chain" as Chain
import "qrc:/res/qml/Chart" as Chart
import "qrc:/res/qml/Setting" as Setting
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Header" as Header
import "qrc:/res/qml/ToolBox" as ToolBox
import "qrc:/res/qml/ToolBox/DebugLog" as DebugLog
import "qrc:/res/qml/Intel" as Intel
import "qrc:/res/qml/Exchange" as Exchange
import "qrc:/res/qml/Monitor" as Monitor
import "qrc:/res/qml/StableCoin" as StableCoin
import "qrc:/res/qml/Address" as Address
import "qrc:/res/qml/Account" as Account
import "qrc:/res/qml/NFT" as NFT
import "qrc:/res/qml/Notify" as Notify

Item {
    id: homepage

    property bool _isMaxHeight: false
    property bool _settingIsChecked: config.panel_type === PanelType.Setting
    property bool _toolBoxIsChecked: config.panel_type === PanelType.ToolBox
    property bool _homeIsChecked: config.panel_type === PanelType.Price
    property bool _chainIsChecked: config.panel_type === PanelType.Chain
    property bool _chartIsChecked: config.panel_type === PanelType.Chart
    property bool _intelIsChecked: config.panel_type == PanelType.Intel
    property bool _exchangeIsChecked: config.panel_type == PanelType.Exchange
    property bool _monitorIsChecked: config.panel_type == PanelType.Monitor
    property bool _addressIsChecked: config.panel_type == PanelType.Address
    property bool _stableCoinIsChecked: config.panel_type == PanelType.StableCoin
    property bool _accountIsChecked: config.panel_type == PanelType.Account
    property bool _debugLogIsChecked: config.panel_type == PanelType.DebugLog
    property bool _nftIsChecked: config.panel_type == PanelType.NFT
    property bool _notifyIsChecked: config.panel_type == PanelType.Notify
    property real _bodyHeight: (_isMaxHeight ? theme.panelMaxHeight : theme.panelHeight) - header.height - footer.height

    function _show_quit_msg_box() {
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
                _show_quit_msg_box();
            else
                window.visibleChanged.connect(_show_quit_msg_box);
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

    Rectangle {
        id: bgField

        anchors.fill: parent
        focus: true
        radius: 5
        color: theme.bgColor

        Shortcut {
            sequence: shortKey.homepageHide
            onActivated: main.hide()
        }

        Shortcut {
            sequence: shortKey.panelViewAtBeginning
            onActivated: {
                if (_homeIsChecked)
                    pricePanel.viewAtBeginning();

            }
        }

        Shortcut {
            sequence: shortKey.panelViewAtEnd
            onActivated: {
                if (_homeIsChecked)
                    pricePanel.viewAtEnd();

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

                onSearchEditingFinished: {
                    if (_homeIsChecked)
                        pricePanel.viewAtBeginning();

                }
            }

            Price.Panel {
                id: pricePanel

                height: _bodyHeight
                visible: _homeIsChecked
            }

            Chain.Panel {
                id: chainPanel

                height: _bodyHeight
                visible: _chainIsChecked
            }

            Chart.Panel {
                id: defiChartPanel

                height: _bodyHeight
                visible: _chartIsChecked
            }

            Intel.Panel {
                id: intelPanel

                height: _bodyHeight
                visible: _intelIsChecked
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

            Exchange.Panel {
                id: exchangePanel

                height: _bodyHeight
                visible: _exchangeIsChecked
            }

            Monitor.Panel {
                id: monitorPanel

                height: _bodyHeight
                visible: _monitorIsChecked
            }

            StableCoin.Panel {
                id: stableCoinPanel

                height: _bodyHeight
                visible: _stableCoinIsChecked
            }

            Address.Panel {
                id: addressPanel

                height: _bodyHeight
                visible: _addressIsChecked
            }

            Account.Panel {
                id: accountPanel

                height: _bodyHeight
                visible: _accountIsChecked
            }

            DebugLog.Panel {
                id: debugLogPanel

                height: _bodyHeight
                visible: _debugLogIsChecked
            }

            NFT.Panel {
                id: nftPanel

                height: _bodyHeight
                visible: _nftIsChecked
            }

            Notify.Panel {
                id: notifyPanel

                height: _bodyHeight
                visible: _notifyIsChecked
            }

            Footer {
                id: footer
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: root

    property bool _settingIsChecked: config.panel_type === PanelType.Setting
    property bool _toolBoxIsChecked: config.panel_type === PanelType.ToolBox
    property bool _homeIsChecked: config.panel_type === PanelType.Price
    property bool _noteIsChecked: config.panel_type === PanelType.Note
    property bool _notifyIsChecked: config.panel_type === PanelType.Todo
    property bool _defiProtocolIsChecked: config.panel_type === PanelType.DefiProtocol
    property bool _defiChainIsChecked: config.panel_type === PanelType.DefiChain
    property bool _defiChartIsChecked: config.panel_type === PanelType.DefiChart

    signal priceRefresh()
    signal defiProtocolRefresh()
    signal defiChainRefresh()
    signal searchEditingFinished()
    signal noteClicked()

    width: parent.width
    height: theme.popupPanelHeaderHeight
    color: theme.headerBG
    radius: theme.itemRadius

    Component {
        id: dItem

        Item {
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

    Shortcut {
        sequence: shortKey.refresh
        onActivated: {
            if (root._homeIsChecked)
                root.priceRefresh();
            else if (root._defiProtocolIsChecked)
                root.defiProtocolRefresh();
            else if (root._defiChainIsChecked)
                root.defiChainRefresh();
        }
    }

    Row {
        anchors.left: parent.left
        anchors.leftMargin: theme.itemMargins
        spacing: theme.itemSpacing
        height: parent.height

        Left {
        }

        Price {
        }

        DefiProtocol {
        }

        DefiChain {
        }

    }

    Right {
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Header/LeftBtnField" as LeftBtnField

Rectangle {
    id: root

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
            if (_homeIsChecked)
                root.priceRefresh();
            else if (_defiProtocolIsChecked)
                root.defiProtocolRefresh();
            else if (_defiChainIsChecked)
                root.defiChainRefresh();
        }
    }

    Shortcut {
        sequence: shortKey.search
        onActivated: {
            if (_homeIsChecked)
                leftPrice.showSearchBar();
            else if (_defiProtocolIsChecked)
                leftDefiProtocol.showSearchBar();
            else if (_defiChainIsChecked)
                leftDefiChain.showSearchBar();
        }
    }

    Row {
        anchors.left: parent.left
        anchors.leftMargin: theme.itemMargins
        spacing: theme.itemSpacing
        height: parent.height

        Left {
        }

        LeftBtnField.Price {
            id: leftPrice
        }

        LeftBtnField.DefiProtocol {
            id: leftDefiProtocol
        }

        LeftBtnField.DefiChain {
            id: leftDefiChain
        }

    }

    Right {
    }

}

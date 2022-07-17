import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Exchange/Btc" as Btc

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: exchangeCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("BTC余额")
                property Component sourceComponent

                sourceComponent: Btc.Panel {
                }

            }
        ]
    }

}

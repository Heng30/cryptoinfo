import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: addressCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("BTC地址")
                property Component sourceComponent

                sourceComponent: Item {
                    Base.ItemLabel {
                        anchors.centerIn: parent
                        text: "没有实现"
                    }
                }

            },
            QtObject {
                property string tabText: translator.tr("ETH地址")
                property Component sourceComponent

                sourceComponent: Eth {
                }

            }
        ]
    }

}

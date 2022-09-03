import QtQuick 2.15
import QtQuick.Controls 2.15
import FooQml 1.0
import CusImage 1.0
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

                    FooQml {
                        name: "FooQml"
                        Component.onCompleted: console.log(name)
                    }

                    CusImage {
                        width: 200
                        height: 200
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

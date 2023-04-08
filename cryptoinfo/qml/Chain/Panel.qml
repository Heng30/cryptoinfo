import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Chain/Protocol" as Protocol
import "qrc:/res/qml/Chain/Tvl" as Tvl
import "qrc:/res/qml/Chain/Yield" as Yield
import "qrc:/res/qml/Chain/CryptoFee" as CryptoFee

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: chainCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("协议")
                property Component sourceComponent

                sourceComponent: Protocol.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("挖矿")
                property Component sourceComponent

                sourceComponent: Yield.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("锁仓量")
                property Component sourceComponent

                sourceComponent: Tvl.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("消耗费用")
                property Component sourceComponent

                sourceComponent: CryptoFee.Panel {
                }

            }
        ]
    }

}

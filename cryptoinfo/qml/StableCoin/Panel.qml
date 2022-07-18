import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/StableCoin/Mcap" as Mcap
import "qrc:/res/qml/StableCoin/Chain" as Chain

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: stableCoinCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("流通量")
                property Component sourceComponent

                sourceComponent: Mcap.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("公链")
                property Component sourceComponent

                sourceComponent: Chain.Panel {
                }

            }
        ]
    }

}

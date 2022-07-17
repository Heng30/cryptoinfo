import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/StableCoin/Mcap" as Mcap

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: StableCoinCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("流通量")
                property Component sourceComponent

                sourceComponent: Mcap.Panel {
                }

            }
        ]
    }

}

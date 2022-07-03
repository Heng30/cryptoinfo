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
        onClickedTabChanged: defiChartCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("公链锁仓量")
                property Component sourceComponent

                sourceComponent: ChainTVL {
                }

            }
        ]
    }

}

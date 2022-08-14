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
        onClickedTabChanged: nftCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("Gem 数据")
                property Component sourceComponent

                sourceComponent: Gem {
                }

            }
        ]
    }

}

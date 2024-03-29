import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Intel/MacroEvent" as MacroEvent
import "qrc:/res/qml/Intel/MacroNews" as MacroNews

Item {
    id: panel

    width: parent.width
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        onClickedTabChanged: intelCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("全球资讯")
                property Component sourceComponent

                sourceComponent: MacroNews.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("宏观事件")
                property Component sourceComponent

                sourceComponent: MacroEvent.Panel {
                }

            }
        ]
    }

}

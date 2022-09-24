import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Intel/News" as News
import "qrc:/res/qml/Intel/MacroEvent" as MacroEvent

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
                property string tabText: translator.tr("加密资讯")
                property Component sourceComponent

                sourceComponent: News.Panel {
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

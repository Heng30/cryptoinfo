import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: panel

    width: parent.width
    implicitHeight: 100
    property alias checkedTabIndex: bTab.clickedTab

    Base.BTab {
        id: bTab
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        enableBGColor: true
        model: [
            QtObject {
                property string tabText: translator.tr("总锁仓量")
                property Component sourceComponent

                sourceComponent: TotalTVL {
                }

            },
            QtObject {
                property string tabText: translator.tr("公链锁仓量")
                property Component sourceComponent

                sourceComponent: ChainTVL {
                }

            },
            QtObject {
                property string tabText: translator.tr("测试用例")
                property Component sourceComponent

                sourceComponent: Test {
                }

            }
        ]
    }

}

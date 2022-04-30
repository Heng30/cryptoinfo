import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/ToolBox/Encipher" as Encipher
import "qrc:/res/qml/ToolBox/IL" as IL
import "qrc:/res/qml/ToolBox/Other" as Other

Item {
    id: panel

    width: parent.width
    implicitWidth: 100
    implicitHeight: 100

    Base.BTab {
        id: bTab
        anchors.margins: theme.itemMargins
        anchors.fill: parent
        enableBGColor: true
        model: [
            QtObject {
                property string tabText: translator.tr("加解密")
                property Component sourceComponent

                sourceComponent: Encipher.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("无常损失")
                property Component sourceComponent

                sourceComponent: IL.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("其他")
                property Component sourceComponent

                sourceComponent: Other.Panel {
                }

            }
        ]
    }

}

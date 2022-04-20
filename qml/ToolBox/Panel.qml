import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/ToolBox/Encipher" as Encipher

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

            }
        ]
    }

}

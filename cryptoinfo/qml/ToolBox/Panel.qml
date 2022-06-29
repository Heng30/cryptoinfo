import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/ToolBox/Encipher" as Encipher
import "qrc:/res/qml/ToolBox/IL" as IL
import "qrc:/res/qml/ToolBox/AddrBook" as AddrBook
import "qrc:/res/qml/ToolBox/HandBook" as HandBook
import "qrc:/res/qml/ToolBox/BookMark" as BookMark
import "qrc:/res/qml/ToolBox/Other" as Other
import "qrc:/res/qml/ToolBox/Todo" as Todo
import "qrc:/res/qml/ToolBox/Note" as Note

Item {
    id: panel

    width: parent.width
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
                property string tabText: translator.tr("地址簿")
                property Component sourceComponent

                sourceComponent: AddrBook.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("手  帐")
                property Component sourceComponent

                sourceComponent: HandBook.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("书  签")
                property Component sourceComponent

                sourceComponent: BookMark.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("笔 记")
                property Component sourceComponent

                sourceComponent: Note.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("代 办")
                property Component sourceComponent

                sourceComponent: Todo.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("其  他")
                property Component sourceComponent

                sourceComponent: Other.Panel {
                }

            }
        ]
    }

}

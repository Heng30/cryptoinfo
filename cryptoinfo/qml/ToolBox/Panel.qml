import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/ToolBox/Encipher" as Encipher
import "qrc:/res/qml/ToolBox/Cal" as Cal
import "qrc:/res/qml/ToolBox/AddrBook" as AddrBook
import "qrc:/res/qml/ToolBox/HandBook" as HandBook
import "qrc:/res/qml/ToolBox/FundBook" as FundBook
import "qrc:/res/qml/ToolBox/BookMark" as BookMark
import "qrc:/res/qml/ToolBox/Other" as Other
import "qrc:/res/qml/ToolBox/Todo" as Todo
import "qrc:/res/qml/ToolBox/Note" as Note
import "qrc:/res/qml/ToolBox/ContractStats" as ContractStats

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
                property string tabText: translator.tr("计算策略")
                property Component sourceComponent

                sourceComponent: Cal.Panel {
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
                property string tabText: translator.tr("资 产")
                property Component sourceComponent

                sourceComponent: FundBook.Panel {
                }

            },
            QtObject {
                property string tabText: translator.tr("合约统计")
                property Component sourceComponent

                sourceComponent: ContractStats.Panel {
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

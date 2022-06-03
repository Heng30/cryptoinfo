import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.CDialog {
    id: chart

    anchors.centerIn: parent
    width: parent.width / 5 * 4
    height: parent.height / 5 * 4
    headerText: translator.tr("资产分布图")

    sourceComponent: Base.BTab {
        id: bTab

        enableBGColor: true
        model: [
            QtObject {
                property string tabText: translator.tr("支出占比图")
                property Component sourceComponent

                sourceComponent: PaymentChart {
                }

            },
            QtObject {
                property string tabText: translator.tr("收入占比图")
                property Component sourceComponent

                sourceComponent: IncomeChart {
                }

            }
        ]
    }

}

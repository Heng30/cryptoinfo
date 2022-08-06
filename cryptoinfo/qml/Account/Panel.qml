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
        onClickedTabChanged: accountCheckedTabIndex = clickedTab

        model: [
            QtObject {
                property string tabText: translator.tr("交易账户")
                property Component sourceComponent

                sourceComponent: OkexAccountChannel {
                }

            },
            QtObject {
                property string tabText: translator.tr("永续仓位")
                property Component sourceComponent

                sourceComponent: OkexPositionChannel {
                }

            },
            QtObject {
                property string tabText: translator.tr("交易余额")
                property Component sourceComponent

                sourceComponent: OkexGreekChannel {
                }

            },
            QtObject {
                property string tabText: translator.tr("资金余额")
                property Component sourceComponent

                sourceComponent: OkexMainAccountRest {
                }

            },
            QtObject {
                property string tabText: translator.tr("帐单流水")
                property Component sourceComponent

                sourceComponent: OkexBillRest {
                }

            },
            QtObject {
                property string tabText: translator.tr("充值记录")
                property Component sourceComponent

                sourceComponent: OkexDepositRest {
                }

            },
            QtObject {
                property string tabText: translator.tr("提现记录")
                property Component sourceComponent

                sourceComponent: OkexWithdrawalRest {
                }

            },
            QtObject {
                property string tabText: translator.tr("订阅详情")
                property Component sourceComponent

                sourceComponent: OkexSubscribeStatus {
                }

            }
        ]
    }

}

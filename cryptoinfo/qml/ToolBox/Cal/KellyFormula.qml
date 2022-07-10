/**
 * @brief 凯利计算公式
 *
 * @see https://fical.net/zh-hans/%E5%87%AF%E5%88%A9%E5%85%AC%E5%BC%8F%E8%AE%A1%E7%AE%97%E5%99%A8
 */

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: kellyFormula

    width: parent.width
    headerText: translator.tr("凯利公式")
    spacing: theme.itemSpacing

    contentItem: Column {
        id: content

        function calculate() {
            if (Number(odd.text) <= 0) {
                outputText.investPositionPer = 0;
                return ;
            }
            if (bet.checked) {
                outputText.investPositionPer = ((Number(winPercent.text) / 100 * (Number(odd.text) + 1) - 1) / Number(odd.text)) * 100;
            } else {
                if (Number(afterWinPercent.text) <= 0 || Number(afterLossPercent) <= 0) {
                    outputText.investPositionPer = 0;
                    return ;
                }
                outputText.investPositionPer = (Number(winPercent.text) / Number(afterLossPercent.text) - (100 - Number(winPercent.text)) / Number(afterWinPercent.text)) * 100;
            }
        }

        Component.onCompleted: content.calculate()
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.SelectBox {
                id: odd

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3
                boxWidth: theme.fontPixelNormal * 2
                labelText: translator.tr("赔率") + ":"
                model: [translator.tr("倍")]
                visible: bet.checked
                text: String(2)
                onTextAccepted: content.calculate()
            }

            Row {
                width: parent.width / 2
                visible: !odd.visible

                Base.SelectBox {
                    id: afterWinPercent

                    width: parent.width / 2
                    txtFieldWidth: theme.fontPixelNormal * 3
                    boxWidth: theme.fontPixelNormal * 2
                    labelText: translator.tr("止盈率") + ":"
                    model: [translator.tr("%")]
                    text: String(50)
                    onTextAccepted: content.calculate()

                    validator: IntValidator {
                        bottom: 0
                    }

                }

                Base.SelectBox {
                    id: afterLossPercent

                    width: parent.width / 2
                    txtFieldWidth: theme.fontPixelNormal * 3
                    boxWidth: theme.fontPixelNormal * 2
                    labelText: translator.tr("止损率") + ":"
                    model: [translator.tr("%")]
                    text: String(20)
                    onTextAccepted: content.calculate()

                    validator: IntValidator {
                        bottom: 0
                        top: 100
                    }

                }

            }

            Base.SelectBox {
                id: winPercent

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 5
                boxWidth: theme.fontPixelNormal * 4
                labelWidth: totalPosition.labelWidth
                labelText: translator.tr("胜率") + ":"
                model: [translator.tr("%")]
                text: String(50)
                onTextAccepted: content.calculate()

                validator: IntValidator {
                    bottom: 0
                    top: 100
                }

            }

        }

        Row {
            width: parent.width

            Row {
                width: parent.width / 2
                anchors.verticalCenter: parent.verticalCenter

                Base.RadioButton {
                    id: investment

                    width: parent.width / 2
                    text: translator.tr("投资公式")
                    checked: !bet.checked
                    onCheckedChanged: content.calculate()
                }

                Base.RadioButton {
                    id: bet

                    width: parent.width / 2
                    height: investment.height
                    text: translator.tr("下注公式")
                    checked: false
                }

            }

            Row {
                width: parent.width / 2
                anchors.verticalCenter: parent.verticalCenter

                Base.SelectBox {
                    id: totalPosition

                    width: parent.width / 2
                    txtFieldWidth: theme.fontPixelNormal * 5
                    boxWidth: theme.fontPixelNormal * 4
                    labelText: translator.tr("总仓位") + ":"
                    text: String(100)
                    model: [translator.tr("美元")]
                    onTextAccepted: content.calculate()

                    validator: IntValidator {
                        bottom: 1
                    }

                }

            }

        }

        Item {
            width: parent.width
            height: outputText.height + theme.itemMargins * 2

            Base.TxtField {
                id: outputText

                readonly property int sepCount: 2
                property double investPositionPer: 0

                anchors.centerIn: parent
                showBorder: false
                font.bold: true
                text: translator.tr("总仓位") + ": " + totalPosition.text + utilityFn.paddingSpace(sepCount) + translator.tr("投入仓位") + ": " + utilityFn.toFixedPrice(Number(totalPosition.text) * investPositionPer / 100) + utilityFn.paddingSpace(sepCount) + translator.tr("投入仓位占比") + ": " + utilityFn.toPercentString(investPositionPer)
                readOnly: true
            }

        }

    }

}

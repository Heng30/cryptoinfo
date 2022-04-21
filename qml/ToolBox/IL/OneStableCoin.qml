import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: oneStableCoin

    width: parent.width
    headerText: translator.tr("稳定币:非稳定币(1:1)")
    spacing: theme.itemSpacing

    contentItem: Column {
        id: content

        function calculate() {
            var unstableCoinPrice = Number(coinPrice.text);
            var unstableCoinCount = Number(coinCount.text);
            var unstableCoinPriceRate = Number(priceChangedRate.text) / 100;
            if (unstableCoinCount <= 0 || unstableCoinPrice <= 0 || unstableCoinPriceRate <= 0)
                return ;

            var stableCoinCount = unstableCoinPrice * unstableCoinCount;
            var c = unstableCoinCount * stableCoinCount;
            unstableCoinPriceRate = priceUp.checked ? unstableCoinPriceRate : -unstableCoinPriceRate;
            // 计算价格变化后的总价值
            var changedUnstableCoinPrice = unstableCoinPrice * (1 + unstableCoinPriceRate);
            if (changedUnstableCoinPrice <= 0)
                return ;

            var changedUnstableCoinCount = Math.sqrt(c / changedUnstableCoinPrice);
            var changedStableCoinCount = Math.sqrt(c * changedUnstableCoinPrice);
            var changedValue = changedUnstableCoinPrice * changedUnstableCoinCount + changedStableCoinCount;
            var unchangedValue = changedUnstableCoinPrice * unstableCoinCount + stableCoinCount;
            ilRate.rate = (unchangedValue - changedValue) / unchangedValue * 100;
            ilValue.value = unchangedValue - changedValue;
        }

        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.SelectBox {
                id: coinPrice

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 5 + itemSpacing
                boxWidth: theme.fontPixelNormal * 3 + theme.itemSpacing
                labelText: translator.tr("非稳定币价格") + ":"
                model: [translator.tr("美元")]
                text: String(100)
                onTextAccepted: content.calculate()

                validator: IntValidator {
                    bottom: 1
                }

            }

            Base.SelectBox {
                id: coinCount

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 5 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("非稳定币数量") + ":"
                model: [translator.tr("枚")]
                text: String(100)
                onTextAccepted: content.calculate()

                validator: IntValidator {
                    bottom: 1
                }

            }

        }

        Row {
            width: parent.width

            Row {
                width: parent.width / 2
                anchors.verticalCenter: parent.verticalCenter

                Base.RadioButton {
                    id: priceUp

                    width: parent.width / 2
                    text: translator.tr("价格上涨")
                    checked: !priceDown.checked
                    onCheckedChanged: content.calculate()
                }

                Base.RadioButton {
                    id: priceDown

                    width: parent.width / 2
                    height: priceUp.height
                    text: translator.tr("价格下跌")
                    checked: false
                }

            }

            Row {
                width: parent.width / 2
                anchors.verticalCenter: parent.verticalCenter

                Base.SelectBox {
                    id: priceChangedRate

                    width: parent.width / 2
                    txtFieldWidth: theme.fontPixelNormal * 5 + itemSpacing
                    boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                    labelWidth: coinCount.labelWidth
                    labelText: translator.tr("价格变化") + ":"
                    text: String(0)
                    model: [translator.tr("%")]
                    onTextAccepted: content.calculate()

                    validator: IntValidator {
                        bottom: 0
                    }

                }

            }

        }

        Item {
            width: parent.width
            height: ilRateRow.height

            Row {
                id: ilRateRow

                anchors.centerIn: parent
                spacing: theme.itemSpacing * 5

                Base.TxtField {
                    id: ilValue

                    property double value: 0

                    showBorder: false
                    text: translator.tr("损失金额: ") + utilityFn.toFixedPrice(value) + translator.tr("美元")
                    readOnly: true
                }

                Base.TxtField {
                    id: ilRate

                    property double rate: 0

                    showBorder: false
                    text: translator.tr("无常损失: ") + utilityFn.toPercentString(rate)
                    readOnly: true
                }

            }

        }

    }

}

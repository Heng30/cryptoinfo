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
            if (unstableCoinCount < 0 || unstableCoinPrice < 0 || unstableCoinPriceRate < 0)
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
            il.unchangedValue = unchangedValue;
            il.changedValue = changedValue;
            il.lostValue = unchangedValue - changedValue;
            il.lostRate = (unchangedValue - changedValue) / unchangedValue * 100;
        }

        Component.onCompleted: content.calculate()
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.SelectBox {
                id: coinPrice

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 5
                boxWidth: theme.fontPixelNormal * 4
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
                txtFieldWidth: theme.fontPixelNormal * 5
                boxWidth: theme.fontPixelNormal * 2
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
                    txtFieldWidth: theme.fontPixelNormal * 5
                    boxWidth: theme.fontPixelNormal * 2
                    labelWidth: coinCount.labelWidth
                    labelText: translator.tr("价格变化") + ":"
                    text: String(10)
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
            height: il.height + theme.itemMargins * 2

            Base.TxtField {
                id: il

                readonly property int sepCount: 2
                property double unchangedValue: 0
                property double changedValue: 0
                property double lostValue: 0
                property double lostRate: 0

                anchors.centerIn: parent
                showBorder: false
                font.bold: true
                text: translator.tr("无常损失前价值") + ": " + utilityFn.toFixedPrice(unchangedValue) + translator.tr("美元") + utilityFn.paddingSpace(sepCount) + translator.tr("无常损失后价值") + ": " + utilityFn.toFixedPrice(changedValue) + translator.tr("美元") + utilityFn.paddingSpace(sepCount) + translator.tr("损失") + ": " + utilityFn.toFixedPrice(lostValue) + translator.tr("美元") + utilityFn.paddingSpace(sepCount) + translator.tr("百分比") + ": " + utilityFn.toPercentString(lostRate)
                readOnly: true
            }

        }

    }

}

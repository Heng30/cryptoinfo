import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: compoundIntrest

    width: parent.width
    headerText: translator.tr("复利")
    spacing: theme.itemSpacing

    contentItem: Column {
        id: content

        function calculate() {
            if (isNaN(Number(capital.text)) || isNaN(Number(rate.text)) || isNaN(Number(period.text)))
                return String(0);

            return (Number(capital.text) * Math.pow((1 + Number(rate.text) / 100), Number(period.text))).toFixed(0);
        }

        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.SelectBox {
                id: capital

                width: parent.width / 4
                anchors.verticalCenter: parent.verticalCenter
                txtFieldWidth: theme.fontPixelNormal * 5
                boxWidth: theme.fontPixelNormal * 2
                labelWidth: rate.labelWidth
                labelText: translator.tr("本金") + ":"
                text: String(100)
                model: [translator.tr("元")]

                validator: IntValidator {
                    bottom: 0
                }

            }

            Base.SelectBox {
                id: rate

                width: parent.width / 4
                txtFieldWidth: theme.fontPixelNormal * 3
                boxWidth: theme.fontPixelNormal * 2
                labelText: translator.tr("利率") + ":"
                model: [translator.tr("%")]
                text: String(10)

                validator: IntValidator {
                    bottom: 0
                }

            }

            Base.SelectBox {
                id: period

                width: parent.width / 4
                txtFieldWidth: theme.fontPixelNormal * 3
                boxWidth: theme.fontPixelNormal * 2
                labelText: translator.tr("期数") + ":"
                model: [translator.tr("期")]
                text: String(10)

                validator: IntValidator {
                    bottom: 1
                }

            }

            Base.SelectBox {
                width: parent.width / 4
                anchors.verticalCenter: parent.verticalCenter
                txtFieldWidth: theme.fontPixelNormal * 5
                boxWidth: theme.fontPixelNormal * 2
                labelText: translator.tr("复利后的金额") + ":"
                text: content.calculate()
                model: [translator.tr("元")]
            }

        }

    }

}

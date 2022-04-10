import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: data

    width: parent.width
    headerText: translator.tr("数据设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        Row {
            width: parent.width

            Base.SelectBox {
                function _setRefreshInterval(index) {
                    var second = index === 0 ? Number(text) : utilityFn.minus2seconds(Number(text));
                    config.price_refresh_interval = second;
                    config.save_config();
                }

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("数据刷新时间间隔") + ":"
                model: [translator.tr("秒"), translator.tr("分")]
                onBoxActived: _setRefreshInterval(boxCurrentIndex)
                onTextAccepted: _setRefreshInterval(boxCurrentIndex)
                Component.onCompleted: {
                    boxCurrentIndex = config.price_refresh_interval < 60 ? 0 : 1;
                    text = boxCurrentIndex === 0 ? config.price_refresh_interval : utilityFn.seconds2minus(config.price_refresh_interval);
                }

                validator: IntValidator {
                    bottom: 1
                    top: 59
                }

            }

            Base.SelectBox {
                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("数据条目") + ":"
                model: [translator.tr("条")]
                text: config.price_item_count
                onTextAccepted: {
                    config.price_item_count = Number(text);
                    config.save_config();
                }

                validator: IntValidator {
                    bottom: 1
                    top: 10000
                }

            }

        }

    }

}

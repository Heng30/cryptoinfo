import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: data

    width: parent.width
    headerText: translator.tr("数据设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing * 2
        Row {
            width: parent.width

            Base.SelectBox {
                function _setRefreshInterval(index) {
                    var second = index === 0 ? Number(text) : utilityFn.minus2seconds(Number(text));
                    config.price_refresh_interval = second;
                    config.save_qml();
                    price_model.update_interval = second;
                }

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("价格刷新时间间隔") + ":"
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
                labelText: translator.tr("价格条目") + ":"
                model: [translator.tr("条")]
                text: config.price_item_count
                onTextAccepted: {
                    config.price_item_count = Number(text);
                    config.save_qml();
                    price_model.set_url_qml(config.price_item_count);
                }

                validator: IntValidator {
                    bottom: 1
                    top: 10000
                }

            }

        }

        Row {
            width: parent.width

            Base.SelectBox {

                function _setRefreshInterval(index) {
                    var second = index === 0 ? utilityFn.minus2seconds(Number(text)) : utilityFn.hours2seconds(Number(text));
                    config.defi_refresh_interval = second;
                    config.save_qml();
                    defi_protocol_model.update_interval = second;
                }

                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("Defi刷新时间间隔") + ":"
                model: [translator.tr("分"), translator.tr("时")]
                onBoxActived: _setRefreshInterval(boxCurrentIndex)
                onTextAccepted: _setRefreshInterval(boxCurrentIndex)
                Component.onCompleted: {
                    boxCurrentIndex = config.defi_refresh_interval < 3600 ? 0 : 1;
                    text = boxCurrentIndex === 0 ? utilityFn.seconds2minus(config.defi_refresh_interval) : utilityFn.seconds2Hours(config.defi_refresh_interval);
                }

                validator: IntValidator {
                    bottom: 1
                }

            }

            Base.SelectBox {
                width: parent.width / 2
                txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                boxWidth: theme.fontPixelNormal * 2 + theme.itemSpacing
                labelText: translator.tr("Defi条目") + ":"
                model: [translator.tr("条")]
                text: config.defi_item_count
                onTextAccepted: {
                    defi_protocol_model.item_max_count = Number(text);
                    config.defi_item_count = Number(text);
                    config.save_qml();
                }

                validator: IntValidator {
                    bottom: 1
                    top: 1000
                }

            }

        }

    }

}

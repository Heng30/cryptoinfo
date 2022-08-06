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
                id: priceRefreshInterval
                function _setRefreshInterval(index) {
                    var second = index === 0 ? Number(text) : utilityFn.minus2seconds(Number(text));
                    if (second < 5) {
                        second = 5;
                        text = String(5);
                    }
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

            Base.Switch {
                property bool _flag: !config.unrefresh_when_not_focus

                width: parent.width / 2
                text: checked ? translator.tr("非关注后台不刷新") : translator.tr("后台一直刷新")
                checked: config.unrefresh_when_not_focus
                onCheckedChanged: {
                    if (_flag) {
                        config.unrefresh_when_not_focus = checked;
                        config.save_qml();
                    }
                    _flag = true;
                }
            }

            Row {
                width: parent.width / 2

                Base.ItemLabel {
                    id: apiKeyLabel

                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("owlracle api key") + ": "
                    tipText: translator.tr("用于获取ETH油费")
                }

                Base.InputBar {
                    width: parent.width - apiKeyLabel.width - parent.spacing - theme.itemSpacing * 8
                    anchors.verticalCenter: parent.verticalCenter
                    underText: translator.tr("api key")
                    text: config.owlracle_api_key
                    onAccepted: {
                        if (text.length <= 0)
                            return ;

                        config.owlracle_api_key = text.trim();
                        config.save_qml();
                        apiKeyLabel.forceActiveFocus();
                        msgTip.add(translator.tr("设置成功! 重启使配置生效."), false);
                    }
                }

            }

        }

    }

}

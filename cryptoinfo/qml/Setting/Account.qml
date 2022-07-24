import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    width: parent.width
    headerText: translator.tr("账户设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing * 2

        Row {
            width: parent.width

            Row {
                width: parent.width / 2

                Base.ItemLabel {
                    id: apiKeyLabel

                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("okex apiv5 key") + ": "
                }

                Base.InputBar {
                    id: apiKeyInput

                    width: parent.width - apiKeyLabel.width - parent.spacing - theme.itemSpacing * 8
                    anchors.verticalCenter: parent.verticalCenter
                    underText: translator.tr("api key")
                    text: config.okex_api_key
                }

            }

            Row {
                width: parent.width / 2

                Base.ItemLabel {
                    id: secretKeyLabel

                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("okex apiv5 secert key") + ": "
                }

                Base.InputBar {
                    id: secretKeyInput

                    width: parent.width - secretKeyLabel.width - parent.spacing - theme.itemSpacing * 8
                    anchors.verticalCenter: parent.verticalCenter
                    underText: translator.tr("secert key")
                    text: config.okex_secret_key
                }

            }

        }

        Row {
            width: parent.width

            Row {
                width: parent.width / 2

                Base.ItemLabel {
                    id: passphraseLabel

                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("okex apiv5 passphrase") + ": "
                }

                Base.InputBar {
                    id: passphraseInput

                    width: parent.width - passphraseLabel.width - parent.spacing - theme.itemSpacing * 8
                    anchors.verticalCenter: parent.verticalCenter
                    underText: translator.tr("passphrase")
                    text: config.okex_passphrase
                }

            }

            Item {
                width: parent.width / 2
                height: parent.height

                Base.TxtButton {
                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("保存")
                    onClicked: {
                        if (apiKeyInput.text.length <= 0 || secretKeyInput.text.length <= 0 || passphraseInput.text.length <= 0) {
                            msgTip.add(translator.tr("保存失败! 输入内容不能为空!"), true);
                            return ;
                        }
                        config.okex_api_key = apiKeyInput.text;
                        config.okex_secret_key = secretKeyInput.text;
                        config.okex_passphrase = passphraseInput.text;
                        config.save_qml();
                        msgTip.add(translator.tr("保存成功! 请刷新账户页面."), true);
                    }
                }

            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: webServer

    width: parent.width
    headerText: translator.tr("Web服务设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.Switch {
                id: enableWebServer

                property bool _flag: !config.enable_web_server

                anchors.verticalCenter: parent.verticalCenter
                width: parent.width / 2
                text: checked ? translator.tr("已启用Web服务") : translator.tr("未启用Web服务")
                checked: config.enable_web_server
                onCheckedChanged: {
                    if (_flag) {
                        config.enable_web_server = checked;
                        config.save();
                        msgTip.add(translator.tr("重启程序, 使配置生效!"), false);
                    }
                    _flag = true;
                }
            }

            Item {
                anchors.verticalCenter: parent.verticalCenter
                width: parent.width / 2
                height: row.height

                Row {
                    id: row

                    anchors.left: parent.left
                    anchors.leftMargin: theme.itemMargins
                    width: parent.width / 2 - anchors.leftMargin
                    spacing: theme.itemSpacing

                    Base.TxtButton {
                        anchors.verticalCenter: parent.verticalCenter
                        text: translator.tr("设置")
                        onClicked: {
                            config.web_server_address = address.text;
                            config.web_server_port = Number(port.text);
                            config.save();
                            msgTip.add(translator.tr("设置成功! 重启程序, 使配置生效!"), false);
                        }
                    }

                    Item {
                        height: parent.height
                    }

                    Base.InputBar {
                        id: address

                        anchors.verticalCenter: parent.verticalCenter
                        underText: translator.tr("IP")
                        width: theme.fontPixelNormal * 10
                        text: config.web_server_address
                    }

                    Base.ItemLabel {
                        anchors.verticalCenter: parent.verticalCenter
                        text: ": "
                    }

                    Base.InputBar {
                        id: port

                        anchors.verticalCenter: parent.verticalCenter
                        underText: translator.tr("端口")
                        width: theme.fontPixelNormal * 3
                        text: String(config.web_server_port)
                    }

                }

            }

        }

    }

}

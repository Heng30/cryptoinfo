import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: webServer

    width: parent.width
    headerText: translator.tr("其他")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Row {
                width: parent.width / 2
                spacing: theme.itemSpacing

                Base.ItemLabel {
                    id: label

                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("浏览器") + ": "
                }

                Base.InputBar {
                    id: browser

                    anchors.verticalCenter: parent.verticalCenter
                    width: Math.min(parent.width - label.width, theme.fontPixelNormal * 15)
                    underText: translator.tr("请输浏览器名称")
                    text: config.browser
                    onAccepted: {
                        if (browser.text.length <= 0)
                            return ;

                        config.browser = browser.text;
                        config.save();
                        label.forceActiveFocus();
                        msgTip.add(translator.tr("设置成功!"), false);
                    }
                }

            }

        }

    }

}

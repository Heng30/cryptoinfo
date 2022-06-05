import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: ui

    width: parent.width
    headerText: translator.tr("界面设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Base.NumInput {
                id: fontSizeSetting

                width: parent.width / 2
                text: theme.fontPixelNormal
                labelText: translator.tr("字体大小") + ":"
                readOnly: true
                onInc: {
                    config.font_pixel_size_normal += 1;
                    config.save();
                }
                onDec: {
                    config.font_pixel_size_normal -= 1;
                    config.save();
                }
            }

            Base.NumInput {
                id: opacitySetting

                width: parent.width / 2
                text: theme.windowOpacity.toFixed(1)
                labelText: translator.tr("透明度") + ":"
                readOnly: true
                onInc: {
                    var opacity = config.window_opacity + 0.1;
                    config.window_opacity = Math.min(opacity, 1.0);
                    config.save();
                }
                onDec: {
                    var opacity = config.window_opacity - 0.1;
                    config.window_opacity = Math.max(opacity, 0.5);
                    config.save();
                }
            }

        }

        Row {
            width: parent.width

            Row {
                anchors.verticalCenter: parent.verticalCenter
                width: parent.width / 2
                spacing: theme.itemSpacing

                Base.SelectBox {
                    anchors.verticalCenter: parent.verticalCenter
                    txtFieldWidth: theme.fontPixelNormal * 3 + itemSpacing
                    boxWidth: theme.fontPixelNormal * 3
                    labelText: translator.tr("窗口(宽x高)") + ":"
                    text: config.window_width
                    model: [translator.tr("像素")]
                    onTextAccepted: {
                        if (text.length <= 0)
                            return ;

                        var width = Number(text);
                        config.window_width = Math.min(Math.max(width, 840), Screen.desktopAvailableWidth);
                        config.save();
                        msgTip.add(translator.tr("设置成功!"), false);
                    }
                }

                Base.ItemLabel {
                    anchors.verticalCenter: parent.verticalCenter
                    text: "x"
                }

                Base.SelectBox {
                    anchors.verticalCenter: parent.verticalCenter
                    boxWidth: theme.fontPixelNormal * 3
                    text: config.window_height
                    model: [translator.tr("像素")]
                    onTextAccepted: {
                        if (text.length <= 0)
                            return ;

                        var height = Number(text);
                        config.window_height = Math.min(Math.max(height, 680), Screen.desktopAvailableHeight);
                        config.save();
                        msgTip.add(translator.tr("设置成功!"), false);
                    }
                }

            }

        }

        Row {
            width: parent.width

            Base.Switch {
                id: showSplash

                property bool _flag: !config.show_splash

                width: parent.width / 2
                text: checked ? translator.tr("已启用启动画面") : translator.tr("未启用启动画面")
                checked: config.show_splash
                onCheckedChanged: {
                    if (_flag) {
                        if (!checked)
                            splashSound.checked = false;

                        config.show_splash = checked;
                        config.save();
                    }
                    _flag = true;
                }
            }

            Base.Switch {
                id: splashSound

                property bool _flag: !(config.use_splash_sound && config.show_splash)

                width: parent.width / 2
                text: checked ? translator.tr("已启用启动画面声音") : translator.tr("未启用启动画面声音")
                checked: config.use_splash_sound && config.show_splash
                onCheckedChanged: {
                    if (_flag) {
                        if (config.show_splash) {
                            config.use_splash_sound = checked;
                            config.save();
                        } else {
                            splashSound.checked = false;
                            msgTip.add(translator.tr("启用启动画面声音前, 需先启用开机画面!"), false);
                        }
                    }
                    _flag = true;
                }
            }

        }

        Row {
            width: parent.width

            Base.Switch {
                id: loginPS

                property bool _flag: !config.enable_login_password

                width: parent.width / 2
                text: checked ? translator.tr("已启用登陆密码保护") : translator.tr("未启用登陆密码保护")
                checked: config.enable_login_password
                onCheckedChanged: {
                    if (_flag) {
                        config.enable_login_password = checked;
                        config.save();
                    }
                    _flag = true;
                }
            }

            Base.Switch {
                id: singleIns

                property bool _flag: !config.single_ins

                width: parent.width / 2
                text: checked ? translator.tr("已启单进程实例") : translator.tr("未启用单进程实例")
                checked: config.single_ins
                onCheckedChanged: {
                    if (_flag) {
                        config.single_ins = checked;
                        config.save();
                    }
                    _flag = true;
                }
            }

        }

    }

}

import QtQuick 2.15
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
                    config.window_opacity += 0.1;
                    config.save();
                }
                onDec: {
                    config.window_opacity -= 0.1;
                    config.save();
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
                        config.show_splash = checked;
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

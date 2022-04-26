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
                onInc: config.font_pixel_size_normal += 1
                onDec: config.font_pixel_size_normal -= 1
                onTextChanged: config.save()
            }

            Base.NumInput {
                id: opacitySetting

                width: parent.width / 2
                text: theme.windowOpacity.toFixed(1)
                labelText: translator.tr("透明度") + ":"
                readOnly: true
                onInc: config.window_opacity += 0.1
                onDec: config.window_opacity -= 0.1
                onTextChanged: config.save()
            }

        }

        Row {
            width: parent.width

            Base.Switch {
                id: liveCircleSwitch

                width: parent.width / 2
                text: checked ? translator.tr("已启用动态扩散效果") : translator.tr("未启用动态扩散效果")
                checked: config.show_live_circle
                onCheckedChanged: {
                    config.set_show_live_circle(checked);
                    config.save();
                }
            }

            Base.Switch {
                id: showSplash

                width: parent.width / 2
                text: checked ? translator.tr("已启用启动画面") : translator.tr("未启用启动画面")
                checked: config.show_splash
                onCheckedChanged: {
                    config.set_show_splash(checked);
                    config.save();
                }
            }

        }

    }

}

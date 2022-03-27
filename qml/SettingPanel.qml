import QtQuick 2.15
import QtQuick.Controls 2.15
import "./Base" as Base

ScrollView {
    id: root

    width: parent.width
    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
    ScrollBar.vertical.policy: ScrollBar.AlwaysOff

    Flickable {
        width: parent.width
        height: parent.height
        contentWidth: width
        contentHeight: content.height
        clip: true

        Column {
            id: content

            width: parent.width
            spacing: theme.itemSpacing

            Base.SettingField {
                id: uiSetting

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
                            onTextChanged: config.save_config()
                        }

                        Base.NumInput {
                            id: opacitySetting

                            width: parent.width / 2
                            text: theme.windowOpacity.toFixed(1)
                            labelText: translator.tr("透明度") + ":"
                            readOnly: true
                            onInc: config.window_opacity += 0.1
                            onDec: config.window_opacity -= 0.1
                            onTextChanged: config.save_config()
                        }

                    }

                    Base.Switch {
                        id: liveCircleSwitch

                        width: parent.width / 2
                        text: translator.tr("启用动态扩散效果")
                        checked: config.show_live_circle
                        onCheckedChanged: {
                            config.set_show_live_circle(checked);
                            config.save_config();
                        }
                    }

                }

            }

            Base.SettingField {
                id: langSetting

                width: parent.width
                headerText: translator.tr("语言设置")
                spacing: theme.itemSpacing

                contentItem: Row {
                    Base.RadioButton {
                        id: chineseLang

                        width: parent.width / 2
                        text: translator.tr("中文")
                        checked: !englishLang.checked
                        onCheckedChanged: {
                            config.use_chinese = checked;
                            config.save_config();
                        }
                    }

                    Base.RadioButton {
                        id: englishLang

                        width: parent.width / 2
                        height: chineseLang.height
                        text: translator.tr("English")
                        checked: !config.use_chinese
                    }

                }

            }

            Base.SettingField {
                id: shortCutSetting

                width: parent.width
                headerText: translator.tr("快捷键")
                spacing: theme.itemSpacing

                contentItem: Column {
                    Repeater {
                        model: [{
                            "key": "Esc",
                            "value": translator.tr("关闭窗口")
                        }, {
                            "key": "Ctrl+M",
                            "value": translator.tr("最大化窗口")
                        }, {
                            "key": "Ctrl+=",
                            "value": translator.tr("放大窗口")
                        }, {
                            "key": "Ctrl+-",
                            "value": translator.tr("缩小窗口")
                        }, {
                            "key": "Ctrl+H",
                            "value": translator.tr("跳到第一个条目")
                        }, {
                            "key": "Ctrl+L",
                            "value": translator.tr("跳到第最后一个条目")
                        }, {
                            "key": "Ctrl+F",
                            "value": translator.tr("打开搜索框")
                        }, {
                            "key": "Tab",
                            "value": translator.tr("关闭搜索框")
                        }, {
                            "key": "Ctrl+R",
                            "value": translator.tr("刷新数据")
                        }, {
                            "key": "Alt+1",
                            "value": translator.tr("关注排序")
                        }, {
                            "key": "Alt+2",
                            "value": translator.tr("排名排序")
                        }, {
                            "key": "Alt+3",
                            "value": translator.tr("价格排序")
                        }, {
                            "key": "Alt+4",
                            "value": translator.tr("代币排序")
                        }, {
                            "key": "Alt+5",
                            "value": translator.tr("24小时行情排序")
                        }, {
                            "key": "Alt+6",
                            "value": translator.tr("24小时交易量排序")
                        }]

                        Row {
                            width: parent.width

                            Base.ItemLabel {
                                width: parent.width / 2
                                text: modelData.key
                            }

                            Base.ItemLabel {
                                width: parent.width / 2
                                text: modelData.value
                            }

                        }

                    }

                }

            }

        }

    }

}

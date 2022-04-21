import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: shortKey

    width: parent.width
    headerText: translator.tr("快捷键")
    spacing: theme.itemSpacing

    contentItem: Column {
        Repeater {
            model: [{
                "key": "Esc",
                "value": translator.tr("关闭窗口")
            }, {
                "key": "Tab",
                "value": translator.tr("关闭搜索框")
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
                "key": "Ctrl+T",
                "value": translator.tr("切换主题")
            }, {
                "key": "Ctrl+R",
                "value": translator.tr("刷新数据")
            }, {
                "key": "Alt+S",
                "value": translator.tr("设置")
            }, {
                "key": "Alt+N",
                "value": translator.tr("笔记")
            }, {
                "key": "Alt+H",
                "value": translator.tr("主页")
            }, {
                "key": "Alt+B",
                "value": translator.tr("工具箱")
            }, {
                "key": "Alt+T",
                "value": translator.tr("代办事项")
            }, {
                "key": "Alt+1",
                "value": translator.tr("关注排序")
            }, {
                "key": "Alt+2",
                "value": translator.tr("市值排序")
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
                "value": translator.tr("7天行情")
            }, {
                "key": "Alt+7",
                "value": translator.tr("24小时交易量排序")
            }, {
                "key": "Alt+8",
                "value": translator.tr("告警地板价")
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

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
                "value": translator.tr("隐藏窗口")
            }, {
                "key": "Tab",
                "value": translator.tr("关闭搜索框")
            }, {
                "key": "Ctrl+=",
                "value": translator.tr("放大字体")
            }, {
                "key": "Ctrl+-",
                "value": translator.tr("缩小字体")
            }, {
                "key": "Ctrl+H",
                "value": translator.tr("跳到第一个条目")
            }, {
                "key": "Ctrl+L",
                "value": translator.tr("跳到第最后一个条目")
            }, {
                "key": "Ctrl+M",
                "value": translator.tr("最大化窗口")
            }, {
                "key": "Ctrl+T",
                "value": translator.tr("切换主题")
            }, {
                "key": "Alt+A",
                "value": translator.tr("图表")
            }, {
                "key": "Alt+B",
                "value": translator.tr("工具箱")
            }, {
                "key": "Alt+C",
                "value": translator.tr("公链")
            }, {
                "key": "Ctrl+Alt+H",
                "value": translator.tr("显示窗口(全局按键)")
            }, {
                "key": "Alt+H",
                "value": translator.tr("主页")
            }, {
                "key": "Alt+N",
                "value": translator.tr("资讯")
            }, {
                "key": "Alt+S",
                "value": translator.tr("设置")
            }, {
                "key": "Alt+1",
                "value": translator.tr("第1列排序")
            }, {
                "key": "Alt+2",
                "value": translator.tr("第2列排序")
            }, {
                "key": "Alt+3",
                "value": translator.tr("第3列排序")
            }, {
                "key": "Alt+4",
                "value": translator.tr("第4列排序")
            }, {
                "key": "Alt+5",
                "value": translator.tr("第5列排序")
            }, {
                "key": "Alt+6",
                "value": translator.tr("第6列排序")
            }, {
                "key": "Alt+7",
                "value": translator.tr("第7列排序")
            }, {
                "key": "Alt+8",
                "value": translator.tr("第8列排序")
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

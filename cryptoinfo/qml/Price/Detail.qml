import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: root

    property alias model: repeater.model

    width: parent.width
    height: content.height
    visible: item._itemChecked
    model: [{
        "key": translator.tr("名称"),
        "value": modelData.name
    }, {
        "key": translator.tr("1小时行情"),
        "value": utilityFn.toPercentString(modelData.percent_change_1h)
    }, {
        "key": translator.tr("24小时行情"),
        "value": utilityFn.toPercentString(modelData.percent_change_24h)
    }, {
        "key": translator.tr("7天小时行情"),
        "value": utilityFn.toPercentString(modelData.percent_change_7d)
    }, {
        "key": translator.tr("24小时交易量(美元)"),
        "value": utilityFn.toFixedPrice(modelData.volume_24h_usd)
    }, {
        "key": translator.tr("市值(美元)"),
        "value": utilityFn.toFixedPrice(modelData.market_cap_usd)
    }, {
        "key": translator.tr("可用流通量"),
        "value": utilityFn.toFixedPrice(modelData.available_supply)
    }, {
        "key": translator.tr("最大流通量"),
        "value": utilityFn.toFixedPrice(modelData.max_supply)
    }, {
        "key": translator.tr("更新时间"),
        "value": utility.get_time_from_utc_seconds_qml(modelData.last_updated)
    }]

    Column {
        id: content

        width: parent.width

        Repeater {
            id: repeater

            delegate: Rectangle {
                property bool _entered: false

                width: root.width
                height: reRow.height
                color: _entered ? theme.itemEnterColor : "transparent"

                Row {
                    id: reRow

                    Base.ItemLabel {
                        text: modelData.key
                        width: root.width / 2
                    }

                    Base.ItemLabel {
                        text: modelData.value
                        width: root.width / 2
                    }

                }

                MouseArea {
                    anchors.fill: parent
                    onDoubleClicked: item._itemChecked = false
                    hoverEnabled: true
                    onExited: parent._entered = false
                    onEntered: parent._entered = true
                }

            }

        }

    }

    Rectangle {
        anchors.bottom: parent.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        width: parent.width
        height: 1
        color: theme.borderColor
    }

}

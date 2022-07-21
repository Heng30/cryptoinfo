import QtQuick 2.15
import QtQuick.Controls 2.15
import MonitorEthSortKey 1.0
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: header

    width: parent.width
    height: content.height
    color: "transparent"

    Row {
        id: content

        width: parent.width

        Repeater {
            id: repeater

            property var keyModel

            keyModel: [MonitorEthSortKey.Unknown, MonitorEthSortKey.Unknown, MonitorEthSortKey.Unknown, MonitorEthSortKey.Unknown, MonitorEthSortKey.Unknown, MonitorEthSortKey.TxValue]
            model: [translator.tr("..."), translator.tr("区块时间"), translator.tr("转帐记录"), translator.tr("发送"), translator.tr("接收"), translator.tr("数量")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    monitor_eth_model.toggle_sort_dir_qml();
                    monitor_eth_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

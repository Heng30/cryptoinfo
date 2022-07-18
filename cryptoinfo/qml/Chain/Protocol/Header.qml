import QtQuick 2.15
import QtQuick.Controls 2.15
import ChainProtocolSortKey 1.0
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

            keyModel: [ChainProtocolSortKey.Index, ChainProtocolSortKey.Name, ChainProtocolSortKey.Symbol, ChainProtocolSortKey.TVL, ChainProtocolSortKey.Staking, ChainProtocolSortKey.MarketCap, ChainProtocolSortKey.Per24H, ChainProtocolSortKey.Per7D]
            model: [translator.tr("排名"), translator.tr("名称"), translator.tr("代币"), translator.tr("锁仓量"), translator.tr("质押"), translator.tr("市值"), translator.tr("24行情"), translator.tr("7d行情")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    chain_protocol_model.toggle_sort_dir_qml();
                    chain_protocol_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

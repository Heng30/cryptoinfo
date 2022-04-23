import QtQuick 2.15
import QtQuick.Controls 2.15
import DefiProtocolSortKey 1.0
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: header

    width: parent.width
    height: content.height
    color: theme.priceHeaderBG

    Row {
        id: content

        width: parent.width

        Repeater {
            id: repeater

            property var keyModel

            keyModel: [DefiProtocolSortKey.Index, DefiProtocolSortKey.Name, DefiProtocolSortKey.Symbol, DefiProtocolSortKey.TVL, DefiProtocolSortKey.Staking, DefiProtocolSortKey.MarketCap, DefiProtocolSortKey.Per24H, DefiProtocolSortKey.Per7D]
            model: [translator.tr("排名"), translator.tr("名称"), translator.tr("代币"), translator.tr("锁仓量"), translator.tr("质押"), translator.tr("市值"), translator.tr("24行情"), translator.tr("7d行情")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    defi_protocol_model.toggle_sort_dir();
                    defi_protocol_model.sort_by_key(repeater.keyModel[index]);
                }
            }

        }

    }

}

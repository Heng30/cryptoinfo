import QtQuick 2.15
import QtQuick.Controls 2.15
import EthTokenSortKey 1.0
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

            keyModel: [EthTokenSortKey.Unknown, EthTokenSortKey.Unknown, EthTokenSortKey.Name, EthTokenSortKey.Symbol, EthTokenSortKey.Unknown, EthTokenSortKey.Price, EthTokenSortKey.MarketCap, EthTokenSortKey.Volumn, EthTokenSortKey.CirQuantity, EthTokenSortKey.IssueQuantity ]
            model: [translator.tr("..."), translator.tr("发行日期"), translator.tr("名称"), translator.tr("代币"), translator.tr("地址"), translator.tr("价格"), translator.tr("市值"), translator.tr("交易量"), translator.tr("当前流通量"), translator.tr("总流通量")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    chain_eth_token_model.toggle_sort_dir_qml();
                    chain_eth_token_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

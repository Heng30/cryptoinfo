import QtQuick 2.15
import QtQuick.Controls 2.15
import DefiChainSortKey 1.0
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

            keyModel: [DefiChainSortKey.Index, DefiChainSortKey.Name, DefiChainSortKey.Symbol, DefiChainSortKey.TVL]
            model: [translator.tr("排名"), translator.tr("名称"), translator.tr("代币"), translator.tr("锁仓量")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    defi_chain_model.toggle_sort_dir_qml();
                    defi_chain_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

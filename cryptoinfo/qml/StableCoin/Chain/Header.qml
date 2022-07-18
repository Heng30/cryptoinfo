import QtQuick 2.15
import QtQuick.Controls 2.15
import StableCoinChainSortKey 1.0
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

            keyModel: [StableCoinChainSortKey.Index, StableCoinChainSortKey.Name, StableCoinChainSortKey.Circulating]
            model: [translator.tr("排名"), translator.tr("公链(代币)"), translator.tr("稳定币流通量")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    stable_coin_chain_model.toggle_sort_dir_qml();
                    stable_coin_chain_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

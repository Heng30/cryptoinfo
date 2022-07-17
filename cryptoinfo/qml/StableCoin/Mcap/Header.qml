import QtQuick 2.15
import QtQuick.Controls 2.15
import StableCoinMcapSortKey 1.0
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

            keyModel: [StableCoinMcapSortKey.Index, StableCoinMcapSortKey.Name, StableCoinMcapSortKey.Symbol, StableCoinMcapSortKey.Circulating, StableCoinMcapSortKey.Price, StableCoinMcapSortKey.Source]
            model: [translator.tr("排名"), translator.tr("名称"), translator.tr("代币"), translator.tr("流通量"), translator.tr("价格"), translator.tr("数据来源")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    stable_coin_mcap_model.toggle_sort_dir_qml();
                    stable_coin_mcap_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import ChainYieldSortKey 1.0
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

    keyModel: [ChainYieldSortKey.Index, ChainYieldSortKey.Chain, ChainYieldSortKey.Symbol, ChainYieldSortKey.Project, ChainYieldSortKey.Pool, ChainYieldSortKey.Exposure, ChainYieldSortKey.Tvl, ChainYieldSortKey.Apy, ChainYieldSortKey.StableCoin]
            model: [translator.tr("排名"), translator.tr("公链"), translator.tr("代币"), translator.tr("项目"), translator.tr("池子"), translator.tr("交易对"), translator.tr("锁仓量"), translator.tr("年化率"), translator.tr("稳定币") ]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    chain_yield_model.toggle_sort_dir_qml();
                    chain_yield_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

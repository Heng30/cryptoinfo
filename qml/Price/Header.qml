import QtQuick 2.15
import QtQuick.Controls 2.15
import PriceSortKey 1.0
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: header

    width: parent.width
    height: content.height
    color: theme.priceHeaderBG

    Row {
        id: content

        width: parent.width

        Base.ItemText {
            id: markerHeaderField

            width: height
            text: "..."
            onClicked: {
                pricer_model.toggle_sort_dir();
                pricer_model.sort_by_key(PriceSortKey.Marked);
            }
        }

        Repeater {
            id: repeater

            property var keyModel

            keyModel: [PriceSortKey.Index, PriceSortKey.Symbol, PriceSortKey.Price, PriceSortKey.Per24H, PriceSortKey.Per7D, PriceSortKey.Volume24H, PriceSortKey.Floor]
            model: [translator.tr("排名"), translator.tr("代币"), translator.tr("价格"), translator.tr("24h行情"), translator.tr("7d行情"), translator.tr("24h交易量"), translator.tr("地板价")]

            delegate: Base.ItemText {
                width: (parent.width - markerHeaderField.width) / repeater.model.length
                text: modelData
                onClicked: {
                    pricer_model.toggle_sort_dir();
                    pricer_model.sort_by_key(repeater.keyModel[index]);
                }
            }

        }

    }

}

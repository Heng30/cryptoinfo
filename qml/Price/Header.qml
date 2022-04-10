import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: header

    width: ListView.view.width
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
                pricer_model.sort_by_key("marked");
            }
        }

        Repeater {
            id: repeater

            property var keyModel

            keyModel: ["index", "symbol", "price", "24h%", "7d%", "24h_volume"]
            model: [translator.tr("市值"), translator.tr("代币"), translator.tr("价格"), translator.tr("24h行情"), translator.tr("7d行情"), translator.tr("24h交易量")]

            delegate: Base.ItemText {
                width: index === 0 ? root._smallItemWidth : (parent.width - root._smallItemWidth - markerHeaderField.width) / (repeater.model.length - 1)
                text: modelData
                onClicked: {
                    pricer_model.toggle_sort_dir();
                    pricer_model.sort_by_key(repeater.keyModel[index]);
                }
            }

        }

    }

}

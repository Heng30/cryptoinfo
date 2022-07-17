import QtQuick 2.15
import QtQuick.Controls 2.15
import ExchangeBtcSortKey 1.0
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

            keyModel: [ExchangeBtcSortKey.Unknown, ExchangeBtcSortKey.Name, ExchangeBtcSortKey.Balance, ExchangeBtcSortKey.Income, ExchangeBtcSortKey.Rate]
            model: [translator.tr("..."), translator.tr("名称"), translator.tr("余额"), translator.tr("流入量"), translator.tr("24小时变动")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    exchange_btc_model.toggle_sort_dir_qml();
                    exchange_btc_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

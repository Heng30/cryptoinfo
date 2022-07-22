import QtQuick 2.15
import QtQuick.Controls 2.15
import AddressEthSortKey 1.0
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

            keyModel: [AddressEthSortKey.Unknown, AddressEthSortKey.Unknown, AddressEthSortKey.Balance, AddressEthSortKey.Percentage, AddressEthSortKey.Transactions]
            model: [translator.tr("..."), translator.tr("地址"), translator.tr("数量"), translator.tr("占比"), translator.tr("转帐次数")]

            delegate: Base.ItemText {
                width: parent.width / repeater.model.length
                text: modelData
                onClicked: {
                    address_eth_model.toggle_sort_dir_qml();
                    address_eth_model.sort_by_key_qml(repeater.keyModel[index]);
                }
            }

        }

    }

}

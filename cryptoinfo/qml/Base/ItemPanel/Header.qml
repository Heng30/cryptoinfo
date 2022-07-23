import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    width: parent.width
    height: content.height
    color: "transparent"

    Row {
        id: content

        width: parent.width

        Repeater {
            model: headerModel

            delegate: Base.ItemText {
                width: parent.width / headerModel.length
                text: modelData
                onClicked: {
                    if (!itemPanel.listModel.toggle_sort_dir_qml || !itemPanel.listModel.sort_by_key_qml)
                        return ;

                    itemPanel.listModel.toggle_sort_dir_qml();
                    itemPanel.listModel.sort_by_key_qml(itemPanel.headerSortKeyModel[index]);
                }
            }

        }

    }

}

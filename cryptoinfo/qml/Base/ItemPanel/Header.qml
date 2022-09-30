import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: header

    property var headerItemWidthList: []

    function _calHeaderItemWidthList() {
        if (itemPanel.itemWidthList.length <= 0 || itemPanel.itemWidthList.length !== itemPanel.headerModel.length)
            return ;

        headerItemWidthList = utilityFn.calWidth(header.width, itemPanel.itemWidthList);
    }

    width: parent.width
    height: content.height
    color: "transparent"
    onWidthChanged: _calHeaderItemWidthList()

    Row {
        id: content

        width: parent.width

        Repeater {
            model: headerModel

            delegate: Base.ItemText {
                width: headerItemWidthList.length === headerModel.length ? headerItemWidthList[index] : header.width / headerModel.length
                text: modelData
                onClicked: {
                    if (!itemPanel.listModel.toggle_sort_dir_qml || !itemPanel.listModel.sort_by_key_qml || itemPanel.headerSortKeyModel.length <= 0)
                        return ;

                    itemPanel.listModel.toggle_sort_dir_qml();
                    itemPanel.listModel.sort_by_key_qml(itemPanel.headerSortKeyModel[index]);
                }
            }

        }

    }

}

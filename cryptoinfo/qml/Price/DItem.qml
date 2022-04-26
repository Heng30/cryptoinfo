import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: item

    property bool _itemChecked: false

    width: ListView.view.width
    height: column.height

    Column {
        id: column

        width: parent.width

        PItem {
        }

        Detail {
        }

    }

}

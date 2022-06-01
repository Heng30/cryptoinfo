import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: handbook

    signal addItemSig()
    signal downItemOrderSig(int downItemIndex)
    signal upItemOrderSig(int upItemIndex)

    width: parent.width
    implicitHeight: 100

    Chart {
        id: chart
    }

    Column {
        anchors.fill: parent
        anchors.margins: theme.itemMargins
        spacing: theme.itemSpacing

        Rectangle {
            width: parent.width
            height: parent.height - footer.height - parent.spacing
            border.width: 1
            border.color: "steelblue"
            color: "transparent"

            ListView {
                id: listView

                anchors.fill: parent
                anchors.margins: theme.itemMargins
                spacing: theme.itemSpacing
                clip: true
                model: handbook_model

                ScrollBar.vertical: Base.SBar {
                    policy: ScrollBar.AlwaysOff
                }

                delegate: DItem {
                }

            }

        }

        Footer {
            id: footer
        }

    }

}

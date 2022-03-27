import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias text: textField.text
    property alias readOnly: textField.readOnly
    property alias leftPadding: content.leftPadding
    property alias labelText: label.text

    signal inc()
    signal dec()

    implicitHeight: content.height
    implicitWidth: content.width

    Row {
        id: content

        anchors.verticalCenter: parent.verticalCenter
        height: label.height

        ItemLabel {
            id: label
        }

        Item {
            height: parent.height
            width: height

            ImageButton {
                anchors.centerIn: parent
                height: parent.height / 2
                width: height
                source: "qrc:/res/image/minus.png"
                onClicked: root.dec()
            }

        }

        TextField {
            id: textField

            height: parent.height
            color: theme.fontColor
            background: null
        }

        Item {
            height: parent.height
            width: height

            ImageButton {
                anchors.centerIn: parent
                height: parent.height / 2
                width: height
                source: "qrc:/res/image/add.png"
                onClicked: root.inc()
            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: test

    width: parent.width
    headerText: translator.tr("组件测试")
    spacing: theme.itemSpacing

    contentItem: Column {
        Item {
            width: parent.width
            height: 20

            Base.SlideBar {
                anchors.centerIn: parent
                width: parent.width - 20
                from: 1
                to: 100
                value: 1
                showValue: true
                tipText: value.toFixed(2)
                onValueChanged: {
                    progBar.value = value;
                    dial.value = value;
                }
            }

        }

        Item {
            width: parent.width
            height: 20

            Base.ProgBar {
                id: progBar

                anchors.centerIn: parent
                width: parent.width - 20
                height: 4
                from: 1
                to: 100
                value: 0
            }

        }

        Base.BDial {
            id: dial

            from: 1
            to: 100
        }

        Base.TxtButton {
            text: "Test"
            tipText: "It is a test."
            anchors.horizontalCenter: parent.horizontalCenter
            onClicked: console.log("It is a test.")
        }

        Base.TxtArea {
            width: test.width
            height: 100
            text: "TextArea\n...\n...\n...\n...\n...\n...\n"
        }

    }

}

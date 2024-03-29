import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: test

    width: parent.width
    headerText: translator.tr("组件测试")
    spacing: theme.itemSpacing

    contentItem: Column {
        Base.BTab {
            width: parent.width
            height: 300
            model: [
                QtObject {
                    property string tabText: "tab1"
                    property Component sourceComponent

                    sourceComponent: Rectangle {
                        color: "red"
                    }

                },
                QtObject {
                    property string tabText: "tab2"
                    property Component sourceComponent

                    sourceComponent: Rectangle {
                        color: "green"
                    }

                }
            ]
        }

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

        Base.TxtArea {
            width: test.width
            height: 100
            text: "TextArea\n...\n...\n...\n...\n...\n...\n"
        }

        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: theme.itemSpacing

            Base.TxtButton {
                text: "Test"
                tipText: "It is a test."
                onClicked: console.log("It is a test.")
            }

            Base.TxtButton {
                text: translator.tr("测试MsgBox")
                onClicked: {
                    if (msgBox.boxData.length > 0 && msgBox.boxData[0].okCB === null && msgBox.boxData[0].cancellCB === null)
                        msgBox.boxData.shift();

                    var isWarnMsg = Math.round(Math.random() * 100) % 2 ? true : false;
                    var msg = translator.tr("测试") + "-" + Math.round(Math.random() * 100);
                    var okCB = Math.round(Math.random() * 100) % 2 ? null : function() {
                        console.log("ok callback");
                    };
                    var cancellCB = Math.round(Math.random() * 100) % 2 ? null : function() {
                        console.log("cancell callback");
                    };
                    msgBox.add(msg, isWarnMsg, okCB, cancellCB);
                }
            }

            Base.TxtButton {
                text: translator.tr("测试MsgTip")
                onClicked: {
                    var isWarnMsg = Math.round(Math.random() * 100) % 2 ? true : false;
                    var msg = translator.tr("测试") + "-" + Math.round(Math.random() * 100);
                    var msg_2 = msg + "-2";
                    msgTip.add(msg, isWarnMsg);
                    msgTip.add(msg_2, isWarnMsg);
                }
            }

        }

        Base.SPage {
            width: parent.width
            maxPages: 50
            onClicked: console.log(index)
        }

    }

}

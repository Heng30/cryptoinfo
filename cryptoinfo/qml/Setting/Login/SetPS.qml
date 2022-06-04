import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.CDialog {
    id: setPS

    anchors.centerIn: parent
    width: 300
    height: 200
    headerText: translator.tr("设置登陆密码")
    modal: true

    sourceComponent: Item {
        Column {
            id: content

            anchors.centerIn: parent
            spacing: theme.itemSpacing * 2
            Component.onCompleted: setPS.visibleChanged.connect(function() {
                password.text = "";
                passwordAgain.text = "";
                password.forceFocus();
            })

            Row {
                anchors.horizontalCenter: parent.horizontalCenter

                Base.InputBar {
                    id: password

                    anchors.verticalCenter: parent.verticalCenter
                    width: setPS.width * 4 / 5
                    textInput.echoMode: TextInput.Password
                    underText: translator.tr("请输入密码")
                }

            }

            Row {
                anchors.horizontalCenter: parent.horizontalCenter

                Base.InputBar {
                    id: passwordAgain

                    anchors.verticalCenter: parent.verticalCenter
                    width: setPS.width * 4 / 5
                    textInput.echoMode: TextInput.Password
                    underText: translator.tr("请再次输入密码")
                }

            }

            Row {
                spacing: theme.itemSpacing * 6
                anchors.horizontalCenter: parent.horizontalCenter

                Base.TxtButton {
                    id: cancelBtn

                    text: translator.tr("取消")
                    onClicked: {
                        setPS.visible = false;
                    }
                }

                Base.TxtButton {
                    id: okBtn

                    text: translator.tr("确定")
                    onClicked: {
                        if (password.text !== passwordAgain.text) {
                            msgTip.add(translator.tr("输入的密码不一致!"), true);
                            return ;
                        }
                        if (password.text.length <= 0) {
                            msgTip.add(translator.tr("密码不能为空!"), true);
                            return ;
                        }
                        if (login_table.set_password(password.text)) {
                            msgTip.add(translator.tr("设置密码成功!"), false);
                            setPS.visible = false;
                        } else {
                            msgTip.add(translator.tr("设置密码失败!"), true);
                        }
                    }
                }

            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.CDialog {
    id: delPS

    anchors.centerIn: parent
    width: 300
    height: 200
    headerText: translator.tr("删除登陆密码")
    modal: true

    sourceComponent: Item {
        Column {
            id: content

            anchors.centerIn: parent
            spacing: theme.itemSpacing * 2
            Component.onCompleted: delPS.visibleChanged.connect(function() {
                password.text = "";
                password.forceFocus();
            })

            Row {
                anchors.horizontalCenter: parent.horizontalCenter

                Base.InputBar {
                    id: password

                    anchors.verticalCenter: parent.verticalCenter
                    width: delPS.width * 4 / 5
                    textInput.echoMode: TextInput.Password
                    underText: translator.tr("请输入密码")
                }

            }

            Row {
                spacing: theme.itemSpacing * 6
                anchors.horizontalCenter: parent.horizontalCenter

                Base.TxtButton {
                    id: cancelBtn

                    text: translator.tr("取消")
                    onClicked: {
                        delPS.visible = false;
                    }
                }

                Base.TxtButton {
                    id: okBtn

                    text: translator.tr("确定")
                    onClicked: {
                        if (password.text.length <= 0) {
                            msgTip.add(translator.tr("密码不能为空!"), true);
                            return ;
                        }
                        if (login_table.del_password_qml(password.text)) {
                            delPS.visible = false;
                            msgTip.add(translator.tr("删除密码成功!"), false);
                        } else {
                            msgTip.add(translator.tr("删除密码失败!"), true);
                        }
                    }
                }

            }

        }

    }

}

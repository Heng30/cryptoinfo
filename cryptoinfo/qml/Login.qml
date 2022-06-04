import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: login

    property bool canLogin: false

    implicitWidth: 450
    implicitHeight: content.height + theme.itemSpacing * 8
    visible: config.enable_login_password
    Component.onCompleted: {
        if (login.visible)
            password.forceFocus();
        else
            login.canLogin = true;
    }

    Column {
        id: content

        width: parent.width
        anchors.centerIn: parent
        spacing: theme.itemSpacing * 2

        Row {
            anchors.horizontalCenter: parent.horizontalCenter

            Base.InputBar {
                id: password

                anchors.verticalCenter: parent.verticalCenter
                width: login.width / 3
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
                onClicked: utilityFn.quit()
            }

            Base.TxtButton {
                id: loginBtn

                text: translator.tr("登入")
                onClicked: {
                    login.canLogin = true;
                }
            }

        }

    }

}
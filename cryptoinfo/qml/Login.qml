import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: login

    property bool canLogin: false

    function _auth() {
        if (login_table.auth_qml(password.text))
            login.canLogin = true;
        else
            msgTip.add(translator.tr("登陆密码不存在!"), true);
    }

    implicitWidth: 450
    implicitHeight: content.height + theme.itemSpacing * 8
    visible: config.enable_login_password
    Component.onCompleted: {
        if (login.visible)
            password.forceFocus();
        else
            login.canLogin = true;
    }

    Row {
        id: content

        anchors.centerIn: parent
        spacing: theme.itemSpacing * 8

        Base.InputBar {
            id: password

            anchors.verticalCenter: parent.verticalCenter
            width: login.width / 3
            height: row.height
            textInput.echoMode: TextInput.Password
            underText: translator.tr("请输入密码")
            onAccepted: login._auth()
        }

        Row {
            id: row
            anchors.verticalCenter: parent.verticalCenter
            spacing: theme.itemSpacing * 4

            Base.TxtButton {
                id: cancelBtn

                anchors.verticalCenter: parent.verticalCenter
                horizontalPadding: theme.itemPadding * 8
                verticalPadding: theme.itemPadding * 2
                text: translator.tr("取消")
                onClicked: utilityFn.quit()
            }

            Base.TxtButton {
                id: loginBtn

                anchors.verticalCenter: parent.verticalCenter
                horizontalPadding: theme.itemPadding * 8
                verticalPadding: theme.itemPadding * 2
                text: translator.tr("登入")
                onClicked: login._auth()
            }

        }

    }

}

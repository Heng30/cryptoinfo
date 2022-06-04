import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: panel

    width: parent.width
    headerText: translator.tr("登陆密码设置")
    spacing: theme.itemSpacing

    contentItem: Column {
        spacing: theme.itemSpacing

        Row {
            width: parent.width

            Item {
                width: parent.width / 2
                height: setPSBtn.height + theme.itemMargins * 2

                Base.TxtButton {
                    id: setPSBtn

                    anchors.left: parent.left
                    anchors.leftMargin: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("设置")
                    onClicked: setPS.visible = true
                }

            }

            Item {
                width: parent.width / 2
                height: delPSBtn.height + theme.itemMargins * 2

                Base.TxtButton {
                    id: delPSBtn

                    anchors.left: parent.left
                    anchors.leftMargin: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("删除")
                    onClicked: delPS.visible = true
                }

            }

        }

    }

}

import QtQuick 2.15
import QtQuick.Dialogs 1.3
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    property var _btnCB: null

    width: parent.width
    headerText: translator.tr("备份恢复")
    spacing: theme.itemSpacing

    contentItem: Item {
        width: parent.width
        height: row.height

        FileDialog {
            id: dialog

            folder: shortcuts.home
            onAccepted: {
                if (!_btnCB)
                    return ;

                if (selectFolder)
                    _btnCB(folder);
                else
                    _btnCB(fileUrl);
            }
        }

        Row {
            id: row

            width: parent.width

            Item {
                width: parent.width / 2
                height: backup.height + theme.itemMargins * 2

                Base.TxtButton {
                    id: backup

                    anchors.left: parent.left
                    anchors.leftMargin: theme.itemMargins
                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("备份")
                    onClicked: {
                        dialog.selectFolder = true;
                        dialog.nameFilters = ["All files (*)"];
                        dialog.title = translator.tr("请选择导出目录");
                        _btnCB = function _btnCB(dir) {
                            console.log("You chose: " + dir);
                        };
                        dialog.open();
                    }
                }

            }

            Item {
                width: parent.width / 2
                height: recover.height + theme.itemMargins

                Base.TxtButton {
                    id: recover

                    anchors.left: parent.left
                    anchors.leftMargin: theme.itemMargins * 2
                    anchors.verticalCenter: parent.verticalCenter
                    text: translator.tr("恢复")
                    onClicked: {
                        dialog.selectFolder = false;
                        dialog.nameFilters = ["Database files (*.db)"];
                        dialog.title = translator.tr("请选择导入文件");
                        _btnCB = function _btnCB(file) {
                            console.log("You chose: " + file);
                        };
                        dialog.open();
                    }
                }

            }

        }

    }

}

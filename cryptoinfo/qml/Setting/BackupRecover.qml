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
                            var filename = "cryptoinfo" + "-" + Date.now() + ".tar.gz";
                            var dst = String(dir).replace("file://", "") + "/" + filename;
                            if (utility.pack(filename, "backup", config.config_dir, config.data_dir) && utility.move_file(filename, dst))
                                msgTip.add(translator.tr("备份成功!"), false);
                            else
                                msgTip.add(translator.tr("备份失败!"), true);
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
                        dialog.nameFilters = ["Database files (*.tar.gz)"];
                        dialog.title = translator.tr("请选择导入文件");
                        _btnCB = function _btnCB(file) {
                            var filepath = String(file).replace("file://", "");
                            var config_dir = config.config_dir;
                            var data_dir = config.data_dir;

                            if (utility.unpack(filepath) && utility.move_files("backup/config", config_dir) && utility.move_files("backup/data", data_dir))
                                msgTip.add(translator.tr("恢复成功, 请重启程序!"), false);
                            else
                                msgTip.add(translator.tr("恢复失败!"), true);

                            utility.remove_dirs("backup");
                        };
                        dialog.open();
                    }
                }

            }

        }

    }

}

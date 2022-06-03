import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: mode

    width: parent.width
    headerText: translator.tr("窗口模式")
    spacing: theme.itemSpacing

    contentItem: Row {
        Base.RadioButton {
            id: windowMode

            property bool _flag: !config.is_window_mode

            width: parent.width / 2
            text: translator.tr("Window 模式")
            checked: !dialogMode.checked
            onCheckedChanged: {
                // 排除启动程序时，触发的信号
                if (_flag) {
                    msgTip.add(translator.tr("重启程序, 使配置生效."), false);
                    config.is_window_mode = checked;
                    config.save();
                }
                _flag = true;
            }
        }

        Base.RadioButton {
            id: dialogMode

            width: parent.width / 2
            height: windowMode.height
            text: translator.tr("Dialog 模式")
            checked: !config.is_window_mode
        }

    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base

Row {
    id: right

    property list<QtObject> imageModel

    anchors.right: parent.right
    height: parent.height
    spacing: theme.itemSpacing
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/setting.png"
            property string tipText: translator.tr("设置")
            property bool visible: true
            property bool checked: _settingIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Setting;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/about.png"
            property string tipText: translator.tr("关于")
            property bool visible: true
            property var clicked: function() {
                about.open();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/theme.png"
            property string tipText: translator.tr("皮肤")
            property bool visible: true
            property var clicked: function() {
                config.is_dark_theme = !config.is_dark_theme;
                config.save_qml();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/max-height.png"
            property string tipText: translator.tr("缩放")
            property bool visible: true
            property var clicked: function() {
                homePage._isMaxHeight = !homePage._isMaxHeight;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/logout.png"
            property string tipText: translator.tr("登出")
            property bool visible: true
            property var clicked: function() {
                splash.showLogin();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/eye-hiden.png"
            property string tipText: translator.tr("隐藏(Ctrl+Alt+H显示)")
            property bool visible: true
            property var clicked: function() {
                main.hide();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/exit.png"
            property string tipText: translator.tr("关闭")
            property bool visible: true
            property var clicked: function() {
                utilityFn.quit();
            }
        }
    ]

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base

Row {
    id: left

    property list<QtObject> imageModel

    height: parent.height
    spacing: theme.itemSpacing
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/home.png"
            property string tipText: translator.tr("主页")
            property bool visible: !root._homeIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Price;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/setting.png"
            property string tipText: translator.tr("设置")
            property bool visible: !root._settingIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Setting;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/tool-box.png"
            property string tipText: translator.tr("工具箱")
            property bool visible: !root._toolBoxIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.ToolBox;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/note.png"
            property string tipText: translator.tr("笔记")
            property bool visible: !root._noteIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Note;
                root.noteClicked();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/todo-list.png"
            property string tipText: translator.tr("代办事项")
            property bool visible: !root._notifyIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Todo;
            }
        }
    ]

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }

}

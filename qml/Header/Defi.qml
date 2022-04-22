import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: price

    property list<QtObject> imageModel

    height: parent.height
    spacing: theme.itemSpacing
    visible: root._defiIsChecked
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/clear.png"
            property string tipText: translator.tr("清除")
            property bool visible: true
            property var clicked: function() {
                defi_model.clear();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/refresh.png"
            property string tipText: translator.tr("刷新")
            property bool visible: true
            property var clicked: function() {
                root.defiRefresh();
            }
        }
    ]

    Base.Sep {
        height: parent.height / 2
        anchors.verticalCenter: parent.verticalCenter
    }

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }
}

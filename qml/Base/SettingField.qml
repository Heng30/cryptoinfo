import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias headerBGColor: settingHeader.color
    property alias spacing: innerContent.spacing
    property alias headerText: settingHeader.text
    property alias contentItem: loader.sourceComponent
    property alias contentSource: loader.source

    implicitWidth: 300
    height: innerContent.height

    Column {
        property bool _isChecked: true
        id: innerContent

        width: parent.width

        ItemLabel {
            id: settingHeader

            width: parent.width
            textFontBold: true
            textFontPixelSize: theme.fontPixelNormal + 4
            color: theme.settingFieldHeaderColor

            onClicked: innerContent._isChecked = !innerContent._isChecked
        }

        Loader {
            id: loader

            visible: innerContent._isChecked;
            width: parent.width
        }

    }

}

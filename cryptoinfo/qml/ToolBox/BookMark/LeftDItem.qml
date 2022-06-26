import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Rectangle {
    id: dItem

    property bool _isEntered: false
    property bool _isChecked: index === leftField.checkedIndex

    width: ListView.view.width
    height: label.height + theme.itemMargins * 2
    color: _isChecked ? theme.itemCheckedBG : (_isEntered ? theme.itemEnteredBG: "transparent")

    Base.ItemLabel {
        id: label

        width: parent.width
        anchors.centerIn: parent
        text: index
        onEntered: dItem._isEntered = true
        onExited: dItem._isEntered = false
        onClicked: {
            leftField.checkedIndex = index
        }
    }

}

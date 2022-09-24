import QtQml 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property real textVMargins: theme.itemPadding * 2
    property alias text: label.text
    property alias textColor: label.color
    property alias textFontBold: label.font.bold
    property alias textFontPixelSize: label.font.pixelSize
    property bool run: true
    property bool _entered: false

    signal entered()
    signal exited()

    color: theme.carouselBG
    implicitWidth: label.width
    implicitHeight: label.height + textVMargins

    Label {
        id: label

        anchors.verticalCenter: parent.verticalCenter
        leftPadding: theme.itemPadding
        rightPadding: theme.itemPadding
        color: theme.fontColor
        font.pixelSize: theme.fontPixelNormal
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: {
            root._entered = true;
            root.entered();
        }
        onExited: {
            root._entered = false;
            root.exited();
        }
    }

    Timer {
        id: timer

        interval: 100
        running: label.width > root.width && !root._entered && root.run
        repeat: true
        onTriggered: {
            if (label.text.length <= 0)
                return ;

            var tArray = Array.from(label.text);
            tArray.push(tArray.shift());
            label.text = tArray.join("");
        }
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15

ComboBox {
    id: root

    property color textColor: theme.fontColor
    property color bgColor: theme.bgColor
    property color itemEnteredBG: theme.itemEnteredBG
    property color indicatorFillColor: theme.invertBgColor
    property real itemSpacing: theme.itemSpacing
    property real fontTextPixelSize: theme.fontPixelNormal
    property alias popupHeight: textPopup.height

    implicitWidth: 200
    implicitHeight: fontTextPixelSize + 10
    onIndicatorFillColorChanged: canvas.requestPaint()

    //ComboBox上显示的内容的背景
    background: Rectangle {
        anchors.fill: parent
        color: root.bgColor
        border.width: 1
        border.color: root.textColor
    }

    indicator: Rectangle {
        anchors.right: parent.right
        anchors.rightMargin: 4
        anchors.verticalCenter: parent.verticalCenter
        width: root.fontTextPixelSize / 2
        height: width

        Canvas {
            id: canvas

            anchors.fill: parent
            onPaint: {
                var ctx = getContext("2d");

                ctx.beginPath();
                ctx.moveTo(0, 0);
                ctx.lineTo(width / 2, height);
                ctx.lineTo(width, 0);
                ctx.closePath();
                ctx.fillStyle = root.indicatorFillColor;
                ctx.fill();

                ctx.beginPath();
                ctx.moveTo(0, 0);
                ctx.lineTo(width /2, height);
                ctx.lineTo(0, height);
                ctx.closePath();
                ctx.fillStyle = root.bgColor;
                ctx.fill();

                ctx.beginPath();
                ctx.moveTo(width, 0);
                ctx.lineTo(width/2, height );
                ctx.lineTo(width, height);
                ctx.closePath();
                ctx.fillStyle = root.bgColor;
                ctx.fill();

            }
        }

    }

    // ComboBox上显示的内容
    contentItem: Text {
        id: ctxt

        verticalAlignment: Text.AlignVCenter
        leftPadding: root.itemSpacing
        rightPadding: root.itemSpacing
        width: root.width - root.indicator.width - leftPadding - rightPadding
        text: root.displayText
        font.pixelSize: root.fontTextPixelSize
        color: root.textColor
        clip: true
    }

    // popup的listview中的delegate
    delegate: ItemDelegate {
        id: itemDelegate

        width: root.width
        height: txt.height
        background: null

        Rectangle {
            property bool _isEntered: false

            anchors.fill: parent
            color: _isEntered ? root.itemEnteredBG : "transparent"

            MouseArea {
                anchors.fill: parent
                hoverEnabled: true
                onEntered: parent._isEntered = true
                onExited: parent._isEntered = false
                onClicked: itemDelegate.clicked()
            }

        }

        Text {
            id: txt

            anchors.verticalCenter: parent.verticalCenter
            text: modelData
            font.pixelSize: root.fontTextPixelSize
            color: root.textColor
        }

    }

    popup: Popup {
        id: textPopup
        property real _minHeight: (root.fontTextPixelSize + listview.spacing) * 5

        y: root.height - 1
        width: root.width
        height: listview.contentHeight > _minHeight ? _minHeight : listview.contentHeight + bottomMargin
        padding: root.itemSpacing
        bottomMargin: root.itemSpacing

        contentItem: ListView {
            id: listview

            clip: true
            spacing: root.itemSpacing * 2
            model: root.popup.visible ? root.delegateModel : null
            currentIndex: root.highlightedIndex

            ScrollIndicator.vertical: ScrollIndicator {
            }

        }

        //下拉框背景
        background: Rectangle {
            color: root.bgColor
            border.width: 1
            border.color: root.textColor
        }

    }

}

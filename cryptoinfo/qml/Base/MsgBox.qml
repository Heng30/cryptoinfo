import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15
import "qrc:/res/qml/Base" as Base

Popup {
    id: msgBox

    property bool isWarnMsg: true
    property var boxData: []

    function add(msg, isWarnMsg, okCB, cancellCB) {
        if (!okCB && !cancellCB)
            return ;

        var item = {
            "msg": msg,
            "isWarnMsg": isWarnMsg,
            "okCB": okCB,
            "cancellCB": cancellCB
        };
        boxData.push(item);
    }

    implicitWidth: 300
    implicitHeight: 180
    modal: true
    focus: true
    closePolicy: Popup.NoAutoClose
    padding: 0

    Column {
        anchors.margins: theme.itemMargins
        anchors.fill: parent
        spacing: theme.itemSpacing

        Row {
            width: parent.width
            height: parent.height - row.height - spacing
            spacing: theme.itemSpacing

            Item {
                width: image.width + theme.itemPadding
                height: parent.height

                Image {
                    id: image

                    anchors.centerIn: parent
                    fillMode: Image.PreserveAspectFit
                    source: msgBox.isWarnMsg ? "qrc:/res/image/warn.png" : "qrc:/res/image/info.png"

                    ColorOverlay {
                        anchors.fill: parent
                        source: parent
                        color: theme.imageColor
                    }

                }

            }

            Item {
                width: parent.width - image.width - parent.spacing
                height: parent.height
                clip: true

                Label {
                    id: label

                    anchors.verticalCenter: parent.verticalCenter
                    anchors.margins: theme.itemMargins
                    width: parent.width
                    wrapMode: Text.WordWrap
                    color: theme.fontColor
                    font.pixelSize: theme.fontPixelNormal
                }

            }

        }

        Row {
            id: row

            anchors.rightMargin: theme.itemMargins * 5
            width: parent.width - anchors.rightMargin
            spacing: theme.itemSpacing * 5
            layoutDirection: Qt.RightToLeft

            Base.TxtButton {
                id: okBtn

                property var cb: null

                visible: cb
                text: translator.tr("确定")
                onClicked: {
                    if (okBtn.cb)
                        okBtn.cb();

                    msgBox.boxData.shift();
                    msgBox.close();
                }
            }

            Base.TxtButton {
                id: cancellBtn

                property var cb: null

                height: okBtn.height
                visible: cb
                text: translator.tr("取消")
                onClicked: {
                    if (cancellBtn.cb)
                        cancellBtn.cb();

                    msgBox.boxData.shift();
                    msgBox.close();
                }
            }

        }

    }

    Timer {
        interval: 500
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            if (msgBox.boxData.length > 0) {
                var item = msgBox.boxData[0];
                label.text = item.msg;
                msgBox.isWarnMsg = item.isWarnMsg;
                okBtn.cb = item.okCB;
                cancellBtn.cb = item.cancellCB;
                msgBox.open();
            } else {
                msgBox.close();
            }
        }
    }

    background: Rectangle {
        anchors.fill: parent
        border.width: 2
        border.color: theme.borderColor
        color: theme.bgColor
    }

}

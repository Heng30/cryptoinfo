import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Popup {
    id: qrcode

    function show(index, name, addr, source) {
        nameLabel.text = name;
        imageBtn.source = "file://" + source;
        imageBtn._qrcodeIndex = index;
        imageBtn._addr = addr;
        qrcode.visible = true;
    }

    anchors.centerIn: parent
    width: column.width + theme.itemMargins * 4
    height: column.height + theme.itemMargins * 4
    modal: true
    focus: true
    padding: 0
    closePolicy: Popup.CloseOnPressOutside | Popup.CloseOnEscape

    Column {
        id: column

        anchors.centerIn: parent
        width: 300
        spacing: theme.itemSpacing * 2

        Base.ItemLabel {
            id: nameLabel

            anchors.margins: theme.itemMargins
            anchors.horizontalCenter: parent.horizontalCenter
            label.width: parent.width - anchors.margins * 2
            label.horizontalAlignment: Text.AlignHCenter
            wrapMode: Text.Wrap
        }

        Base.ImageButton {
            id: imageBtn

            property int _qrcodeIndex: 0
            property string _addr: ""

            width: Math.min(200, parent.width / 3 * 2)
            height: width
            anchors.horizontalCenter: parent.horizontalCenter
            tipText: translator.tr("点击刷新")
            enableColorOverlay: false
            onClicked: {
                var source = imageBtn.source;
                addrbook_model.set_item_qml(_qrcodeIndex, nameLabel.text, _addr);
                imageBtn.source = "";
                imageBtn.source = source;
            }
        }

        Base.ItemLabel {
            id: addrLabel

            anchors.horizontalCenter: parent.horizontalCenter
            anchors.margins: theme.itemMargins
            label.width: parent.width - anchors.margins * 2
            label.horizontalAlignment: Text.AlignHCenter
            wrapMode: Text.WrapAnywhere
            text: imageBtn._addr
            tipText: translator.tr("点击复制")

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    if (utility.copy_to_clipboard_qml(addrLabel.text))
                        msgTip.add(translator.tr("复制成功"), false)
                    else
                        msgTip.add(translator.tr("复制失败"), false)

                }
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

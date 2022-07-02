import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: panel

    anchors.fill: parent
    Component.onCompleted: bTab.clicked.connect(function(index) {
        if (index !== 0)
            return ;

        password.forceFocus();
    })

    Column {
        anchors.fill: parent
        spacing: theme.itemSpacing

        Row {
            id: row

            width: parent.width
            spacing: theme.itemSpacing

            Base.InputBar {
                id: password

                width: parent.width - row2.width - parent.spacing
            }

            Row {
                id: row2

                spacing: theme.itemSpacing * 2

                Base.TxtButton {
                    id: encryption

                    text: translator.tr("加密")
                    height: password.height
                    onClicked: {
                        if (!encipher.verify_qml(password.text, inputArea.text)) {
                            outputArea.text = outputArea.text + translator.tr("内部错误，加解密结果不一致.");
                            return ;
                        }
                        outputArea.text = encipher.encrypt_qml(password.text, inputArea.text);
                    }
                }

                Base.TxtButton {
                    id: decryption

                    text: translator.tr("解密")
                    height: password.height
                    onClicked: outputArea.text = encipher.decrypt_qml(password.text, inputArea.text)
                }

            }

        }

        Base.TxtArea {
            id: inputArea

            width: parent.width
            height: (parent.height - row.height - parent.spacing * 2) / 2
            border.color: theme.borderColor
            innerHeight: height
        }

        Base.TxtArea {
            id: outputArea

            width: parent.width
            height: inputArea.height
            border.color: theme.borderColor
            innerHeight: height
        }

    }

}

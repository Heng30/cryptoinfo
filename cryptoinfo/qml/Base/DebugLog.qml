import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.TxtArea {
    id: textArea

    function append(newText) {
        if (textArea.flickableItem.contentHeight - textArea.height <= 0) {
            textArea.text = textArea.text + newText;
        } else {
            if (textArea.flickableItem.contentY >= textArea.flickableItem.contentHeight - textArea.height) {
                textArea.text = textArea.text + newText;
                textArea.flickableItem.contentY = textArea.flickableItem.contentHeight - textArea.height;
            } else {
                textArea.text = textArea.text + newText;
            }
        }
    }
}

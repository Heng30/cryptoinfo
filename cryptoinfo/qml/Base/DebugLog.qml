import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.TxtArea {
    id: textArea

    function append(newText) {
        if (textArea.flickableItem.contentHeight - textArea.height <= 0) {
            var oldContentY = textArea.flickableItem.contentY;
            textArea.text = textArea.text + newText;
            textArea.flickableItem.contentY = oldContentY;
        } else {
            if (textArea.flickableItem.contentY >= textArea.flickableItem.contentHeight - textArea.height) {
                textArea.text = textArea.text + newText;
                textArea.flickableItem.contentY = textArea.flickableItem.contentHeight - textArea.height;
            } else {
                var oldContentY = textArea.flickableItem.contentY;
                textArea.text = textArea.text + newText;
                textArea.flickableItem.contentY = oldContentY;
            }
        }
    }

}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.TxtArea {
    id: textArea

    property var _lines: []

    function append(newText) {
        if (_lines.length > 50)
            _lines.shift();

        _lines.push(newText);

        newText = _lines.join("");
        if (textArea.flickableItem.contentHeight - textArea.height <= 0) {
            var oldContentY = textArea.flickableItem.contentY;
            textArea.text = newText;
            textArea.flickableItem.contentY = oldContentY;
        } else {
            if (textArea.flickableItem.contentY >= textArea.flickableItem.contentHeight - textArea.height) {
                textArea.text = newText;
                textArea.flickableItem.contentY = textArea.flickableItem.contentHeight - textArea.height;
            } else {
                var oldContentY = textArea.flickableItem.contentY;
                textArea.text = newText;
                textArea.flickableItem.contentY = oldContentY;
            }
        }
    }

}

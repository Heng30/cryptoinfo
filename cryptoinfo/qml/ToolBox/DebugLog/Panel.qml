import QtQml 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.DebugLog {
    id: debuglog
    width: parent.width
    readOnly: true
    scrollBarPolicy: ScrollBar.AlwaysOn
    border.width: 0

    Connections {
        function onText_changed() {
            if (debug_log.text.length <= 0) return;
            debuglog.append(debug_log.text.trim() + "\n\n");
        }

        target: debug_log
    }

    Connections {
        function onClear() {
            debuglog.text = "";
        }

        target: debug_log
    }

}

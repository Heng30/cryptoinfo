import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        debug_log.clear_qml();
    })
    visible: _debugLogIsChecked
}

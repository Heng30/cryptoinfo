import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        macro_event_model.clear_qml();
    })
    refreshClickedCB: (function() {
        macro_event_model.refresh_qml();
    })
    visible: _intelIsChecked && _macroEventTabIsChecked
}

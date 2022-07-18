import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        chain_protocol_model.clear_qml();
    })
    refreshClickedCB: (function() {
        chain_protocol_model.update_now = true;
    })
    visible: _chainIsChecked && _chainProtocolTabIsChecked
}

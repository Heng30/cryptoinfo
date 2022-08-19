import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        chain_yield_model.clear_qml();
    })
    refreshClickedCB: (function() {
        chain_yield_model.refresh_qml();
    })
    visible: _chainIsChecked && _chainYieldTabIsChecked
}

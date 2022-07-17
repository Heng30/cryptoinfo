import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        monitor_btc_model.clear_qml();
    })
    refreshClickedCB: (function() {
        monitor_btc_model.update_now = true;
    })
    visible: _monitorIsCheched && _monitorBtcTabIsChecked
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        monitor_eth_model.clear_qml();
    })
    refreshClickedCB: (function() {
        monitor_eth_model.up_refresh_qml();
    })
    visible: _monitorIsChecked && _monitorEthTabIsChecked
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        stable_coin_mcap_model.clear_qml();
    })
    refreshClickedCB: (function() {
        stable_coin_mcap_model.update_now = true;
    })
    visible: _stableCoinIsChecked && _stableCoinMcapTabIsChecked
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        stable_coin_chain_model.clear_qml();
    })
    refreshClickedCB: (function() {
        stable_coin_chain_model.refresh_qml();
    })
    visible: _stableCoinIsChecked && _stableCoinChainTabIsChecked
}

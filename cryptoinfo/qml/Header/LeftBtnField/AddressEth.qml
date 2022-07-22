import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        address_eth_model.clear_qml();
    })
    refreshClickedCB: (function() {
        address_eth_model.up_refresh_qml();
    })
    visible: _addressIsCheched && _addressEthTabIsChecked
}

import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        exchange_btc_model.clear_qml();
    })
    refreshClickedCB: (function() {
        exchange_btc_model.update_now = true;
    })
    visible: _exchangeIsCheched && _exchangeBtcTabIsChecked
}

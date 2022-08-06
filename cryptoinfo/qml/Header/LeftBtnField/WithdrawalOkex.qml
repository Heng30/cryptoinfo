import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        okex_withdrawal_rest_model.clear_qml();
    })
    refreshClickedCB: (function() {
        okex_withdrawal_rest_model.refresh_qml();
    })
    visible: _accountIsChecked && _accountWithdrawalTabIsChecked
}

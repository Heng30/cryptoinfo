import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    refreshClickedCB: (function() {
        chart_chain_tvl_model.update_now = true;
    })
    visible: _chartIsChecked && _chartChainTvlTabIsChecked
}

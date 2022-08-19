import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    refreshClickedCB: (function() {
        chart_chain_tvl_model.refresh_qml();
    })
    visible: _chartIsChecked && _chartChainTvlTabIsChecked
}

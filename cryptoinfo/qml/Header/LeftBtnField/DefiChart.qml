import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    refreshClickedCB: (function() {
        if (_defiChartChainTvlTabIsChecked)
            defi_chain_tvl_model.update_now = true;

    })
    visible: _defiChartIsChecked
}

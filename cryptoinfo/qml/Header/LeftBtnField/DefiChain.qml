import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        defi_chain_model.clear();
    })
    refreshClickedCB: (function() {
        root.defiChainRefresh();
    })
    search: (function(text) {
        defi_chain_model.search_and_view_at_beginning_qml(text);
    })
    visible: _defiChainIsChecked
}

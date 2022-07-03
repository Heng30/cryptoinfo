import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        defi_protocol_model.clear();
    })
    refreshClickedCB: (function() {
        defi_protocol_model.update_now = true;
    })
    search: (function(text) {
        defi_protocol_model.search_and_view_at_beginning_qml(text);
    })
    visible: _defiProtocolIsChecked
}

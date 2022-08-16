import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        nft_genie_model.clear_qml();
    })
    refreshClickedCB: (function() {
        nft_genie_model.refresh_qml();
    })
    visible: _nftIsChecked && _nftGenieTabIsChecked
}

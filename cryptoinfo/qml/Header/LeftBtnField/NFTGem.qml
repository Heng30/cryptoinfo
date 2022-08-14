import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        nft_gem_model.clear_qml();
    })
    refreshClickedCB: (function() {
        nft_gem_model.refresh_qml();
    })
    visible: _nftIsChecked && _nftGemTabIsChecked
}

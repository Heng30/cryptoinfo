import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        news_model.clear_qml();
        news_model.reset_page_index_qml();
    })
    refreshClickedCB: (function() {
        news_model.reset_page_index_qml();
        news_model.refresh_qml();
    })
    visible: _intelIsChecked && _newsTabIsChecked
}

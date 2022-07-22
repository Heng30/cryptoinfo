import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        news_model.clear_qml();
        news_model.page_index = 1;
    })
    refreshClickedCB: (function() {
        news_model.page_index = 1;
        news_model.update_now = true;
    })
    visible: _newsIsChecked
}

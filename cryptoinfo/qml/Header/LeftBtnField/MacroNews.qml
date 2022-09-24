import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        macro_news_model.clear_qml();
        macro_news_model.reset_cursor_qml();
    })
    refreshClickedCB: (function() {
        macro_news_model.reset_cursor_qml();
        macro_news_model.refresh_qml();
    })
    visible: _intelIsChecked && _macroNewsTabIsChecked
}

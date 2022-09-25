import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    clearClickedCB: (function() {
        msgBox.add(translator.tr("是否删除所有通知信息！"), true, function() {
            notify_model.clear_qml();
            notify_model.save_qml();
        }, function() {
        });
    })
    visible: _notifyIsChecked
}

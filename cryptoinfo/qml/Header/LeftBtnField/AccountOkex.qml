import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    property bool _isOnline: okex_account.is_login

    refreshClickedCB: (function() {
        okex_account.refresh_qml();
    })
    visible: _accountIsCheched
    Component.onCompleted: addImageModelItem(onlineStatus)

    QtObject {
        id: onlineStatus

        property string source: _isOnline ? "qrc:/res/image/green-circle.png" : "qrc:/res/image/red-circle.png"
        property string tipText: _isOnline ? translator.tr("在线") : translator.tr("离线")
        property bool visible: true
        property bool enableColorOverlay: false
        property var clicked: null
    }

}

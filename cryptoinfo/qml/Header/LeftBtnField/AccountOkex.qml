import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    property bool _isOnline: false

    Component.onCompleted: {
        okex_account.is_login_changed.connect(function() {
            _isOnline = okex_account.is_login_qml() ? true : false;
        });
    }
    visible: _accountIsChecked && _accountChanTabIsChecked
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/link-break.png"
            property string tipText: translator.tr("断开连接")
            property bool visible: true
            property var clicked: (function() {
                okex_account.break_link_qml();
            })
        },
        QtObject {
            property string source: "qrc:/res/image/refresh.png"
            property string tipText: translator.tr("刷新")
            property bool visible: true
            property var clicked: function() {
                if (config.okex_api_key.length <= 0 || config.okex_passphrase.length <= 0 || config.okex_secret_key.length <= 0)
                    msgTip.add(translator.tr("登陆信息不完整，请到设置完善登陆信息!"), true);

                okex_account_channel_model.clear_qml();
                okex_position_channel_model.clear_qml();
                okex_greek_channel_model.clear_qml();
                okex_account.refresh_qml();
            }
        },
        QtObject {
            property string source: _isOnline ? "qrc:/res/image/green-circle.png" : "qrc:/res/image/red-circle.png"
            property string tipText: _isOnline ? translator.tr("在线") : translator.tr("离线")
            property bool visible: true
            property bool enableColorOverlay: false
            property var clicked: null
        }
    ]
}

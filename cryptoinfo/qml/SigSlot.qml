import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onUpdated() {
            okex_subscribe_status_model.set_item_qml();
        }

        target: okex_subscribe_status_model
    }

    Connections {
        function onUpdated() {
            okex_account_channel_model.set_item_qml();
        }

        target: okex_account_channel_model
    }

    Connections {
        function onUpdated() {
            okex_position_channel_model.set_item_qml();
        }

        target: okex_position_channel_model
    }

    Connections {
        function onUpdated() {
            okex_greek_channel_model.set_item_qml();
        }

        target: okex_greek_channel_model
    }

    Connections {
        function onUpdated() {
            debug_log.recv_qml();
        }

        target: debug_log
    }

    Component.onCompleted: {
        // okex_account_channel_model.test_qml();
    }
}

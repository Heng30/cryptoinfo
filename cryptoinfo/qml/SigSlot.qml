import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onText_changed() {
            chart_chain_tvl_model.update_all_qml();
        }

        target: chart_chain_tvl_model
    }

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

    Component.onCompleted: {
        // okex_account_channel_model.test_qml();
    }
}

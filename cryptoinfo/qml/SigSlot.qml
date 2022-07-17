import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onText_changed() {
            chart_chain_tvl_model.update_all_qml();
        }

        target: chart_chain_tvl_model
    }
}

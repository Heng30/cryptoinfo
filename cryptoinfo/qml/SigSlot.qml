import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onText_changed() {
            defi_chain_tvl_model.update_all_qml();
        }

        target: defi_chain_tvl_model
    }
}

/// 因为rust中没有signal-slot机制，所以使用qml的connections实现相同的效果
import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onFear_greed_text_changed() {
            pricer_addition.update_fear_greed();
        }

        target: pricer_addition
    }

    Connections {
        function onMarket_text_changed() {
            pricer_addition.update_market();
        }

        target: pricer_addition
    }

    Connections {
        function onText_changed() {
            price_model.update_all();
        }

        target: price_model
    }

    Connections {
        function onText_changed() {
            defi_protocol_model.update_all();
        }

        target: defi_protocol_model
    }

    Connections {
        function onText_changed() {
            defi_chain_model.update_all();
        }

        target: defi_chain_model
    }
}

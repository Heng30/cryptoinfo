/// 因为rust中没有signal-slot机制，所以使用qml的connections实现相同的效果
import QtQuick 2.15
import QtQml 2.15

Item {
    Connections {
        function onFear_greed_text_changed() {
            price_addition.update_fear_greed_qml();
        }

        target: price_addition
    }

    Connections {
        function onMarket_text_changed() {
            price_addition.update_market_qml();
        }

        target: price_addition
    }

    Connections {
        function onEth_gas_text_changed() {
            price_addition.update_eth_gas_qml();
        }

        target: price_addition
    }

    Connections {
        function onBtc_stats_text_changed() {
            price_addition.update_btc_stats_qml();
        }

        target: price_addition
    }

    Connections {
        function onAhr999_text_changed() {
            price_addition.update_ahr999_qml();
        }

        target: price_addition
    }

    Connections {
        function onLong_short_text_changed() {
            price_addition.update_long_short_qml();
        }

        target: price_addition
    }

    Connections {
        function onOtc_text_changed() {
            price_addition.update_otc_qml();
        }

        target: price_addition
    }

    Connections {
        function onText_changed() {
            price_model.update_all_qml();
        }

        target: price_model
    }

    Connections {
        function onText_changed() {
            defi_protocol_model.update_all_qml();
        }

        target: defi_protocol_model
    }

    Connections {
        function onText_changed() {
            defi_chain_model.update_all_qml();
        }

        target: defi_chain_model
    }

    Connections {
        function onText_changed() {
            defi_chain_tvl_model.update_all_qml();
        }

        target: defi_chain_tvl_model
    }

    Connections {
        function onText_changed() {
            news_model.update_all_qml();
        }

        target: news_model
    }

    Connections {
        function onCtrl_alt_h_pressed() {
            main.visible = !main.visible;
        }

        target: ghotkey
    }

}

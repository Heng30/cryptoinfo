import QtQuick 2.15
import PanelType 1.0
import PriceSortKey 1.0
import DefiProtocolSortKey 1.0
import DefiChainSortKey 1.0

Item {
    id: shortKey

    property string popupPanelClose: "Esc"
    property string fontPixelSizeNormalInc: "Ctrl+="
    property string fontPixelSizeNormalDec: "Ctrl+-"
    property string clear: "Ctrl+C"
    property string search: "Ctrl+F"
    property string panelViewAtBeginning: "Ctrl+H"
    property string panelViewAtEnd: "Ctrl+L"
    property string popupPanelMax: "Ctrl+M"
    property string refresh: "Ctrl+R"
    property string save: "Ctrl+S"
    property string theme: "Ctrl+T"
    property string alt_1: "Alt+1"
    property string alt_2: "Alt+2"
    property string alt_3: "Alt+3"
    property string alt_4: "Alt+4"
    property string alt_5: "Alt+5"
    property string alt_6: "Alt+6"
    property string alt_7: "Alt+7"
    property string alt_8: "Alt+8"
    property string alt_9: "Alt+9"
    property string defiChart: "Alt+A"
    property string toolBox: "Alt+B"
    property string defiChain: "Alt+C"
    property string homepage: "Alt+H"
    property string note: "Alt+N"
    property string defiProtocol: "Alt+P"
    property string setting: "Alt+S"
    property string todo: "Alt+T"

    signal saved()

    Shortcut {
        sequence: StandardKey.Quit
        context: Qt.ApplicationShortcut
        onActivated: Qt.quit()
    }

    Shortcut {
        sequence: fontPixelSizeNormalInc
        context: Qt.ApplicationShortcut
        onActivated: config.font_pixel_size_normal += 1
    }

    Shortcut {
        sequence: fontPixelSizeNormalDec
        context: Qt.ApplicationShortcut
        onActivated: config.font_pixel_size_normal -= 1
    }

    Shortcut {
        sequence: clear
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.clear();
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.clear();
            } else if (config.panel_type === PanelType.DefiChain) {
                defi_chain_model.clear();
            }
        }
    }


    Shortcut {
        sequence: alt_1
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Marked);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Index);
            } else if (config.panel_type === PanelType.DefiChain) {
                defi_chain_model.toggle_sort_dir();
                defi_chain_model.sort_by_key(DefiChainSortKey.Index);
            }
        }
    }

    Shortcut {
        sequence: alt_2
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Index);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Name);
            } else if (config.panel_type === PanelType.DefiChain) {
                defi_chain_model.toggle_sort_dir();
                defi_chain_model.sort_by_key(DefiChainSortKey.Name);
            }
        }
    }

    Shortcut {
        sequence: alt_3
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Symbol);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Symbol);
            } else if (config.panel_type === PanelType.DefiChain) {
                defi_chain_model.toggle_sort_dir();
                defi_chain_model.sort_by_key(DefiChainSortKey.Symbol);
            }
        }
    }

    Shortcut {
        sequence: alt_4
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Price);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.TVL);
            } else if (config.panel_type === PanelType.DefiChain) {
                defi_chain_model.toggle_sort_dir();
                defi_chain_model.sort_by_key(DefiChainSortKey.TVL);
            }
        }
    }

    Shortcut {
        sequence: alt_5
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Per24H);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Staking);
            }
        }
    }

    Shortcut {
        sequence: alt_6
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Per7D);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.MarketCap);
            }
        }
    }

    Shortcut {
        sequence: alt_7
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Volume24H);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Per24H);
            }
        }
    }

    Shortcut {
        sequence: alt_8
        context: Qt.ApplicationShortcut
        onActivated: {
            if (config.panel_type === PanelType.Price) {
                price_model.toggle_sort_dir();
                price_model.sort_by_key(PriceSortKey.Floor);
            } else if (config.panel_type === PanelType.DefiProtocol) {
                defi_protocol_model.toggle_sort_dir();
                defi_protocol_model.sort_by_key(DefiProtocolSortKey.Per7D);
            }
        }
    }

    Shortcut {
        sequence: alt_9
        context: Qt.ApplicationShortcut
        onActivated: {
        }
    }

    Shortcut {
        sequence: save
        context: Qt.ApplicationShortcut
        onActivated: {
            shortKey.saved();
            if (config.panel_type === PanelType.Note)
                window.noteSaved();

        }
    }

    Shortcut {
        sequence: setting
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.Setting
    }

    Shortcut {
        sequence: note
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.Note
    }

    Shortcut {
        sequence: homepage
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.Price
    }

    Shortcut {
        sequence: todo
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.Todo
    }

    Shortcut {
        sequence: toolBox
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.ToolBox
    }

    Shortcut {
        sequence: defiProtocol
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.DefiProtocol
    }

    Shortcut {
        sequence: defiChain
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.DefiChain
    }

    Shortcut {
        sequence: defiChart
        context: Qt.ApplicationShortcut
        onActivated: config.panel_type = PanelType.DefiChart
    }

    Shortcut {
        sequence: theme
        context: Qt.ApplicationShortcut
        onActivated: {
            config.is_dark_theme = !config.is_dark_theme;
            config.save();
        }
    }

}

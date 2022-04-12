import QtQuick 2.15
import PriceSortKey 1.0

Item {
    property string popupPanelClose: "Esc"
    property string popupPanelMax: "Ctrl+M"
    property string pricePanelViewAtBeginning: "Ctrl+H"
    property string pricePanelViewAtEnd: "Ctrl+L"
    property string pricePanelRefresh: "Ctrl+R"
    property string fontPixelSizeNormalInc: "Ctrl+="
    property string fontPixelSizeNormalDec: "Ctrl+-"
    property string sortMarked: "Alt+1"
    property string sortMarketCap: "Alt+2"
    property string sortSymbol: "Alt+3"
    property string sortPrice: "Alt+4"
    property string sort24hPrecent: "Alt+5"
    property string sort7dPrecent: "Alt+6"
    property string sort24hVolume: "Alt+7"
    property string sortFloorPrice: "Alt+8"
    property string search: "Ctrl+F"

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
        sequence: sortMarked
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Marked);
        }
    }

    Shortcut {
        sequence: sortMarketCap
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.MarketCap);
        }
    }

    Shortcut {
        sequence: sortSymbol
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Symbol);
        }
    }

    Shortcut {
        sequence: sortPrice
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Price);
        }
    }

    Shortcut {
        sequence: sort24hPrecent
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Per24H);
        }
    }

    Shortcut {
        sequence: sort7dPrecent
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Per7D);
        }
    }

    Shortcut {
        sequence: sort24hVolume
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Volume24H);
        }
    }

    Shortcut {
        sequence: sortFloorPrice
        context: Qt.ApplicationShortcut
        onActivated: {
            pricer_model.toggle_sort_dir();
            pricer_model.sort_by_key(PriceSortKey.Floor);
        }
    }
}

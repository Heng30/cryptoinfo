import QtQml 2.15
import QtQuick.Window 2.15

QtObject {
    property bool darkTheme: config.is_dark_theme
    property color bgColor: darkTheme ? "black" : "white"
    property color invertBgColor: darkTheme ? "white" : "black"
    property color itemBgColor: darkTheme ? "#444444" : "lightgray"
    property color scrollBarColor: itemBgColor
    property color borderColor: darkTheme ? "white" : "black"
    property color fontColor: darkTheme ? "white" : "black"
    property color imageColor: darkTheme ? "white" : "black"
    property color lineSeriesColor: darkTheme ? Qt.rgba(1, 1, 1, 1) : Qt.rgba(0.0001, 0, 0, 1)
    property color imageEnteredColor: darkTheme ? "white" : "black"
    property color windowBorderEnterColor: darkTheme ? "white" : "black"
    property color headerBG: darkTheme ? Qt.darker("lightgray") : "steelblue"
    property color headImageColor: darkTheme ? "white" : "black"
    property color itemEnterColor: darkTheme ? "#555555" : "lightgray"
    property color itemEnteredBG: darkTheme ? "#444444" : "lightgray"
    property color itemEnxitedBG: darkTheme ? "lightgray" : "#444444"
    property color sepColor: darkTheme ? "lightgray" : "steelblue"
    property color priceHeaderBG: darkTheme ? "#555555" : "lightgray"
    property color priceUnmarkedColor: darkTheme ? "#555555" : "lightgray"
    property color priceMarkedColor: darkTheme ? Qt.lighter("red") : Qt.lighter("red")
    property color unmarkedColor: darkTheme ? "#555555" : "lightgray"
    property color markedColor: darkTheme ? Qt.lighter("red") : Qt.lighter("red")
    property color priceUpFontColor: darkTheme ? Qt.lighter("green") : "green"
    property color priceDownFontColor: darkTheme ? Qt.lighter("red") : "red"
    property color floorPriceBGColor: darkTheme ? Qt.lighter("red") : Qt.lighter("red")
    property color todoItemBGColor: darkTheme ? Qt.lighter("red") : Qt.lighter("red")
    property color settingFieldHeaderColor: darkTheme ? "#555555" : "lightgray"
    property color searchBarColor: darkTheme ? "#444444" : "lightgray"
    property color switchButtonColor: darkTheme ? "#bbbbbb" : "#444444"
    property color splashBarColor: theme.darkTheme ? Qt.darker("steelblue") : "steelblue"
    property real windowOpacity: config.window_opacity
    property real exitedOpacity: windowOpacity
    property real enteredOpacity: windowOpacity
    property real liveCircleOpacity: 0.1
    property int itemSpacing: 4
    property int itemPadding: 4
    property int itemMargins: 4
    property int itemRadius: 4
    property int fontPixelNormal: config.font_pixel_size_normal
    property real popupPanelWidth: 700 + (fontPixelNormal - 16) * fontPixelNormal
    property real popupPanelHeight: 600 + (fontPixelNormal - 16) * fontPixelNormal
    property real popupPanelMaxHeight: Screen.desktopAvailableHeight - 140
    property real popupPanelHeaderHeight: 32
    property int windowWidth: 50
    property int startupX: Screen.desktopAvailableWidth / 2 - windowWidth / 2
    property int startupY: Screen.desktopAvailableHeight / 2 - windowWidth / 2
    property int splashWitdh: Screen.desktopAvailableWidth / 3
    property int splashHeight: Screen.desktopAvailableHeight / 3
    property int splashStartupX: Screen.desktopAvailableWidth / 2 - splashWitdh / 2
    property int splashStartupY: Screen.desktopAvailableHeight / 2 - splashHeight / 2
}

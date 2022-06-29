import QtQml 2.15
import QtQuick.Window 2.15

QtObject {
    property bool darkTheme: config.is_dark_theme
    property color bgColor: darkTheme ? "black" : "white"
    property color invertBgColor: darkTheme ? "white" : "black"
    property color itemBgColor: darkTheme ? "#444444" : "lightgray"
    property color inputBarBgColor: darkTheme ? "#333333" : "#eeeeee"
    property color scrollBarColor: itemBgColor
    property color borderColor: darkTheme ? "white" : "black"
    property color fontColor: darkTheme ? "white" : "black"
    property color underFontColor: theme.darkTheme ? Qt.darker("lightgray") : "lightgray"
    property color imageColor: darkTheme ? "white" : "black"
    property color lineSeriesColor: darkTheme ? Qt.rgba(1, 1, 1, 1) : Qt.rgba(0.0001, 0, 0, 1)
    property color imageEnteredColor: darkTheme ? "white" : "black"
    property color windowBorderEnterColor: darkTheme ? "white" : "black"
    property color headerBG: darkTheme ? Qt.darker("lightgray") : "steelblue"
    property color headImageColor: darkTheme ? "white" : "black"
    property color itemEnterColor: darkTheme ? "#555555" : "lightgray"
    property color itemEnteredBG: darkTheme ? "#444444" : "lightgray"
    property color itemEnxitedBG: darkTheme ? "lightgray" : "#444444"
    property color itemCheckedBG: darkTheme ? Qt.lighter("#444444") :  Qt.darker("lightgray")
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
    property int itemSpacing: 4
    property int itemPadding: 4
    property int itemMargins: 4
    property int itemRadius: 4
    property int fontPixelNormal: config.font_pixel_size_normal
    property real panelWidth: config.window_width
    property real panelHeight: 600
    property real panelMaxHeight: config.window_height
    property real panelHeaderHeight: 32
    property int splashWitdh: Screen.desktopAvailableWidth / 3
    property int splashHeight: Screen.desktopAvailableHeight / 3

    signal themeSig()

    onDarkThemeChanged: themeSig()
}

import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: window

    property int defiChartCheckedTabIndex: 0
    property int exchangeCheckedTabIndex: 0

    width: homePage.width
    height: homePage.height

    ShortKey {
        id: shortKey
    }

    SigSlot {
    }

    HomePage {
        id: homePage
    }

}

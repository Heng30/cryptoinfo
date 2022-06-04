import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: window

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

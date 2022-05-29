import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: window


    signal noteSaved()

    width: homePage.width
    height: homePage.height

    ShortKey {
        id: shortKey
    }

    UtilityFn {
        id: utilityFn
    }

    SigSlot {
    }

    HomePage {
        id: homePage
    }

}

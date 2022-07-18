import QtQuick 2.15
import QtQuick.Window 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: window

    property int chartCheckedTabIndex: 0
    property bool _chartChainTvlTabIsChecked: chartCheckedTabIndex === 0

    property int exchangeCheckedTabIndex: 0
    property bool _exchangeBtcTabIsChecked: exchangeCheckedTabIndex === 0

    property int monitorCheckedTabIndex: 0
    property bool _monitorBtcTabIsChecked: monitorCheckedTabIndex === 0

    property int stableCoinCheckedTabIndex: 0
    property bool _stableCoinMcapTabIsChecked: stableCoinCheckedTabIndex === 0
    property bool _stableCoinChainTabIsChecked: stableCoinCheckedTabIndex === 1

    property int chainCheckedTabIndex: 0
    property bool _chainProtocolTabIsChecked: chainCheckedTabIndex === 0
    property bool _chainTvlTabIsChecked: chainCheckedTabIndex === 1


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

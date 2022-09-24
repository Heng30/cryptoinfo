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
    property bool _monitorEthTabIsChecked: monitorCheckedTabIndex === 1
    property int stableCoinCheckedTabIndex: 0
    property bool _stableCoinMcapTabIsChecked: stableCoinCheckedTabIndex === 0
    property bool _stableCoinChainTabIsChecked: stableCoinCheckedTabIndex === 1
    property int chainCheckedTabIndex: 0
    property bool _chainProtocolTabIsChecked: chainCheckedTabIndex === 0
    property bool _chainYieldTabIsChecked: chainCheckedTabIndex === 1
    property bool _chainTvlTabIsChecked: chainCheckedTabIndex === 2
    property bool _chainEthTokenTabIsChecked: chainCheckedTabIndex === 3
    property int addressCheckedTabIndex: 0
    property bool _addressBtcTabIsChecked: addressCheckedTabIndex === 0
    property bool _addressEthTabIsChecked: addressCheckedTabIndex === 1

    property int accountCheckedTabIndex: 0
    property bool _accountChanTabIsChecked: accountCheckedTabIndex === 0 || accountCheckedTabIndex === 1 || accountCheckedTabIndex === 2 || accountCheckedTabIndex === 7
    property bool _accountMainRestTabIsChecked: accountCheckedTabIndex === 3
    property bool _accountBillTabIsChecked: accountCheckedTabIndex === 4
    property bool _accountDepositTabIsChecked: accountCheckedTabIndex === 5
    property bool _accountWithdrawalTabIsChecked: accountCheckedTabIndex === 6

    property int nftCheckedTabIndex: 0
    property bool _nftGemTabIsChecked: nftCheckedTabIndex === 0
    property bool _nftSudoSwapTabIsChecked: nftCheckedTabIndex === 1
    property bool _nftGenieTabIsChecked: nftCheckedTabIndex === 2

    property int intelCheckedTabIndex: 0
    property bool _newsTabIsChecked: intelCheckedTabIndex === 0
    property bool _macroNewsTabIsChecked: intelCheckedTabIndex === 1
    property bool _macroEventTabIsChecked: intelCheckedTabIndex === 2


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

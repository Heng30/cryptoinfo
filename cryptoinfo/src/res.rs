use qmetaobject::prelude::*;

qrc!(pub resource_init,
    "res" {
        "qml/main.qml",
        "qml/Splash.qml",
        "qml/MainWindow.qml",
        "qml/HomePage.qml",
        "qml/Footer.qml",
        "qml/About.qml",
        "qml/Login.qml",

        "qml/Header/Field.qml",
        "qml/Header/Left.qml",
        "qml/Header/Right.qml",

        "qml/Header/LeftBtnField/BtnField.qml",
        "qml/Header/LeftBtnField/Price.qml",
        "qml/Header/LeftBtnField/ChainProtocol.qml",
        "qml/Header/LeftBtnField/ChainTvl.qml",
        "qml/Header/LeftBtnField/ChartChainTvl.qml",
        "qml/Header/LeftBtnField/ChainCryptoFee.qml",
        "qml/Header/LeftBtnField/StableCoinMcap.qml",
        "qml/Header/LeftBtnField/StableCoinChain.qml",
        "qml/Header/LeftBtnField/ChainYield.qml",
        "qml/Header/LeftBtnField/AccountOkex.qml",
        "qml/Header/LeftBtnField/MainAccountOkex.qml",
        "qml/Header/LeftBtnField/DepositOkex.qml",
        "qml/Header/LeftBtnField/WithdrawalOkex.qml",
        "qml/Header/LeftBtnField/BillOkex.qml",
        "qml/Header/LeftBtnField/DebugLog.qml",
        "qml/Header/LeftBtnField/MacroEvent.qml",
        "qml/Header/LeftBtnField/MacroNews.qml",
        "qml/Header/LeftBtnField/Notify.qml",

        "qml/Price/Panel.qml",
        "qml/Price/Header.qml",
        "qml/Price/DItem.qml",
        "qml/Price/PItem.qml",
        "qml/Price/Detail.qml",
        "qml/Price/MacroChara.qml",

        "qml/Chain/Panel.qml",
        "qml/Chain/Protocol/Panel.qml",
        "qml/Chain/Protocol/Header.qml",
        "qml/Chain/Protocol/DItem.qml",
        "qml/Chain/Protocol/PItem.qml",
        "qml/Chain/Tvl/Panel.qml",
        "qml/Chain/Tvl/Header.qml",
        "qml/Chain/Tvl/DItem.qml",
        "qml/Chain/Tvl/PItem.qml",
        "qml/Chain/Yield/Panel.qml",
        "qml/Chain/Yield/Header.qml",
        "qml/Chain/Yield/DItem.qml",
        "qml/Chain/Yield/PItem.qml",
        "qml/Chain/CryptoFee/Panel.qml",

        "qml/Chart/Panel.qml",
        "qml/Chart/ChainTVL.qml",
        "qml/Chart/FloatTip.qml",

        "qml/Intel/Panel.qml",
        "qml/Intel/MacroEvent/Panel.qml",
        "qml/Intel/MacroNews/Panel.qml",
        "qml/Intel/MacroNews/DItem.qml",

        "qml/Account/Panel.qml",
        "qml/Account/OkexSubscribeStatus.qml",
        "qml/Account/OkexAccountChannel.qml",
        "qml/Account/OkexGreekChannel.qml",
        "qml/Account/OkexPositionChannel.qml",
        "qml/Account/OkexMainAccountRest.qml",
        "qml/Account/OkexDepositRest.qml",
        "qml/Account/OkexWithdrawalRest.qml",
        "qml/Account/OkexBillRest.qml",

        "qml/Notify/Panel.qml",
        "qml/Notify/DItem.qml",

        "qml/StableCoin/Panel.qml",
        "qml/StableCoin/Mcap/DItem.qml",
        "qml/StableCoin/Mcap/PItem.qml",
        "qml/StableCoin/Mcap/Header.qml",
        "qml/StableCoin/Mcap/Panel.qml",
        "qml/StableCoin/Chain/DItem.qml",
        "qml/StableCoin/Chain/PItem.qml",
        "qml/StableCoin/Chain/Header.qml",
        "qml/StableCoin/Chain/Panel.qml",

        "qml/Setting/Panel.qml",
        "qml/Setting/Account.qml",
        "qml/Setting/Data.qml",
        "qml/Setting/Lang.qml",
        "qml/Setting/ShortKey.qml",
        "qml/Setting/UI.qml",
        "qml/Setting/BackupRecover.qml",
        "qml/Setting/WindowMode.qml",
        "qml/Setting/Login/Panel.qml",
        "qml/Setting/Login/SetPS.qml",
        "qml/Setting/Login/DelPS.qml",
        "qml/Setting/Other.qml",
        "qml/Setting/Test.qml",

        "qml/ToolBox/Panel.qml",
        "qml/ToolBox/Encipher/Panel.qml",

        "qml/ToolBox/Cal/Panel.qml",
        "qml/ToolBox/Cal/ILOneStableCoin.qml",
        "qml/ToolBox/Cal/KellyFormula.qml",
        "qml/ToolBox/Cal/CompoundInterest.qml",

        "qml/ToolBox/AddrBook/Panel.qml",
        "qml/ToolBox/AddrBook/DItem.qml",
        "qml/ToolBox/AddrBook/Qrcode.qml",
        "qml/ToolBox/AddrBook/AddItem.qml",

        "qml/ToolBox/HandBook/Panel.qml",
        "qml/ToolBox/HandBook/DItem.qml",
        "qml/ToolBox/HandBook/SItem.qml",
        "qml/ToolBox/HandBook/SDItem.qml",
        "qml/ToolBox/HandBook/Footer.qml",
        "qml/ToolBox/HandBook/Chart.qml",
        "qml/ToolBox/HandBook/PaymentChart.qml",
        "qml/ToolBox/HandBook/IncomeChart.qml",

        "qml/ToolBox/FundBook/Panel.qml",
        "qml/ToolBox/FundBook/DItem.qml",
        "qml/ToolBox/FundBook/Footer.qml",

        "qml/ToolBox/BookMark/Panel.qml",
        "qml/ToolBox/BookMark/Left.qml",
        "qml/ToolBox/BookMark/LeftDItem.qml",
        "qml/ToolBox/BookMark/Right.qml",
        "qml/ToolBox/BookMark/RightDItem.qml",
        "qml/ToolBox/BookMark/LeftHeaderBar.qml",
        "qml/ToolBox/BookMark/RightHeaderBar.qml",

        "qml/ToolBox/Other/Panel.qml",
        "qml/ToolBox/Other/ExchangeRate.qml",

        "qml/ToolBox/Todo/Panel.qml",
        "qml/ToolBox/Todo/DItem.qml",

        "qml/ToolBox/Note/Panel.qml",
        "qml/ToolBox/Note/Left.qml",
        "qml/ToolBox/Note/LeftDItem.qml",
        "qml/ToolBox/Note/LeftHeaderBar.qml",
        "qml/ToolBox/Note/Right.qml",

        "qml/ToolBox/ContractStats/Panel.qml",
        "qml/ToolBox/ContractStats/Chart.qml",
        "qml/ToolBox/ContractStats/DItem.qml",

        "qml/ToolBox/DebugLog/Panel.qml",

        "qml/ShortKey.qml",
        "qml/UtilityFn.qml",
        "qml/SigSlot.qml",

        "qml/Base/Theme.qml",
        "qml/Base/ItemText.qml",
        "qml/Base/ItemLabel.qml",
        "qml/Base/SettingField.qml",
        "qml/Base/NumInput.qml",
        "qml/Base/ImageButton.qml",
        "qml/Base/LiveCircle.qml",
        "qml/Base/RadioButton.qml",
        "qml/Base/CheckBox.qml",
        "qml/Base/ComBox.qml",
        "qml/Base/SearchBar.qml",
        "qml/Base/Switch.qml",
        "qml/Base/Tip.qml",
        "qml/Base/SelectBox.qml",
        "qml/Base/TxtField.qml",
        "qml/Base/SlideBar.qml",
        "qml/Base/SBar.qml",
        "qml/Base/ProgBar.qml",
        "qml/Base/BDial.qml",
        "qml/Base/TxtButton.qml",
        "qml/Base/TxtArea.qml",
        "qml/Base/MsgBox.qml",
        "qml/Base/MsgTip.qml",
        "qml/Base/BTab.qml",
        "qml/Base/InputBar.qml",
        "qml/Base/Sep.qml",
        "qml/Base/TimeLineSeriesChart.qml",
        "qml/Base/DashLine.qml",
        "qml/Base/FloatLabel.qml",
        "qml/Base/LabelTxtField.qml",
        "qml/Base/DragArea.qml",
        "qml/Base/CDialog.qml",
        "qml/Base/CPieChart.qml",
        "qml/Base/NewsItem.qml",
        "qml/Base/DebugLog.qml",
        "qml/Base/SPage.qml",
        "qml/Base/Carousel.qml",

        "qml/Base/ItemPanel/DItem.qml",
        "qml/Base/ItemPanel/PItem.qml",
        "qml/Base/ItemPanel/Header.qml",
        "qml/Base/ItemPanel/Panel.qml",

        "image/icon.png",
        "image/splash.png",
        "image/exit.png",
        "image/setting.png",
        "image/refresh.png",
        "image/theme.png",
        "image/max-height.png",
        "image/minus.png",
        "image/add.png",
        "image/add2.png",
        "image/open.png",
        "image/search.png",
        "image/clear.png",
        "image/home.png",
        "image/note.png",
        "image/bell.png",
        "image/warn.png",
        "image/info.png",
        "image/todo-list.png",
        "image/tool-box.png",
        "image/blockchain.png",
        "image/chain.png",
        "image/chart.png",
        "image/qrcode.png",
        "image/edit.png",
        "image/copy.png",
        "image/up.png",
        "image/down.png",
        "image/save.png",
        "image/up-join.png",
        "image/about.png",
        "image/eye-hiden.png",
        "image/browser.png",
        "image/logout.png",
        "image/news.png",
        "image/exchange.png",
        "image/monitor.png",
        "image/stablecoin.png",
        "image/pool.png",
        "image/wallet.png",
        "image/account.png",
        "image/address.png",
        "image/red-circle.png",
        "image/green-circle.png",
        "image/link-break.png",
        "image/debug.png",
        "image/recovery.png",
        "image/preview.png",
        "image/rename.png",
        "image/like.png",
        "image/unlike.png",
        "image/notify.png",
        "image/notify-red.png",

        "sound/login.wav",
    }
);

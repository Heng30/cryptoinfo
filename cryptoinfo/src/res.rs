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
        "qml/Header/LeftBtnField/DefiProtocol.qml",
        "qml/Header/LeftBtnField/DefiChain.qml",
        "qml/Header/LeftBtnField/DefiChart.qml",
        "qml/Header/LeftBtnField/News.qml",

        "qml/Price/Panel.qml",
        "qml/Price/Header.qml",
        "qml/Price/DItem.qml",
        "qml/Price/PItem.qml",
        "qml/Price/Detail.qml",

        "qml/Defi/Protocol/Panel.qml",
        "qml/Defi/Protocol/Header.qml",
        "qml/Defi/Protocol/DItem.qml",
        "qml/Defi/Protocol/PItem.qml",

        "qml/Defi/Chain/Panel.qml",
        "qml/Defi/Chain/Header.qml",
        "qml/Defi/Chain/DItem.qml",
        "qml/Defi/Chain/PItem.qml",

        "qml/Defi/Chart/Panel.qml",
        "qml/Defi/Chart/ChainTVL.qml",
        "qml/Defi/Chart/FloatTip.qml",

        "qml/News/Panel.qml",
        "qml/News/DItem.qml",

        "qml/Setting/Panel.qml",
        "qml/Setting/Data.qml",
        "qml/Setting/Lang.qml",
        "qml/Setting/ShortKey.qml",
        "qml/Setting/UI.qml",
        "qml/Setting/BackupRecover.qml",
        "qml/Setting/WindowMode.qml",
        "qml/Setting/Login/Panel.qml",
        "qml/Setting/Login/SetPS.qml",
        "qml/Setting/Login/DelPS.qml",
        "qml/Setting/WebServer.qml",
        "qml/Setting/Other.qml",
        "qml/Setting/Test.qml",

        "qml/ToolBox/Panel.qml",
        "qml/ToolBox/Encipher/Panel.qml",
        "qml/ToolBox/IL/Panel.qml",
        "qml/ToolBox/IL/OneStableCoin.qml",

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
        "qml/ToolBox/Other/QueryPublicIP.qml",

        "qml/ToolBox/Todo/Panel.qml",
        "qml/ToolBox/Todo/DItem.qml",

        "qml/ToolBox/Note/Panel.qml",

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

        "sound/login.wav",
    }
);

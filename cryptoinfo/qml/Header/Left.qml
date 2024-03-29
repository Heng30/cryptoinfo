import QtQuick 2.15
import QtQuick.Controls 2.15
import PanelType 1.0
import "qrc:/res/qml/Base" as Base

Row {
    id: left

    property list<QtObject> imageModel

    height: parent.height
    spacing: theme.itemSpacing
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/home.png"
            property string tipText: translator.tr("主页")
            property bool visible: true
            property bool checked: _homeIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Price;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/chain.png"
            property string tipText: translator.tr("公链")
            property bool visible: true
            property bool checked: _chainIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Chain;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/stablecoin.png"
            property string tipText: translator.tr("稳定币")
            property bool visible: true
            property bool checked: _stableCoinIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.StableCoin;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/chart.png"
            property string tipText: translator.tr("图表")
            property bool visible: true
            property bool checked: _chartIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Chart;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/news.png"
            property string tipText: translator.tr("情报")
            property bool visible: true
            property bool checked: _intelIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Intel;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/tool-box.png"
            property string tipText: translator.tr("工具箱")
            property bool visible: true
            property bool checked: _toolBoxIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.ToolBox;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/account.png"
            property string tipText: translator.tr("个人账户")
            property bool visible: true
            property bool checked: _accountIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.Account;
            }
        }
    ]

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }

}

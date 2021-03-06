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
            property string source: "qrc:/res/image/blockchain.png"
            property string tipText: translator.tr("协议")
            property bool visible: true
            property bool checked: _defiProtocolIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.DefiProtocol;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/chain.png"
            property string tipText: translator.tr("公链")
            property bool visible: true
            property bool checked: _defiChainIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.DefiChain;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/chart.png"
            property string tipText: translator.tr("图表")
            property bool visible: true
            property bool checked: _defiChartIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.DefiChart;
            }
        },
        QtObject {
            property string source: "qrc:/res/image/news.png"
            property string tipText: translator.tr("资讯")
            property bool visible: true
            property bool checked: _newsIsChecked
            property var clicked: function() {
                config.panel_type = PanelType.News;
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
        }
    ]

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }

}

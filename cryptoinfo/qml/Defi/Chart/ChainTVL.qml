import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    anchors.fill: parent

    Base.TimeLineSeries {
        id: chainTVL

        function _update(count) {
            if (count <= 0)
                return ;

            var flag = utilityFn.isAsBillion(defi_chain_tvl_model.max_tvl);

            chainTVL.series.clear();
            chainTVL.title = defi_chain_tvl_model.name + translator.tr("锁仓量");
            var firstItem = defi_chain_tvl_model.item(0);
            var endItem = defi_chain_tvl_model.item(count - 1);
            chainTVL.xMin = new Date(utilityFn.seconds2milliseconds(firstItem.second));
            chainTVL.xMax = new Date(utilityFn.seconds2milliseconds(endItem.second));
            for (var i = 0; i < count; i++) {
                var item = defi_chain_tvl_model.item(i);
                var x = utilityFn.seconds2milliseconds(item.second);
                var y = flag ? utilityFn.asBillion(item.tvl, 9) : utilityFn.asMillion(item.tvl, 9);
                series.append(new Date(x), y);
            }
        }

        anchors.fill: parent
        xTitleText: translator.tr("时间")
        yTitleText: translator.tr("美元") + "(" + (utilityFn.isAsBillion(defi_chain_tvl_model.max_tvl) ? translator.tr("十亿") : translator.tr("百万")) + ")"
        yMax: utilityFn.asMillionOrBillion(defi_chain_tvl_model.max_tvl, 2) + 0.1

        Connections {
            function onUpdated() {
                chainTVL._update(defi_chain_tvl_model.count);
            }

            target: defi_chain_tvl_model
        }

        Timer {
            interval: 3000
            repeat: false
            running: true
            triggeredOnStart: false
            onTriggered: defi_chain_tvl_model.qml_update_text("Ethereum")
        }

    }

    Base.ComBox {
        id: comBox

        property var _model: []
        property int _index: 0
        property string selectedName: model[_index]

        anchors.right: parent.right
        anchors.margins: theme.itemMargins * 4
        width: 100
        popupHeight: parent.height / 2
        onActivated: {
            _index = index;
            defi_chain_tvl_model.qml_update_text(selectedName);
        }
        Component.onCompleted: {
            for (var i = 0; i < defi_chain_name_model.count; i++) {
                _model.push(defi_chain_name_model.item(i).name);
                model = _model;
            }
        }
    }

}

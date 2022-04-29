import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: exchangeRate

    function _update() {
        const Http = new XMLHttpRequest();
        const url = "https://open.er-api.com/v6/latest/USD";
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function() {
            if (Http.readyState !== 4 || Http.status !== 200) return;
            const text = Http.responseText;
            if (text.length <= 0)
                return ;

            try {
                var data = JSON.parse(text);
                var unixSeconds = data.time_last_update_unix;
                content._rates = data.rates;
                if (!unixSeconds || !content._rates)
                    return ;

                content._updateTime = utility.utc_seconds_to_local_string(Number(unixSeconds), "%y-%m-%d %H:%M:%S");
                var fromModel = [];
                var toModel = [];
                Object.keys(content._rates).map(function(key) {
                    if (key === "CNY") {
                        fromModel.splice(1, 0, key)
                        toModel.unshift(key);
                    } else {
                        fromModel.push(key);
                        toModel.push(key);
                    }
                });
                fromRate.model = fromModel;
                toRate.model = toModel;
                exchangeRate._calc();
            } catch (e) {
                console.log(e);
            }
        };
    }

    function _calc() {
        if (!content._rates)
            return ;

        var fromKey = fromRate.model[fromRate.index];
        var toKey = toRate.model[toRate.index];
        var fromUSD = Number(fromRate.text) / content._rates[fromKey];
        toRate.text = (fromUSD * content._rates[toKey]).toFixed(2);
    }

    anchors.fill: parent
    Component.onCompleted: exchangeRate._update()

    Column {
        id: content

        property string _updateTime: ""
        property var _rates: null

        anchors.centerIn: parent
        spacing: theme.itemSpacing * 2

        Base.ItemLabel {
            id: label
            anchors.horizontalCenter: parent.horizontalCenter
            text: translator.tr("更新时间") + ": " + content._updateTime
            tipText: translator.tr("点击刷新")
            onClicked: exchangeRate._update()
        }

        Row {
            spacing: theme.itemSpacing * 5
            anchors.horizontalCenter: parent.horizontalCenter

            Base.SelectBox {
                id: fromRate

                property int index: 0

                txtFieldWidth: theme.fontPixelNormal * 8 + itemSpacing
                boxWidth: theme.fontPixelNormal * 3 + theme.itemSpacing
                text: String(1)
                onTextAccepted: exchangeRate._calc()
                onBoxActived: {
                    fromRate.index = index;
                    exchangeRate._calc();
                }
            }

            Base.SelectBox {
                id: toRate

                property int index: 0

                txtFieldWidth: theme.fontPixelNormal * 8 + itemSpacing
                boxWidth: theme.fontPixelNormal * 3 + theme.itemSpacing
                readOnly: true
                onTextAccepted: exchangeRate._calc()
                onBoxActived: {
                    toRate.index = index;
                    exchangeRate._calc();
                }
            }

        }

    }

}

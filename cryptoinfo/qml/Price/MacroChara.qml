import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.Carousel {
    id: root

    function _update() {
        const Http = new XMLHttpRequest();
        const url = "https://api-ddc-wscn.awtmt.com/market/real?fields=prod_name%2Cpreclose_px%2Clast_px%2Cpx_change%2Cpx_change_rate%2Cprice_precision&prod_code=000001.SS%2CDXY.OTC%2CUS10YR.OTC%2CUSDCNH.OTC%2C399001.SZ%2C399006.SZ";
        Http.open("GET", url);
        Http.send();
        Http.onreadystatechange = function() {
            if (Http.readyState !== 4 || Http.status !== 200)
                return ;

            const text = Http.responseText;
            if (text.length <= 0)
                return ;

            try {
                var charaText = utilityFn.paddingSpace(8);
                var data = JSON.parse(text);
                var snapshot = data.data.snapshot;
                if (!snapshot)
                    return ;

                Object.values(snapshot).map(function(item) {
                    if (item.length !== 7)
                        return ;

                    charaText += item[0] + utilityFn.paddingSpace(2) + item[2] + "(" + utilityFn.toPercentString(Number(item[4])) + ")" + utilityFn.paddingSpace(8);
                });
                root.text = charaText;
            } catch (e) {
                console.log(e);
            }
        };
    }

    width: parent.width
    run: _homeIsChecked

    Timer {
        interval: 1000 * 10 * 60
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: _update()
    }

}

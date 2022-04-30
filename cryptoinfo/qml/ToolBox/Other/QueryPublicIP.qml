import QtQuick 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: queryIP

    width: parent.width
    headerText: translator.tr("查询公网IP")
    spacing: theme.itemSpacing

    contentItem: Column {
        id: content

        property string _physicalAddr: ""

        function _querySelfIP() {
            const Http = new XMLHttpRequest();
            const url = "https://ip-fast.com/api/ip/";
            Http.open("GET", url);
            Http.send();
            Http.onreadystatechange = function() {
                if (Http.readyState !== 4 || Http.status !== 200)
                    return ;

                const text = Http.responseText;
                if (text.length <= 0)
                    return ;

                ipLabel.text = text;
                content._queryPhysicalAddr();
            };
        }

        function _queryPhysicalAddr() {
            const ip = ipLabel.text;
            const Http = new XMLHttpRequest();
            const url = "http://whois.pconline.com.cn/ipJson.jsp?ip=" + ip + "&json=true";
            Http.open("GET", url);
            Http.send();
            Http.onreadystatechange = function() {
                if (Http.readyState !== 4 || Http.status !== 200)
                    return ;

                const text = Http.responseText;
                if (text.length <= 0)
                    return ;

                try {
                    var data = JSON.parse(text);
                    if (!data.addr)
                        return ;

                    content._physicalAddr = data.addr;
                } catch (e) {
                    console.log(e);
                }
            };
        }

        function _update() {
            var flag = false;
            if (ipLabel.text.length <= 0) {
                flag = true;
                content._querySelfIP();
            }
            if (!flag && ipLabel.text.length > 0)
                content._queryPhysicalAddr();

        }

        Component.onCompleted: content._update()

        Row {
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: theme.itemSpacing * 2

            Base.LabelTxtField {
                id: ipLabel

                anchors.verticalCenter: parent.verticalCenter
                txtFieldWidth: theme.fontPixelNormal * 10 + theme.itemSpacing
                labelText: translator.tr("公网IP") + ":"
                labelTipText: translator.tr("点击刷新")
                onLabelClicked: content._update()
                onTextAccepted: content._update()
            }

            Base.ItemLabel {
                anchors.verticalCenter: parent.verticalCenter
                text: translator.tr("归属地") + ": " + content._physicalAddr
            }

        }

    }

}

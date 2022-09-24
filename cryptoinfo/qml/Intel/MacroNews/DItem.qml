import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Item {
    id: dItem

    width: ListView.view.width
    height: newsItem.height

    Base.NewsItem {
        id: newsItem

        width: parent.width - theme.itemMargins * 2
        anchors.verticalCenter: parent.verticalCenter
        titleText: modelData.title
        contentText: modelData.content
        addTimeText: modelData.add_time
        urlText: modelData.url
        onOpenUrlClicked: utility.process_cmd_qml(config.browser, urlText)
        textColor: modelData.score > 1 ? theme.priceDownFontColor : theme.fontColor

    }

}

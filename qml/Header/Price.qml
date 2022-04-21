import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: price

    property bool _isSearchChecked: false
    property list<QtObject> imageModel

    height: parent.height
    spacing: theme.itemSpacing
    visible: root._homeIsChecked
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/clear.png"
            property string tipText: translator.tr("清除")
            property bool visible: true
            property var clicked: function() {
                pricer_model.clear();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/refresh.png"
            property string tipText: translator.tr("刷新")
            property bool visible: true
            property var clicked: function() {
                root.refresh();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/search.png"
            property string tipText: translator.tr("搜索")
            property bool visible: true
            property var clicked: function() {
                price._isSearchChecked = !price._isSearchChecked;
                if (price._isSearchChecked)
                    searchBar.forceFocus();

            }
        }
    ]

    Base.Sep {
        height: parent.height / 2
        anchors.verticalCenter: parent.verticalCenter
    }

    Repeater {
        model: parent.imageModel
        delegate: dItem
    }

    Base.SearchBar {
        id: searchBar

        anchors.verticalCenter: parent.verticalCenter
        width: 100
        height: parent.height / 4 * 3
        visible: price._isSearchChecked
        color: theme.searchBarColor
        Keys.onTabPressed: price._isSearchChecked = !price._isSearchChecked
        text: price._isSearchChecked ? text : ""
        onEditingFinished: {
            pricer_model.search_and_show_at_beginning(text);
            root.searchEditingFinished();
        }

        Shortcut {
            sequence: shortKey.search
            onActivated: {
                price._isSearchChecked = true;
                searchBar.forceFocus();
            }
        }

    }

}

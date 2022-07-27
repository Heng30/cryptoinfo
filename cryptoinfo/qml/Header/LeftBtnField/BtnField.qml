import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Row {
    id: btnField

    property var clearClickedCB: null
    property var refreshClickedCB: null
    property var search: null
    property bool _isSearchChecked: false
    property list<QtObject> imageModel

    function showSearchBar() {
        btnField._isSearchChecked = true;
        searchBar.forceFocus();
    }

    function addImageModelItem(item) {
        if (!item) return;
        imageModel.push(item);
    }

    height: parent.height
    spacing: theme.itemSpacing
    imageModel: [
        QtObject {
            property string source: "qrc:/res/image/clear.png"
            property string tipText: translator.tr("清除")
            property bool visible: btnField.clearClickedCB
            property var clicked: function() {
                btnField.clearClickedCB();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/refresh.png"
            property string tipText: translator.tr("刷新")
            property bool visible: btnField.refreshClickedCB
            property var clicked: function() {
                btnField.refreshClickedCB();
            }
        },
        QtObject {
            property string source: "qrc:/res/image/search.png"
            property string tipText: translator.tr("搜索")
            property bool visible: btnField.search
            property var clicked: function() {
                btnField._isSearchChecked = !btnField._isSearchChecked;
                if (btnField._isSearchChecked)
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
        visible: btnField._isSearchChecked
        color: theme.searchBarColor
        Keys.onTabPressed: btnField._isSearchChecked = !btnField._isSearchChecked
        text: btnField._isSearchChecked ? text : ""
        onEditingFinished: {
            if (btnField.search)
                btnField.search(text);

            root.searchEditingFinished();
        }
    }

}

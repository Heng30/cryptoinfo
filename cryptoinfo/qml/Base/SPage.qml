import QtQuick 2.15
import QtQuick.Controls 2.15
import "." as Base

Item {
    id: root

    required property int maxPages
    property real pageSize: 40
    property int _startPage: 1
    property int _checkedPage: 1
    property int _calPages: Math.floor((content.width - prevPage.width) / (root.pageSize + content.spacing))

    signal clicked(int index)

    function _goPage() {
        goPageBtn.forceActiveFocus();
        var p = Number(goPage.text);
        if (isNaN(p))
            return ;

        p = Math.min(Math.max(1, p), root.maxPages);
        root._startPage = p;
        root._checkedPage = root._startPage;
        root.clicked(root._checkedPage);
    }

    implicitWidth: 300
    height: content.height

    Row {
        id: content

        width: parent.width - rightRow.width - sep.width
        spacing: theme.itemSpacing

        Base.TxtButton {
            id: prevPage

            enabled: root._startPage > 1
            height: root.pageSize
            radius: 0
            text: translator.tr("上一页")
            onClicked: root._startPage = Math.max(root._startPage - root._calPages, 1)
        }

        Repeater {
            model: Math.min(Math.max(root._calPages, 0), root.maxPages - root._startPage + 1)

            Base.TxtButton {
                width: root.pageSize
                height: width
                radius: 0
                text: root._startPage + index
                checked: root._checkedPage === root._startPage + index
                onClicked: {
                    root._checkedPage = root._startPage + index;
                    root.clicked(root._checkedPage);
                }
            }

        }

    }

    Row {
        id: rightRow

        anchors.right: parent.right
        spacing: theme.itemSpacing

        Item {
            id: sep

            width: theme.itemSpacing
            height: root.pageSize
        }

        Base.TxtButton {
            id: nextPage

            enabled: root._startPage + root._calPages <= root.maxPages
            height: root.pageSize
            radius: 0
            text: translator.tr("下一页")
            onClicked: root._startPage = Math.min(root._startPage + root._calPages, root.maxPages)
        }

        Base.InputBar {
            id: goPage

            width: root.pageSize * 1.5
            height: root.pageSize
            underText: root.maxPages
            onAccepted: root._goPage()
        }

        Base.TxtButton {
            id: goPageBtn

            height: root.pageSize
            radius: 0
            text: translator.tr("跳转")
            onClicked: root._goPage()
        }

    }

}

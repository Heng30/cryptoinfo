import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base
import "qrc:/res/qml/Setting/Login" as Login

Flickable {
    width: parent.width
    implicitHeight: 100
    contentWidth: width
    contentHeight: content.height
    clip: true

    Login.SetPS {
        id: setPS
    }

    Login.DelPS {
        id: delPS
    }

    Column {
        id: content

        width: parent.width
        spacing: theme.itemSpacing

        UI {
        }

        WindowMode {
        }

        Lang {
        }

        Data {
        }

        Login.Panel {
        }

        BackupRecover {
        }

        ShortKey {
        }

    }

}

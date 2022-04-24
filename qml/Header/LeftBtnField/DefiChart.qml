import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

BtnField {
    refreshClickedCB: (function() {
        root.defiChartRefresh();
    })
    visible: _defiChartIsChecked
}

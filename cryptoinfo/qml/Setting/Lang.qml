import QtQuick 2.15
import QtQuick.Controls 2.15
import "qrc:/res/qml/Base" as Base

Base.SettingField {
    id: lang

    width: parent.width
    headerText: translator.tr("语言设置")
    spacing: theme.itemSpacing

    contentItem: Row {
        Base.RadioButton {
            id: chineseLang

            width: parent.width / 2
            text: translator.tr("中文")
            checked: !englishLang.checked
            onCheckedChanged: {
                config.use_chinese = checked;
                config.save();
            }
        }

        Base.RadioButton {
            id: englishLang

            width: parent.width / 2
            height: chineseLang.height
            text: translator.tr("English")
            checked: !config.use_chinese
        }

    }

}

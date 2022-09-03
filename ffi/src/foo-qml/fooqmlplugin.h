#ifndef _FOO_QML_PLUGIN_H_
#define _FOO_QML_PLUGIN_H_

#include <QQmlEngineExtensionPlugin>

class FooQmlPlugin : public QQmlEngineExtensionPlugin
{
    Q_OBJECT
    Q_PLUGIN_METADATA(IID QQmlEngineExtensionInterface_iid)
};

#endif // _FOO_QML_PLUGIN_H_

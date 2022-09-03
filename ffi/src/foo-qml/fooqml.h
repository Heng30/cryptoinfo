#ifndef _FOO_QML_H_
#define _FOO_QML_H_

#include <QObject>
#include <QQmlEngine>
#include <QString>

QT_BEGIN_NAMESPACE

class FooQml : public QObject {
    Q_OBJECT

    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged)
    QML_ELEMENT

  public:
    FooQml(QObject *parent = nullptr);
    QString name() const { return m_name; }

  public slots:
    void setName(const QString &name);

  signals:
    void nameChanged(const QString &name);

  private:
    QString m_name;
};

QT_END_NAMESPACE

#endif // _FOO_QML_H_

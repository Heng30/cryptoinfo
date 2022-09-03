#include "fooqml.h"

FooQml::FooQml(QObject *parent) : QObject(parent) {}

void FooQml::setName(const QString &name) {
  if (name != m_name) {
    m_name = name;
    emit nameChanged(m_name);
  }
}

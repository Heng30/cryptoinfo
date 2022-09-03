#ifndef _CUSIMAGE_H_
#define _CUSIMAGE_H_

#include <QImage>
#include <QPainter>
#include <QQuickItem>
#include <QQuickPaintedItem>

QT_BEGIN_NAMESPACE
class CusImage : public QQuickPaintedItem {
  Q_OBJECT
  Q_PROPERTY(QImage image READ image WRITE setImage NOTIFY imageChanged)
  QML_ELEMENT

 public:
  explicit CusImage(QQuickItem *parent = nullptr);

  QImage image() const;
  Q_INVOKABLE void setImage(const QImage &image);
  void paint(QPainter *painter);

 signals:
  void imageChanged();

 private:
  QImage m_image;
};
QT_END_NAMESPACE

#endif  // _CUSIMAGE_H_

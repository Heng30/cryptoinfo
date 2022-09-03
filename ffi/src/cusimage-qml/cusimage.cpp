#include "cusimage.h"

CusImage::CusImage(QQuickItem *parent) : QQuickPaintedItem(parent) {
  m_image = QImage(10, 10, QImage::Format_RGBA8888);
  m_image.fill(QColor(0, 0, 0));
}

void CusImage::paint(QPainter *painter) {
  QRectF rect = boundingRect();
  QImage scaled =
      m_image.scaled(rect.width(), rect.height(), Qt::KeepAspectRatio);
  QPointF center = rect.center() - scaled.rect().center();

  if (center.x() < 0) center.setX(0);
  if (center.y() < 0) center.setY(0);
  painter->drawImage(center, scaled);
}

QImage CusImage::image() const { return m_image; }

void CusImage::setImage(const QImage &image) {
  bool canEmit = m_image != image;
  m_image = image;
  update();

  if (canEmit) emit imageChanged();
}

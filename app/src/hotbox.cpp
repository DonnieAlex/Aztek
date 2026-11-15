// The hotbox: Space held. Rows of menus at the cursor, the zones around them each with a marking menu on a press.
#include "markingmenu.h"
#include "widgets.h"

#include <QDateTime>
#include <QGridLayout>
#include <QGuiApplication>
#include <QKeyEvent>
#include <QLabel>
#include <QMouseEvent>
#include <QPainter>
#include <QScreen>
#include <QToolButton>
#include <cstdlib>

namespace az {

Hotbox::Hotbox(const QList<QMenu*>& menus, int common, QWidget* parent) : QWidget(parent, Qt::Popup | Qt::FramelessWindowHint) {
    setMouseTracking(true);
    box_ = new QWidget(this);
    auto* grid = new QGridLayout(box_);
    grid->setContentsMargins(6, 6, 6, 6);
    grid->setSpacing(3);
    auto add = [this, grid](QMenu* m, int row, int col) {
        auto* b = new QToolButton(box_);
        b->setText(m->title().remove('&'));
        b->setMenu(m);
        b->setPopupMode(QToolButton::InstantPopup);
        b->setObjectName("hotboxButton");
        b->setMinimumWidth(92);
        connect(m, &QMenu::aboutToShow, this, [this] { used_ = true; });
        grid->addWidget(b, row, col);
    };
    // Common menus on top, the menu set's below, as the reference's rows, with the Aztek box between them.
    const int per_row = 9;
    int row = 0, col = 0;
    for (int i = 0; i < common && i < menus.size(); ++i) {
        if (col == per_row) {
            row += 1;
            col = 0;
        }
        add(menus[i], row, col++);
    }
    row += 1;
    auto* mid = new QLabel("Aztek", box_);
    mid->setObjectName("hotboxCenter");
    mid->setAlignment(Qt::AlignCenter);
    mid->setMinimumHeight(26);
    QFont f = mid->font();
    f.setBold(true);
    mid->setFont(f);
    middle_ = mid;
    grid->addWidget(mid, row, per_row / 2 - 1, 1, 3);
    row += 1;
    col = 0;
    for (int i = common; i < menus.size(); ++i) {
        if (col == per_row) {
            row += 1;
            col = 0;
        }
        add(menus[i], row, col++);
    }
    // Laid out now, so the Aztek box's place is known before the first show.
    grid->activate();
    box_->adjustSize();
}

void Hotbox::popup(QPoint global) {
    QScreen* s = QGuiApplication::screenAt(global);
    QRect area = s ? s->geometry() : parentWidget()->window()->geometry();
    under_ = s ? s->grabWindow(0) : QPixmap();
    setGeometry(area);
    center_ = global - area.topLeft();
    // The Aztek box under the cursor, ready for the cameras.
    // A layout places its widgets only once shown: placed here by hand, so the Aztek box's spot is known.
    box_->resize(box_->sizeHint());
    box_->layout()->setGeometry(QRect(QPoint(), box_->size()));
    QPoint mid = middle_ ? middle_->geometry().center() : QPoint(box_->width() / 2, box_->height() / 2);
    box_->move(center_ - mid);
    shown_ms_ = QDateTime::currentMSecsSinceEpoch();
    used_ = false;
    show();
    activateWindow();
    setFocus();
}

int Hotbox::zone_at(QPoint p) const {
    // The Aztek box; the rest of the menus' block; outside it the screen splits along its diagonals, as the reference's
    // zones do.
    QRect mid = box_->geometry();
    if (middle_ && middle_->geometry().translated(mid.topLeft()).contains(p)) return 4;
    if (mid.contains(p)) return 5;
    QPoint d = p - center_;
    if (std::abs(d.x()) * mid.height() > std::abs(d.y()) * mid.width()) return d.x() > 0 ? 1 : 3;
    return d.y() > 0 ? 2 : 0;
}

void Hotbox::paintEvent(QPaintEvent*) {
    QPainter p(this);
    p.setRenderHint(QPainter::Antialiasing);
    QRect mid = box_->geometry();
    if (!under_.isNull()) p.drawPixmap(rect(), under_);
    p.fillRect(rect(), QColor(0, 0, 0, 90));
    p.setBrush(QColor(40, 40, 40, 215));
    p.setPen(Qt::NoPen);
    p.drawRoundedRect(mid, 5, 5);
    // The zone lines, from the block's corners outwards.
    p.setPen(QPen(QColor(210, 210, 210, 90), 1));
    for (QPoint c : {mid.topLeft(), mid.topRight(), mid.bottomLeft(), mid.bottomRight()}) {
        QPointF d = QPointF(c - center_);
        p.drawLine(QPointF(c), QPointF(c) + d * 4);
    }
    p.setPen(QColor(235, 235, 235));
    QFont f = font();
    f.setBold(true);
    p.setFont(f);
    static const char* names[] = {"Panels and Layouts", "Display", "Panel Types", "Selection Masks"};
    int off = 44;
    p.drawText(QRect(center_.x() - 100, mid.top() - off, 200, 20), Qt::AlignCenter, names[0]);
    p.drawText(QRect(mid.right() + 16, center_.y() - 10, 200, 20), Qt::AlignLeft | Qt::AlignVCenter, names[1]);
    p.drawText(QRect(center_.x() - 100, mid.bottom() + off - 20, 200, 20), Qt::AlignCenter, names[2]);
    p.drawText(QRect(mid.left() - 216, center_.y() - 10, 200, 20), Qt::AlignRight | Qt::AlignVCenter, names[3]);
}

void Hotbox::mousePressEvent(QMouseEvent* e) {
    QPoint at = e->position().toPoint();
    int zone = zone_at(at);
    used_ = true;
    if (zone == 5 || !zones_) return;
    QPoint global = e->globalPosition().toPoint();
    QList<MarkItem> items = zones_(zone);
    hide();
    if (!items.isEmpty()) MarkingMenu::show_at(global, items);
}

void Hotbox::keyReleaseEvent(QKeyEvent* e) {
    if (e->key() == Qt::Key_Space && !e->isAutoRepeat()) {
        const bool tap = !used_ && QDateTime::currentMSecsSinceEpoch() - shown_ms_ < 220;
        hide();
        emit spaceReleased(tap);
    }
}

} // namespace az

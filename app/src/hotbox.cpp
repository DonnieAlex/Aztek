// The hotbox: Space held. Rows of menus at the cursor, the zones around them each with a marking menu on a press.
#include "markingmenu.h"
#include "widgets.h"

#include <QDateTime>
#include <QBoxLayout>
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

Hotbox::Hotbox(const QList<QMenu*>& common, const QList<QPair<QString, QList<QMenu*>>>& sets, QMenu* recent, QWidget* parent)
    : QWidget(parent, Qt::Popup | Qt::FramelessWindowHint) {
    setAttribute(Qt::WA_TranslucentBackground);
    setMouseTracking(true);
    box_ = new QWidget(this);
    auto* rows = new QVBoxLayout(box_);
    rows->setContentsMargins(0, 0, 0, 0);
    rows->setSpacing(6);
    auto button = [this](const QString& title, QMenu* menu = nullptr) {
        auto* b = new QToolButton;
        b->setText(QString(title).remove('&'));
        b->setMenu(menu);
        b->setPopupMode(QToolButton::InstantPopup);
        b->setObjectName("hotboxButton");
        if (menu) connect(menu, &QMenu::aboutToShow, this, &Hotbox::mark_used, Qt::UniqueConnection);
        return b;
    };
    auto row = [rows] {
        auto* strip = new QWidget;
        auto* items = new QHBoxLayout(strip);
        items->setContentsMargins(0, 0, 0, 0);
        items->setSpacing(0);
        rows->addWidget(strip, 0, Qt::AlignHCenter);
        return items;
    };
    auto menu_row = [&row, &button](const QList<QMenu*>& menus) {
        auto* items = row();
        for (auto* menu : menus) items->addWidget(button(menu->title(), menu));
        return items->parentWidget();
    };
    menu_row(common);

    auto* pane = row();
    for (const char* label : {"View", "Shading", "Lighting", "Show", "Renderer", "Panels"}) {
        auto* b = button(label);
        pane->addWidget(b);
        panel_buttons_.append(b);
    }

    auto* center = new QWidget;
    auto* controls_row = new QGridLayout(center);
    controls_row->setContentsMargins(0, 6, 0, 6);
    controls_row->setHorizontalSpacing(54);
    auto* recents = button("Recent Commands", recent);
    auto* controls = new QMenu("Hotbox Controls", this);
    auto* options = button(controls->title(), controls);
    auto* mid = new QLabel("Aztek");
    mid->setObjectName("hotboxCenter");
    mid->setAlignment(Qt::AlignCenter);
    middle_ = mid;
    controls_row->addWidget(recents, 0, 0, Qt::AlignRight | Qt::AlignVCenter);
    controls_row->addWidget(mid, 0, 1);
    controls_row->addWidget(options, 0, 2, Qt::AlignLeft | Qt::AlignVCenter);
    const int side = qMax(recents->sizeHint().width(), options->sizeHint().width());
    controls_row->setColumnMinimumWidth(0, side);
    controls_row->setColumnMinimumWidth(2, side);
    rows->addWidget(center, 0, Qt::AlignHCenter);

    for (const auto& [label, menus] : sets) {
        auto* strip = menu_row(menus);
        auto* action = controls->addAction(label);
        action->setCheckable(true);
        action->setChecked(true);
        connect(action, &QAction::toggled, strip, &QWidget::setVisible);
    }
}

void Hotbox::popup(QPoint global, const QList<QAction*>& panel_actions) {
    for (auto* b : panel_buttons_) {
        QMenu* menu = nullptr;
        for (auto* action : panel_actions) {
            if (QString(action->text()).remove('&') != b->text()) continue;
            menu = action->menu();
            break;
        }
        b->setMenu(menu);
        b->setEnabled(menu);
        b->setToolTip(menu ? QString() : QStringLiteral("Unavailable in this panel"));
        if (menu) connect(menu, &QMenu::aboutToShow, this, &Hotbox::mark_used, Qt::UniqueConnection);
    }
    QScreen* s = QGuiApplication::screenAt(global);
    QRect area = s ? s->geometry() : parentWidget()->window()->geometry();
    setGeometry(area);
    center_ = global - area.topLeft();
    shown_ms_ = QDateTime::currentMSecsSinceEpoch();
    used_ = false;
    show();
    // Center on the camera tile after Qt has laid out the nested rows.
    box_->adjustSize();
    box_->layout()->activate();
    middle_->parentWidget()->layout()->activate();
    const QPoint mid = middle_->mapTo(box_, middle_->rect().center());
    const QPoint pos = center_ - mid;
    box_->move(qBound(0, pos.x(), qMax(0, width() - box_->width())),
               qBound(0, pos.y(), qMax(0, height() - box_->height())));
    center_ = box_->pos() + mid;
    activateWindow();
    setFocus();
}

int Hotbox::zone_at(QPoint p) const {
    // The Aztek box; the rest of the menus' block; outside it the screen splits along its diagonals, as the reference's
    // zones do.
    QRect mid = box_->geometry();
    if (middle_->rect().contains(middle_->mapFrom(this, p))) return 4;
    QPoint d = p - center_;
    if (std::abs(d.x()) * mid.height() > std::abs(d.y()) * mid.width()) return d.x() > 0 ? 1 : 3;
    return d.y() > 0 ? 2 : 0;
}

void Hotbox::paintEvent(QPaintEvent*) {
    QPainter p(this);
    p.setCompositionMode(QPainter::CompositionMode_Source);
    // Nearly invisible, but still hit-testable in the marking zones on Windows.
    p.fillRect(rect(), QColor(0, 0, 0, 1));
    p.setCompositionMode(QPainter::CompositionMode_SourceOver);
    p.setRenderHint(QPainter::Antialiasing);
    QRect mid = box_->geometry();
    // Short diagonal guides mark the outer zones without covering the viewport.
    p.setPen(QPen(QColor(230, 230, 230, 165), 1));
    for (QPoint c : {mid.topLeft(), mid.topRight(), mid.bottomLeft(), mid.bottomRight()}) {
        const QPointF d(c.x() < center_.x() ? -1 : 1, c.y() < center_.y() ? -1 : 1);
        p.drawLine(QPointF(c) + d * 10, QPointF(c) + d * 26);
    }
}

void Hotbox::mousePressEvent(QMouseEvent* e) {
    QPoint at = e->position().toPoint();
    int zone = zone_at(at);
    used_ = true;
    if (!zones_) return;
    QPoint global = e->globalPosition().toPoint();
    QList<MarkItem> items = zones_(zone);
    if (!items.isEmpty()) MarkingMenu::show_at(global, items, this);
}

void Hotbox::keyReleaseEvent(QKeyEvent* e) {
    if (e->key() == Qt::Key_Space && !e->isAutoRepeat()) {
        const bool tap = !used_ && QDateTime::currentMSecsSinceEpoch() - shown_ms_ < 220;
        hide();
        emit spaceReleased(tap);
    }
}

} // namespace az

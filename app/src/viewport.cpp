#include <QTimer>
#include "viewport.h"
#include <QDropEvent>
#include <QDragEnterEvent>
#include <QMimeData>
#include "aztek.h"
#include "host.h"
#include "icons.h"
#include "panels.h"
#include <QJsonArray>
#include <QJsonDocument>
#include <QStackedLayout>

#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QGridLayout>
#include <QGuiApplication>
#include <QSettings>
#include <algorithm>
#include <QMenuBar>
#include <QMouseEvent>
#include <QToolBar>
#include <QVBoxLayout>
#include <QWheelEvent>
#include <QWindow>
#include <QPainter>
#include <cmath>
#include <QtGui/qguiapplication_platform.h>

#ifdef Q_OS_WIN
#include <windows.h>
#endif

namespace az {

namespace {
int mods_of(Qt::KeyboardModifiers m) {
    int r = 0;
    if (m & Qt::ShiftModifier) r |= 1;
    if (m & Qt::ControlModifier) r |= 2;
    if (m & Qt::AltModifier) r |= 4;
    return r;
}
int buttons_of(Qt::MouseButtons b) {
    int r = 0;
    if (b & Qt::LeftButton) r |= 1;
    if (b & Qt::MiddleButton) r |= 2;
    if (b & Qt::RightButton) r |= 4;
    return r;
}
int button_of(Qt::MouseButton b) {
    switch (b) {
    case Qt::LeftButton: return 1;
    case Qt::MiddleButton: return 2;
    case Qt::RightButton: return 3;
    default: return 0;
    }
}
QList<ViewportWidget*>& all_views() {
    static QList<ViewportWidget*> v;
    return v;
}
} // namespace

ViewportWidget::ViewportWidget(const QString& camera, QWidget* parent) : QWidget(parent), camera_(camera) {
    setAttribute(Qt::WA_NativeWindow);
    setAttribute(Qt::WA_PaintOnScreen);
    setAttribute(Qt::WA_NoSystemBackground);
    setAttribute(Qt::WA_OpaquePaintEvent);
    setMouseTracking(true);
    setFocusPolicy(Qt::StrongFocus);
    setMinimumSize(64, 64);
    // A camera or light middle-dragged from the Outliner: the view looks through it.
    setAcceptDrops(true);
    all_views().append(this);
    connect(Host::get(), &Host::sceneChanged, this, [this] { update(); });
}

namespace {
// What a dropped node can be looked through: a camera, or a light with a direction, as the reference allows (not a
// point or an ambient light).
bool can_look_through(const QString& node) {
    QJsonArray shapes = ask("listRelatives", QJsonObject{{"args", QJsonArray{node}}, {"shapes", true}, {"fullPath", true}}).toArray();
    QStringList types;
    for (const QJsonValue& v : shapes) types << ask("nodeType", QJsonObject{{"args", QJsonArray{v.toString()}}}).toString();
    types << ask("nodeType", QJsonObject{{"args", QJsonArray{node}}}).toString();
    for (const QString& t : types) {
        if (t == "camera" || t == "spotLight" || t == "directionalLight" || t == "areaLight" || t == "volumeLight") return true;
        if (t.startsWith("aiAreaLight") || t == "aiPhotometricLight" || t == "aiMeshLight") return true;
    }
    return false;
}
} // namespace

void ViewportWidget::dragEnterEvent(QDragEnterEvent* e) {
    const QString node = e->mimeData()->text().trimmed();
    if (!node.isEmpty() && !node.contains('\n') && can_look_through(node)) e->acceptProposedAction();
    else e->ignore();
}

void ViewportWidget::dropEvent(QDropEvent* e) {
    const QString node = e->mimeData()->text().trimmed();
    if (!can_look_through(node)) return;
    e->acceptProposedAction();
    // This view, not the last one used: the one it was dropped on.
    set("camera", QString::fromUtf8(QJsonDocument(QJsonArray{node}).toJson(QJsonDocument::Compact)).mid(1).chopped(1));
    emit activated(this);
}

ViewportWidget::~ViewportWidget() {
    all_views().removeAll(this);
    if (id_) aztek_view_destroy(id_);
}

void ViewportWidget::ensure() {
    if (id_ || !isVisible()) return;
    double dpr = devicePixelRatioF();
    int w = int(width() * dpr), h = int(height() * dpr);
    QByteArray cam = camera_.toUtf8();
#if defined(Q_OS_WIN)
    id_ = aztek_view_create(AZTEK_WIN32, nullptr, quint64(winId()), (void*)GetModuleHandleW(nullptr), w, h, dpr, cam.constData());
#elif defined(Q_OS_LINUX)
    if (auto* x11 = qApp->nativeInterface<QNativeInterface::QX11Application>()) {
        id_ = aztek_view_create(AZTEK_XCB, (void*)x11->connection(), quint64(winId()), nullptr, w, h, dpr, cam.constData());
    } else {
        log("// Error: viewport: run on X11 (QT_QPA_PLATFORM=xcb) //", 2);
    }
#elif defined(Q_OS_MACOS)
    // winId() is the widget's NSView; Vulkan draws to a Metal layer on it through MoltenVK.
    id_ = aztek_view_create(AZTEK_APPKIT, nullptr, quint64(winId()), nullptr, w, h, dpr, cam.constData());
#endif
    if (!id_ && error_.isEmpty()) {
        // Vulkan didn't start: paint the reason with Qt instead of leaving the panel black.
        QString e = take(aztek_view_error());
        if (!e.isEmpty()) {
            error_ = e;
            setAttribute(Qt::WA_PaintOnScreen, false);
            setAttribute(Qt::WA_NoSystemBackground, false);
            update();
        }
    }
    if (id_) {
        for (auto it = pending_.begin(); it != pending_.end(); ++it) aztek_view_set(id_, it.key().section(':', 0, 0).toUtf8().constData(), it.value().toUtf8().constData());
        pending_.clear();
        update();
    }
}

void ViewportWidget::showEvent(QShowEvent*) {
    ensure();
}

void ViewportWidget::paint_error() {
    QPainter p(this);
    p.fillRect(rect(), QColor(64, 64, 64));
    p.setPen(QColor(220, 220, 220));
    QFont f = font();
    p.setFont(f);
    const QString text = tr("No picture: Vulkan could not start (%1).\n\n"
                            "The viewport draws with Vulkan, which comes with your graphics card's own driver. "
                            "Installing the latest driver from the card's maker (NVIDIA, AMD or Intel) usually fixes it. "
                            "On a laptop with two graphics chips, choose the NVIDIA or AMD one for Aztek in the system's graphics settings.\n\n"
                            "Everything else works without it: the Outliner, Channel Box, Attribute Editor, Script Editor and the menus.")
                             .arg(error_);
    const int m = 16;
    p.drawText(rect().adjusted(m, m, -m, -m), Qt::AlignCenter | Qt::TextWordWrap, text);
}

void ViewportWidget::paintEvent(QPaintEvent*) {
    if (!id_ && !error_.isEmpty()) {
        paint_error();
        return;
    }
#ifdef Q_OS_MACOS
    // Cocoa can paint synchronously while committing a layer transaction. Submit Metal frames after it returns,
    // otherwise presenting the frame can keep that transaction painting and starve the application's event loop.
    if (render_queued_) return;
    render_queued_ = true;
    QTimer::singleShot(0, this, [this] {
        render_queued_ = false;
        if (isVisible()) render_frame();
    });
#else
    render_frame();
#endif
}

void ViewportWidget::render_frame() {
    ensure();
    QElapsedTimer t;
    t.start();
    if (id_) aztek_view_render(id_);
    profile_line(QString("render %1").arg(camera_), t);
}

void ViewportWidget::resizeEvent(QResizeEvent*) {
    if (!id_) return;
    double dpr = devicePixelRatioF();
    aztek_view_resize(id_, int(width() * dpr), int(height() * dpr));
    update();
}

bool ViewportWidget::event(QEvent* e) {
    return QWidget::event(e);
}

void ViewportWidget::mouse(int kind, QMouseEvent* e) {
    if (!id_) return;
    double dpr = devicePixelRatioF();
    QPointF p = e->position() * dpr;
    QElapsedTimer t;
    t.start();
    int r = aztek_view_mouse(id_, kind, kind == 1 ? 0 : button_of(e->button()), p.x(), p.y(), mods_of(e->modifiers()), buttons_of(e->buttons()));
    profile_line(QString("mouse %1").arg(kind), t);
    if (r) update();
}

void ViewportWidget::mousePressEvent(QMouseEvent* e) {
    flush_move();
    setFocus();
    emit activated(this);
    // Right button without Alt opens the marking menu; Alt+right dollies.
    if (e->button() == Qt::RightButton && !(e->modifiers() & Qt::AltModifier)) {
        // On the ViewCube, the cube's own menu.
        const QPointF at = e->position() * devicePixelRatioF();
        if (id_ && aztek_view_on_cube(id_, at.x(), at.y())) {
            if (aztek_view_mouse(id_, 0, 3, at.x(), at.y(), mods_of(e->modifiers()), buttons_of(e->buttons()))) update();
            return;
        }
        emit rightClick(e->globalPosition().toPoint(), mods_of(e->modifiers()));
        return;
    }
    if (e->button() == Qt::MiddleButton && !(e->modifiers() & Qt::AltModifier) && !selected_channels().isEmpty()) {
        QJsonArray sel = state()["selection"].toArray();
        slide_plugs_.clear();
        QStringList inputs = selected_input_plugs();
        for (const QString& plug : inputs) {
            QJsonValue v = ask("getAttr", QJsonObject{{"args", QJsonArray{plug}}});
            if (v.isDouble()) slide_plugs_.append({plug, v.toDouble()});
        }
        for (const QJsonValue& o : sel)
            for (const QString& a : selected_channels()) {
                bool taken = false;
                for (const QString& plug : inputs) taken = taken || plug.endsWith("." + a);
                if (taken) continue;
                QString plug = o.toString().section('.', 0, 0) + "." + a;
                QJsonValue v = ask("getAttr", QJsonObject{{"args", QJsonArray{plug}}});
                if (v.isDouble()) slide_plugs_.append({plug, v.toDouble()});
            }
        if (!slide_plugs_.isEmpty()) {
            sliding_ = true;
            slide_start_ = e->position();
            slide_applied_ = 0;
            return;
        }
    }
    mouse(0, e);
}

void ViewportWidget::slide_to(double dx, bool final) {
    for (int i = 0; i < slide_applied_; ++i) aztek_retract();
    slide_applied_ = 0;
    // A tenth per pixel; Ctrl for fine steps, Shift for coarse, as the reference's slider.
    double step = (QGuiApplication::keyboardModifiers() & Qt::ControlModifier) ? 0.01 : (QGuiApplication::keyboardModifiers() & Qt::ShiftModifier) ? 1.0 : 0.1;
    // The Channel Box's Settings: Slowest, Slow, Medium (the default) or Fast, and Hyperbolic, which speeds up the
    // further the drag goes (#137).
    QSettings cb;
    static const double speeds[] = {0.1, 0.5, 1.0, 2.0};
    step *= speeds[std::clamp(cb.value("channelBox/speed", 2).toInt(), 0, 3)];
    if (cb.value("channelBox/hyperbolic", false).toBool()) dx = dx * (1.0 + std::abs(dx) / 50.0);
    if (dx == 0) {
        Host::get()->poke();
        return;
    }
    for (auto& [plug, start] : slide_plugs_) {
        QJsonObject call{{"args", QJsonArray{plug, start + dx * step}}};
        QJsonValue r = final ? exec("setAttr", call) : exec_quiet("setAttr", call);
        if (!final && !r.isUndefined()) ++slide_applied_;
    }
    Host::get()->poke();
    update();
}

void ViewportWidget::mouseMoveEvent(QMouseEvent* e) {
    if (sliding_) {
        slide_to(std::round(e->position().x() - slide_start_.x()), false);
        return;
    }
    queue_move(e);
}

void ViewportWidget::queue_move(QMouseEvent* e) {
    move_at_ = e->position() * devicePixelRatioF();
    move_mods_ = mods_of(e->modifiers());
    move_buttons_ = buttons_of(e->buttons());
    if (move_queued_) return;
    move_queued_ = true;
    QTimer::singleShot(0, this, [this] { flush_move(); });
}

void ViewportWidget::flush_move() {
    if (!move_queued_) return;
    move_queued_ = false;
    if (!id_) return;
    QElapsedTimer t;
    t.start();
    int r = aztek_view_mouse(id_, 1, 0, move_at_.x(), move_at_.y(), move_mods_, move_buttons_);
    profile_line("mouse 1", t);
    if (r) update();
}
void ViewportWidget::mouseReleaseEvent(QMouseEvent* e) {
    flush_move();
    if (sliding_ && e->button() == Qt::MiddleButton) {
        sliding_ = false;
        slide_to(std::round(e->position().x() - slide_start_.x()), true);
        return;
    }
    if (e->button() == Qt::RightButton && !(e->modifiers() & Qt::AltModifier)) return;
    mouse(2, e);
    Host::get()->poke();
}

void ViewportWidget::mouseDoubleClickEvent(QMouseEvent* e) {
    flush_move();
    if (e->modifiers() & Qt::AltModifier) return;
    mouse(3, e);
    Host::get()->poke();
}

void ViewportWidget::leaveEvent(QEvent*) {
    // Drop the pre-selection highlight: a move far outside the view picks nothing.
    if (id_ && aztek_view_mouse(id_, 1, 0, -1e6, -1e6, 0, 0)) update();
}

void ViewportWidget::wheelEvent(QWheelEvent* e) {
    if (!id_) return;
    aztek_view_wheel(id_, e->angleDelta().y() / 8.0);
    update();
}

void ViewportWidget::set(const QString& key, const QString& json) {
    if (!id_) ensure();
    if (!id_) {
        // One-shot requests mean nothing later. `disp` merges, so its pieces each wait under their own key.
        if (key == "frame" || key == "home" || key == "viewUndo" || key == "viewRedo" || key == "isolate") return;
        pending_[key == "disp" ? "disp:" + json : key] = json;
        return;
    }
    aztek_view_set(id_, key.toUtf8().constData(), json.toUtf8().constData());
    update();
}

QString ViewportWidget::get(const QString& key) const {
    if (!id_) return QString();
    return take(aztek_view_get(id_, key.toUtf8().constData()));
}

bool ViewportWidget::capture(const QString& path) {
    ensure();
    return id_ && aztek_view_capture(id_, path.toUtf8().constData());
}

// ---------------------------------------------------------------------------------------------------------------------

ModelPanel::ModelPanel(const QString& camera, QWidget* parent) : QWidget(parent) {
    setFocusPolicy(Qt::StrongFocus);
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(0, 0, 0, 0);
    lay->setSpacing(0);
    menus_ = new QMenuBar(this);
    menus_->setNativeMenuBar(false);
    menus_->setObjectName("panelMenus");
    bar_ = new QToolBar(this);
    bar_->setObjectName("panelBar");
    bar_->setIconSize(QSize(18, 18));
    view_ = new ViewportWidget(camera, this);
    lay->addWidget(menus_);
    lay->addWidget(bar_);
    stack_ = new QStackedLayout();
    stack_->addWidget(view_);
    lay->addLayout(stack_, 1);
    build_menus();
}

// ---------------------------------------------------------------------------------------------------------------------

PanelLayout::PanelLayout(QWidget* parent) : QWidget(parent) {
    grid_ = new QGridLayout(this);
    grid_->setContentsMargins(0, 0, 0, 0);
    grid_->setSpacing(3);
    // The reference's four views: top, persp / front, side. Single view starts on persp.
    for (const char* cam : {"top", "persp", "front", "side"}) {
        auto* p = new ModelPanel(cam, this);
        connect(p->view(), &ViewportWidget::rightClick, this, &PanelLayout::rightClick);
        connect(p->view(), &ViewportWidget::activated, this, [this](ViewportWidget* v) { last_ = v; });
        panels_.append(p);
    }
    focus_ = 1;
    relayout();
}

void PanelLayout::relayout() {
    // Hiding the view that has the keyboard hands it to whatever comes next in the window (the menu set box, which
    // then opens on the next Space). The keyboard stays with the views.
    QWidget* had = QApplication::focusWidget();
    const bool ours = had && (had == this || isAncestorOf(had));
    struct Refocus {
        PanelLayout* l;
        bool on;
        ~Refocus() {
            if (!on) return;
            if (ViewportWidget* v = l->active()) v->setFocus(Qt::OtherFocusReason);
        }
    } refocus{this, ours};
    for (auto* p : panels_) {
        grid_->removeWidget(p);
        p->hide();
    }
    for (int i = 0; i < 2; ++i) {
        grid_->setRowStretch(i, 0);
        grid_->setColumnStretch(i, 0);
    }
    // The panel in focus comes first in the layouts with fewer panes than views.
    QList<ModelPanel*> order{panels_[focus_]};
    for (auto* p : panels_)
        if (p != panels_[focus_]) order << p;
    auto place = [this](ModelPanel* p, int r, int c, int rs, int cs) {
        grid_->addWidget(p, r, c, rs, cs);
        p->show();
        for (int i = r; i < r + rs; ++i) grid_->setRowStretch(i, 1);
        for (int i = c; i < c + cs; ++i) grid_->setColumnStretch(i, 1);
    };
    switch (kind_) {
    case 0: place(order[0], 0, 0, 1, 1); break;
    case 1: place(order[0], 0, 0, 1, 1); place(order[1], 0, 1, 1, 1); break;
    case 2: place(order[0], 0, 0, 1, 1); place(order[1], 1, 0, 1, 1); break;
    case 3: place(order[0], 0, 0, 1, 1); place(order[1], 0, 1, 1, 1); place(order[2], 1, 0, 1, 2); break;
    case 5: place(order[0], 0, 0, 1, 2); place(order[1], 1, 0, 1, 1); place(order[2], 1, 1, 1, 1); break;
    default:
        for (int i = 0; i < 4; ++i) place(panels_[i], i / 2, i % 2, 1, 1);
    }
    if (kind_ == 2 || kind_ == 5) {
        grid_->setRowStretch(0, top_share_);
        grid_->setRowStretch(1, 100 - top_share_);
    }
    if (kind_ == 5) {
        grid_->setColumnStretch(0, left_share_);
        grid_->setColumnStretch(1, 100 - left_share_);
    }
}

void PanelLayout::set_single(bool single) {
    set_layout(single ? 0 : 4);
}

void PanelLayout::set_layout(int kind, int top_share, int left_share) {
    layout_before_single_.clear();
    if (kind_ != 0) {
        last_multi_ = kind_;
        last_multi_focus_ = focus_;
    }
    kind_ = kind;
    top_share_ = top_share;
    left_share_ = left_share;
    relayout();
}

void PanelLayout::set_editor_layout(int kind, const QStringList& types, int top_share, int left_share) {
    if (kind == 0) {
        if (kind_ != 0) {
            layout_before_single_ = state_json();
        }
        focus_ = 1;
        kind_ = 0;
        panels_[focus_]->set_panel_type("model");
        last_ = panels_[focus_]->view();
        last_->set("camera", "\"persp\"");
        relayout();
        return;
    }
    // The perspective viewport keeps its camera in every preset, including when another pane comes first.
    focus_ = types.first() == "model" ? 1 : 0;
    const QList<ModelPanel*> order{panels_[focus_], panels_[1 - focus_], panels_[2], panels_[3]};
    for (int i = 0; i < types.size(); ++i) {
        const QString& type = types[i];
        order[i]->set_panel_type(type);
        if (type == "model") {
            last_ = order[i]->view();
            last_->set("camera", "\"persp\"");
        }
    }
    set_layout(kind, top_share, left_share);
}

void PanelLayout::toggle() {
    if (kind_ == 0) {
        if (!layout_before_single_.isEmpty()) {
            const QString previous = layout_before_single_;
            restore_json(previous);
            return;
        }
        kind_ = last_multi_;
        focus_ = last_multi_focus_;
    } else {
        last_multi_focus_ = focus_;
        ViewportWidget* a = active();
        for (int i = 0; i < panels_.size(); ++i)
            if (panels_[i]->view() == a) focus_ = i;
        last_multi_ = kind_;
        kind_ = 0;
    }
    relayout();
}

QString PanelLayout::state_json() const {
    QJsonArray ps;
    for (auto* p : panels_) ps.append(QJsonObject{{"camera", p->view()->get("camera").remove('"')}, {"type", p->panel_type()}});
    return QString::fromUtf8(QJsonDocument(QJsonObject{{"kind", kind_}, {"focus", focus_}, {"panels", ps},
        {"topShare", top_share_}, {"leftShare", left_share_}}).toJson(QJsonDocument::Compact));
}

void PanelLayout::restore_json(const QString& json) {
    layout_before_single_.clear();
    QJsonObject o = QJsonDocument::fromJson(json.toUtf8()).object();
    QJsonArray ps = o["panels"].toArray();
    for (int i = 0; i < panels_.size() && i < ps.size(); ++i) {
        QJsonObject p = ps[i].toObject();
        panels_[i]->set_panel_type(p["type"].toString("model"));
        if (!p["camera"].toString().isEmpty()) panels_[i]->view()->set("camera", "\"" + p["camera"].toString() + "\"");
    }
    focus_ = qBound(0, o["focus"].toInt(1), int(panels_.size()) - 1);
    kind_ = o["kind"].toInt(0);
    top_share_ = qBound(1, o["topShare"].toInt(50), 99);
    left_share_ = qBound(1, o["leftShare"].toInt(50), 99);
    relayout();
}

ViewportWidget* PanelLayout::active() const {
    QPoint g = QCursor::pos();
    for (auto* p : panels_) {
        ViewportWidget* v = p->view();
        if (v->isVisible() && v->rect().contains(v->mapFromGlobal(g))) {
            last_ = v;
            return v;
        }
    }
    if (last_ && last_->isVisible()) return last_;
    for (auto* p : panels_)
        if (p->view()->isVisible()) return p->view();
    return panels_[focus_]->view();
}

ModelPanel* PanelLayout::panel_at(QPoint global) const {
    for (auto* p : panels_)
        if (p->isVisible() && p->rect().contains(p->mapFromGlobal(global))) return p;
    for (auto* p : panels_)
        if (p->view() == last_ && p->isVisible()) return p;
    return panels_[focus_];
}

QList<ViewportWidget*> PanelLayout::views() const {
    QList<ViewportWidget*> out;
    for (auto* p : panels_) out.append(p->view());
    return out;
}

void PanelLayout::redraw_all() {
    for (auto* p : panels_) p->view()->update();
}

void PanelLayout::set_all(const QString& key, const QString& json) {
    for (auto* p : panels_)
        if (p->isVisible() && p->view()->isVisible()) p->view()->set(key, json);
}

} // namespace az

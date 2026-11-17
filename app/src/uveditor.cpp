#include "uveditor.h"
#include <QSplitter>
#include "aztek.h"
#include "host.h"
#include "icons.h"

#include <QAbstractItemView>
#include <QActionGroup>
#include <QComboBox>
#include <QContextMenuEvent>
#include <QDoubleSpinBox>
#include <QJsonArray>
#include <QKeyEvent>
#include <QLabel>
#include <QMenu>
#include <QMouseEvent>
#include <QPainter>
#include <QPainterPath>
#include <QTimer>
#include <QSlider>
#include <QDateTime>
#include <QToolBar>
#include <QVBoxLayout>
#include <QWheelEvent>
#include <algorithm>
#include <QFontMetricsF>
#include <cmath>

namespace az {

// The ids as runs, "4" or "6:9": one component name each (pSphere1.e[4] pSphere1.e[6:9]), as the reference writes them.
static QStringList id_runs(QList<int> ids) {
    std::sort(ids.begin(), ids.end());
    ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
    QStringList parts;
    for (int i = 0; i < ids.size();) {
        int j = i;
        while (j + 1 < ids.size() && ids[j + 1] == ids[j] + 1) ++j;
        parts << (j > i ? QString("%1:%2").arg(ids[i]).arg(ids[j]) : QString::number(ids[i]));
        i = j + 1;
    }
    return parts;
}

// ---------------------------------------------------------------------------------------------------------------------
// Canvas

UvCanvas::UvCanvas(QWidget* parent) : QWidget(parent) {
    setFocusPolicy(Qt::StrongFocus);
    setMouseTracking(true);
    setMinimumSize(240, 240);
}

QPointF UvCanvas::to_screen(QPointF uv) const {
    return QPointF(width() / 2.0 + (uv.x() - center_.x()) * scale_, height() / 2.0 - (uv.y() - center_.y()) * scale_);
}

QPointF UvCanvas::to_uv(QPointF p) const {
    return QPointF(center_.x() + (p.x() - width() / 2.0) / scale_, center_.y() - (p.y() - height() / 2.0) / scale_);
}

void UvCanvas::set_data(const QJsonObject& d) {
    scene_mode_ = d["mode"].toString();
    meshes_.clear();
    for (const QJsonValue& mv : d["meshes"].toArray()) {
        QJsonObject o = mv.toObject();
        UvMesh m;
        m.path = o["path"].toString();
        for (const QJsonValue& p : o["maps"].toArray()) {
            QJsonArray a = p.toArray();
            m.maps.append(QPointF(a[0].toDouble(), a[1].toDouble()));
            m.map_point.append(a[2].toInt());
        }
        for (const QJsonValue& fv : o["faces"].toArray()) {
            QJsonArray a = fv.toArray();
            UvMesh::Face f;
            f.id = a[0].toInt();
            f.shell = a[1].toInt();
            for (const QJsonValue& x : a[2].toArray()) f.maps.append(x.toInt());
            for (const QJsonValue& x : a[3].toArray()) f.edges.append(x.toInt());
            f.area3d = a.size() > 4 ? a[4].toDouble() : 0.0;
            m.faces.append(f);
        }
        for (const QJsonValue& bv : o["borders"].toArray()) {
            QJsonArray a = bv.toArray();
            m.borders.append({a[0].toInt(), a[1].toInt(), a[2].toInt()});
        }
        for (const QJsonValue& x : o["selFaces"].toArray()) m.sel_faces.insert(x.toInt());
        for (const QJsonValue& x : o["selEdges"].toArray()) m.sel_edges.insert(x.toInt());
        for (const QJsonValue& x : o["selPoints"].toArray()) m.sel_points.insert(x.toInt());
        // The scene's UV selection, which a view may have changed.
        if (o.contains("selMaps")) {
            QSet<int>& s = uv_sel_[m.path];
            s.clear();
            for (const QJsonValue& x : o["selMaps"].toArray()) s.insert(x.toInt());
        }
        // Selected UVs that no longer exist go.
        if (uv_sel_.contains(m.path)) {
            QSet<int>& s = uv_sel_[m.path];
            for (auto it = s.begin(); it != s.end();) it = *it >= m.maps.size() ? s.erase(it) : std::next(it);
        }
        meshes_.append(m);
    }
    if (!framed_ && !meshes_.isEmpty()) {
        framed_ = true;
        center_ = QPointF(0.5, 0.5);
        scale_ = qMin(width(), height()) * 0.85;
    }
    update();
}

void UvCanvas::set_mode(const QString& m) {
    mode_ = m;
    // The selection type is the scene's, shared with the views. Object mode shows the shells alone, no UV points (#99).
    if (m == "face" || m == "edge" || m == "uv" || m == "object") exec("selectMode", QJsonObject{{"args", QJsonArray{m}}});
    else if (m == "shell") exec("selectType", QJsonObject{{"meshUVShell", true}});
    update();
}

QStringList UvCanvas::mesh_paths() const {
    QStringList out;
    for (const UvMesh& m : meshes_) out << m.path;
    return out;
}

void UvCanvas::select_all_uvs(bool invert) {
    for (const UvMesh& m : meshes_) {
        QSet<int>& s = uv_sel_[m.path];
        QSet<int> next;
        for (int i = 0; i < m.maps.size(); ++i)
            if (!invert || !s.contains(i)) next.insert(i);
        s = next;
    }
    if (mode_ != "uv" && mode_ != "shell") mode_ = "uv";
    update();
    emit picked();
}

void UvCanvas::follow_mode(const QString& m) {
    mode_ = m;
    update();
}

void UvCanvas::share_uv_selection() {
    QJsonArray names;
    for (auto it = uv_sel_.begin(); it != uv_sel_.end(); ++it)
        for (const QString& run : id_runs(it.value().values())) names.append(QString("%1.map[%2]").arg(it.key(), run));
    if (names.isEmpty()) {
        // Nothing picked: the meshes stay, their UVs go.
        QJsonArray objs;
        for (const UvMesh& m : meshes_) objs.append(m.path);
        exec("select", QJsonObject{{"args", objs}});
    } else {
        exec("select", QJsonObject{{"args", names}});
    }
    if (mode_ == "shell") exec("selectType", QJsonObject{{"meshUVShell", true}});
    else if (mode_ == "uv") exec("selectMode", QJsonObject{{"args", QJsonArray{"uv"}}});
}

void UvCanvas::set_tool(const QString& t) {
    // The shell tool is the move tool picking whole shells.
    if (t == "shell") {
        set_mode("shell");
        tool_ = "move";
    } else {
        tool_ = t;
    }
    lasso_.clear();
    path_map_ = -1;
    if (tool_ == "lattice") lattice_reset();
    update();
}

// A fresh lattice over the selected UVs, a little bigger than they are.
void UvCanvas::lattice_reset() {
    lat_pts_.clear();
    QRectF b = selection_bounds();
    if (b.isNull()) {
        lat_box_ = QRectF();
        return;
    }
    const double pad = std::max(b.width(), b.height()) * 0.05 + 1e-3;
    lat_box_ = b.adjusted(-pad, -pad, pad, pad);
    for (int j = 0; j < lat_n_; ++j)
        for (int i = 0; i < lat_n_; ++i)
            lat_pts_.append(QPointF(lat_box_.left() + lat_box_.width() * i / (lat_n_ - 1), lat_box_.top() + lat_box_.height() * j / (lat_n_ - 1)));
}

void UvCanvas::lattice_apply(bool final) {
    if (applied_) aztek_retract();
    applied_ = false;
    if (lat_box_.isNull() || drag_targets_.isEmpty()) return;
    QJsonArray pts;
    for (const QPointF& p : lat_pts_) pts << p.x() << p.y();
    const QJsonObject call{{"args", drag_targets_}, {"columns", lat_n_}, {"rows", lat_n_}, {"box", QJsonArray{lat_box_.left(), lat_box_.top(), lat_box_.right(), lat_box_.bottom()}}, {"points", pts}};
    QJsonValue r = final ? exec("texLatticeUVs", call) : exec_quiet("texLatticeUVs", call);
    applied_ = !final && !r.isUndefined();
    Host::get()->poke();
}

// A left press with one of the tools that are not a manipulator. True when it was taken.
void UvCanvas::tool_press(QMouseEvent* e) {
    QPointF pos = e->position();
    bool ctrl = e->modifiers() & Qt::ControlModifier;
    if (tool_ == "lasso") {
        drag_ = Mode::Lasso;
        lasso_.clear();
        lasso_ << pos;
    } else if (tool_ == "window") {
        drag_ = Mode::Pan;
    } else if (tool_ == "tweak") {
        // The nearest UV, picked and dragged at once.
        double best = 14;
        QString mesh;
        int id = -1;
        for (const UvMesh& m : meshes_)
            for (int i = 0; i < m.maps.size(); ++i) {
                double d = QLineF(to_screen(m.maps[i]), pos).length();
                if (d < best) best = d, mesh = m.path, id = i;
            }
        if (id < 0) return;
        uv_sel_.clear();
        uv_sel_[mesh].insert(id);
        mode_ = "uv";
        drag_ = Mode::Drag;
        handle_ = 3;
        pivot_ = selection_center();
        applied_ = false;
        drag_targets_ = targets();
        emit picked();
        update();
    } else if (tool_ == "cut") {
        // Click an edge: cut it. Ctrl-click: sew it.
        double best = 8;
        QString mesh;
        int edge = -1;
        for (const UvMesh& m : meshes_)
            for (const auto& fc : m.faces)
                for (int k = 0; k < fc.edges.size(); ++k) {
                    QLineF l(to_screen(m.maps[fc.maps[k]]), to_screen(m.maps[fc.maps[(k + 1) % fc.maps.size()]]));
                    double t = qBound(0.0, QPointF::dotProduct(pos - l.p1(), l.p2() - l.p1()) / qMax(1e-9, l.length() * l.length()), 1.0);
                    double d = QLineF(l.pointAt(t), pos).length();
                    if (d < best) best = d, mesh = m.path, edge = fc.edges[k];
                }
        if (edge < 0) return;
        exec(ctrl ? "polyMapSew" : "polyMapCut", QJsonObject{{"args", QJsonArray{QString("%1.e[%2]").arg(mesh).arg(edge)}}});
        Host::get()->poke();
    } else if (tool_ == "sew") {
        // The Sew brush: the edge clicked sewn.
        double best = 8;
        QString mesh;
        int edge = -1;
        for (const UvMesh& m : meshes_)
            for (const auto& fc : m.faces)
                for (int k = 0; k < fc.edges.size(); ++k) {
                    QLineF l(to_screen(m.maps[fc.maps[k]]), to_screen(m.maps[fc.maps[(k + 1) % fc.maps.size()]]));
                    double t = qBound(0.0, QPointF::dotProduct(pos - l.p1(), l.p2() - l.p1()) / qMax(1e-9, l.length() * l.length()), 1.0);
                    double d = QLineF(l.pointAt(t), pos).length();
                    if (d < best) best = d, mesh = m.path, edge = fc.edges[k];
                }
        if (edge < 0) return;
        exec("polyMapSew", QJsonObject{{"args", QJsonArray{QString("%1.e[%2]").arg(mesh).arg(edge)}}});
        Host::get()->poke();
    } else if (tool_ == "path") {
        pick_path(pos);
    } else if (tool_ == "smooth") {
        // A press runs the unfold over the selected UVs, or the meshes shown.
        QJsonArray t = targets();
        exec("u3dUnfold", t.isEmpty() ? QJsonObject() : QJsonObject{{"args", t}});
        Host::get()->poke();
    }
}

// What the lasso drew round, as a UV selection.
void UvCanvas::pick_lasso(Qt::KeyboardModifiers mods) {
    bool ctrl = mods & Qt::ControlModifier, shift = mods & Qt::ShiftModifier;
    if (!ctrl && !shift) uv_sel_.clear();
    mode_ = "uv";
    // As a click: Shift toggles, Ctrl removes, Ctrl+Shift adds (#114).
    for (const UvMesh& m : meshes_)
        for (int i = 0; i < m.maps.size(); ++i)
            if (lasso_.containsPoint(to_screen(m.maps[i]), Qt::OddEvenFill)) {
                if (ctrl && !shift) uv_sel_[m.path].remove(i);
                else if (shift && !ctrl && uv_sel_[m.path].contains(i)) uv_sel_[m.path].remove(i);
                else uv_sel_[m.path].insert(i);
            }
    lasso_.clear();
    emit picked();
    update();
}

// The nearest UV: the shortest way along the UV edges from the last one picked is selected.
void UvCanvas::pick_path(QPointF pos) {
    double best = 14;
    int mi = -1, id = -1;
    for (int k = 0; k < meshes_.size(); ++k)
        for (int i = 0; i < meshes_[k].maps.size(); ++i) {
            double d = QLineF(to_screen(meshes_[k].maps[i]), pos).length();
            if (d < best) best = d, mi = k, id = i;
        }
    if (mi < 0) return;
    const UvMesh& m = meshes_[mi];
    mode_ = "uv";
    if (path_mesh_ != m.path || path_map_ < 0) {
        uv_sel_.clear();
        uv_sel_[m.path].insert(id);
    } else {
        // Dijkstra over the maps, by length in UV space.
        QHash<int, QList<int>> next;
        for (const auto& fc : m.faces)
            for (int k = 0; k < fc.maps.size(); ++k) {
                int a = fc.maps[k], b = fc.maps[(k + 1) % fc.maps.size()];
                next[a] << b;
                next[b] << a;
            }
        QHash<int, double> dist{{path_map_, 0.0}};
        QHash<int, int> from;
        QSet<int> done;
        while (true) {
            int cur = -1;
            double low = 1e300;
            for (auto it = dist.begin(); it != dist.end(); ++it)
                if (!done.contains(it.key()) && it.value() < low) low = it.value(), cur = it.key();
            if (cur < 0 || cur == id) break;
            done.insert(cur);
            for (int n : next.value(cur)) {
                double d = low + QLineF(m.maps[cur], m.maps[n]).length();
                if (!dist.contains(n) || d < dist[n]) {
                    dist[n] = d;
                    from[n] = cur;
                }
            }
        }
        QSet<int>& s = uv_sel_[m.path];
        s.insert(id);
        for (int at = id; from.contains(at); at = from[at]) s.insert(from[at]);
    }
    path_mesh_ = m.path;
    path_map_ = id;
    emit picked();
    update();
}

QHash<int, QSet<int>> UvCanvas::active_maps() const {
    QHash<int, QSet<int>> out;
    for (int mi = 0; mi < meshes_.size(); ++mi) {
        const UvMesh& m = meshes_[mi];
        QSet<int> s;
        if (mode_ == "uv" || mode_ == "shell") {
            s = uv_sel_.value(m.path);
        } else if (mode_ == "face") {
            for (const auto& f : m.faces)
                if (m.sel_faces.contains(f.id))
                    for (int x : f.maps) s.insert(x);
        } else if (mode_ == "edge") {
            for (const auto& f : m.faces)
                for (int k = 0; k < f.edges.size(); ++k)
                    if (m.sel_edges.contains(f.edges[k])) {
                        s.insert(f.maps[k]);
                        s.insert(f.maps[(k + 1) % f.maps.size()]);
                    }
        }
        if (!s.isEmpty()) out[mi] = s;
    }
    return out;
}

QJsonArray UvCanvas::targets() const {
    QJsonArray out;
    auto am = active_maps();
    for (auto it = am.begin(); it != am.end(); ++it)
        for (const QString& run : id_runs(it.value().values())) out.append(QString("%1.map[%2]").arg(meshes_[it.key()].path, run));
    return out;
}

QPointF UvCanvas::selection_center() const {
    auto am = active_maps();
    double lx = 1e18, ly = 1e18, hx = -1e18, hy = -1e18;
    for (auto it = am.begin(); it != am.end(); ++it)
        for (int id : it.value()) {
            QPointF p = meshes_[it.key()].maps[id];
            lx = qMin(lx, p.x()), ly = qMin(ly, p.y()), hx = qMax(hx, p.x()), hy = qMax(hy, p.y());
        }
    if (lx > hx) return QPointF(-1e9, -1e9);
    return QPointF((lx + hx) / 2, (ly + hy) / 2);
}

QRectF UvCanvas::selection_bounds() const {
    auto am = active_maps();
    double lx = 1e18, ly = 1e18, hx = -1e18, hy = -1e18;
    for (auto it = am.begin(); it != am.end(); ++it)
        for (int id : it.value()) {
            QPointF p = meshes_[it.key()].maps[id];
            lx = qMin(lx, p.x()), ly = qMin(ly, p.y()), hx = qMax(hx, p.x()), hy = qMax(hy, p.y());
        }
    if (lx > hx) return QRectF();
    return QRectF(QPointF(lx, ly), QPointF(hx, hy));
}

void UvCanvas::remember_view() {
    // Steps a moment apart are one change, as a wheel's turn is.
    const qint64 now = QDateTime::currentMSecsSinceEpoch();
    if (!view_back_.isEmpty() && now - last_view_push_ < 400) {
        last_view_push_ = now;
        return;
    }
    last_view_push_ = now;
    view_back_.append({center_, scale_});
    if (view_back_.size() > 100) view_back_.removeFirst();
    view_ahead_.clear();
}

void UvCanvas::undo_view(bool redo) {
    QVector<ViewState>& from = redo ? view_ahead_ : view_back_;
    QVector<ViewState>& to = redo ? view_back_ : view_ahead_;
    if (from.isEmpty()) return;
    to.append({center_, scale_});
    const ViewState v = from.takeLast();
    center_ = v.center;
    scale_ = v.scale;
    update();
}

void UvCanvas::frame(bool selected) {
    remember_view();
    double lx = 1e18, ly = 1e18, hx = -1e18, hy = -1e18;
    auto am = active_maps();
    for (int mi = 0; mi < meshes_.size(); ++mi)
        for (int i = 0; i < meshes_[mi].maps.size(); ++i) {
            if (selected && !am.value(mi).contains(i)) continue;
            QPointF p = meshes_[mi].maps[i];
            lx = qMin(lx, p.x()), ly = qMin(ly, p.y()), hx = qMax(hx, p.x()), hy = qMax(hy, p.y());
        }
    if (lx > hx) {
        if (selected) return frame(false);
        lx = ly = 0, hx = hy = 1;
    }
    center_ = QPointF((lx + hx) / 2, (ly + hy) / 2);
    double span = qMax(qMax(hx - lx, hy - ly), 0.05);
    scale_ = qMin(width(), height()) * 0.85 / span;
    update();
}

void UvCanvas::set_image(const QString& path, QPointF repeat, QPointF offset) {
    image_repeat_ = repeat;
    image_offset_ = offset;
    if (path == image_path_) return;
    image_path_ = path;
    image_ = QImage();
    if (path.isEmpty()) return;
    int w = 0, h = 0;
    QByteArray utf = path.toUtf8();
    if (unsigned char* data = aztek_image_load(utf.constData(), &w, &h)) {
        image_ = QImage(data, w, h, QImage::Format_RGBA8888).copy();
        aztek_image_free(data, w, h);
    }
}

void UvCanvas::paintEvent(QPaintEvent*) {
    QPainter p(this);
    p.fillRect(rect(), QColor(64, 64, 64));
    // The 0-1 square: checker or plain, with the tenths grid.
    QRectF unit(to_screen(QPointF(0, 1)), to_screen(QPointF(1, 0)));
    if (show_checker) {
        int n = 8;
        for (int i = 0; i < n; ++i)
            for (int j = 0; j < n; ++j) {
                QRectF c(unit.left() + unit.width() * i / n, unit.top() + unit.height() * j / n, unit.width() / n, unit.height() / n);
                p.fillRect(c, (i + j) % 2 ? QColor(150, 150, 150) : QColor(105, 105, 105));
            }
    } else {
        p.fillRect(unit, QColor(78, 78, 78));
    }
    // The image: one tile per repeat over the 0-1 square, as the material maps it.
    if (show_image && !image_.isNull() && image_repeat_.x() > 0 && image_repeat_.y() > 0) {
        double tu = 1.0 / image_repeat_.x(), tv = 1.0 / image_repeat_.y();
        double u0 = -image_offset_.x() * tu, v0 = -image_offset_.y() * tv;
        // The channels and range asked for: RGB drops the alpha, Alpha shows it as grey; Image Range stretches the
        // values between its ends to the whole range.
        QImage shown = image_;
        if (image_channels != 1 || image_lo > 0.0 || image_hi < 1.0 || image_exposure != 0.0 || image_gamma != 1.0) {
            shown = image_.convertToFormat(QImage::Format_RGBA8888);
            const double span = std::max(image_hi - image_lo, 1e-6);
            const double gain = std::pow(2.0, image_exposure), inv_gamma = 1.0 / std::max(image_gamma, 1e-3);
            auto range = [&](int c) {
                const double x = std::clamp((c / 255.0 * gain - image_lo) / span, 0.0, 1.0);
                return int(std::pow(x, inv_gamma) * 255.0 + 0.5);
            };
            for (int y = 0; y < shown.height(); ++y) {
                uchar* row = shown.scanLine(y);
                for (int x = 0; x < shown.width(); ++x) {
                    uchar* px = row + x * 4;
                    if (image_channels == 2) px[0] = px[1] = px[2] = px[3];
                    for (int k = 0; k < 3; ++k) px[k] = uchar(range(px[k]));
                    if (image_channels != 1) px[3] = 255;
                }
            }
        }
        // Image Ratio: the image keeps its own proportions, its height fitted to the tile.
        double ratio = image_ratio && shown.height() > 0 ? double(shown.width()) / shown.height() : 1.0;
        p.save();
        p.setClipRect(image_ratio ? QRectF(to_screen(QPointF(0, 1)), to_screen(QPointF(ratio, 0))) : unit);
        p.setRenderHint(QPainter::SmoothPixmapTransform, !image_unfiltered);
        p.setOpacity((dim_image ? 0.45 : 1.0) * image_opacity);
        for (double v = v0 - std::ceil(v0 / tv) * tv; v < 1.0; v += tv)
            for (double u = u0 - std::ceil(u0 / tu) * tu; u < ratio; u += tu)
                p.drawImage(QRectF(to_screen(QPointF(u, v + tv)), to_screen(QPointF(u + tu * ratio, v))), shown);
        p.restore();
    }
    QPointF lo = to_uv(QPointF(0, height())), hi = to_uv(QPointF(width(), 0));
    double step = scale_ > 150 ? 0.1 : 1.0;
    for (double u = std::floor(lo.x() / step) * step; show_grid && u <= hi.x(); u += step) {
        bool whole = std::abs(u - std::round(u)) < 1e-6;
        p.setPen(whole ? QColor(40, 40, 40) : QColor(86, 86, 86));
        double x = to_screen(QPointF(u, 0)).x();
        p.drawLine(QPointF(x, 0), QPointF(x, height()));
    }
    for (double v = std::floor(lo.y() / step) * step; show_grid && v <= hi.y(); v += step) {
        bool whole = std::abs(v - std::round(v)) < 1e-6;
        p.setPen(whole ? QColor(40, 40, 40) : QColor(86, 86, 86));
        double y = to_screen(QPointF(0, v)).y();
        p.drawLine(QPointF(0, y), QPointF(width(), y));
    }
    p.setPen(QPen(QColor(25, 25, 25), 1.5));
    p.setBrush(Qt::NoBrush);
    p.drawRect(unit);
    QFont f = font();
    f.setPointSizeF(qMax(6.5, f.pointSizeF() - 1));
    p.setFont(f);
    // Grid numbers: each whole unit along the bottom and left of the view, tenths when close enough, and each UDIM
    // tile's number (1001 for 0-1, then 1002 along U, 1011 up V) at its corner.
    p.setPen(QColor(160, 160, 160));
    const QFontMetricsF fm(f);
    const double label_step = scale_ > 400 ? 0.1 : scale_ > 40 ? 1.0 : scale_ > 4 ? 10.0 : 100.0;
    auto num = [](double x) { return std::abs(x - std::round(x)) < 1e-6 ? QString::number(qRound64(x)) : QString::number(x, 'f', 1); };
    for (double u = std::ceil(lo.x() / label_step) * label_step; u <= hi.x(); u += label_step) {
        const double x = to_screen(QPointF(u, 0)).x();
        p.drawText(QPointF(x + 2, height() - 4), num(u));
    }
    for (double v = std::ceil(lo.y() / label_step) * label_step; v <= hi.y(); v += label_step) {
        const double y = to_screen(QPointF(0, v)).y();
        p.drawText(QPointF(3, y - 2), num(v));
    }
    if (scale_ > 60) {
        p.setPen(QColor(120, 120, 120));
        for (int v = int(std::floor(lo.y())); v <= int(std::ceil(hi.y())); ++v)
            for (int u = int(std::floor(lo.x())); u <= int(std::ceil(hi.x())); ++u) {
                if (u < 0 || u > 9 || v < 0) continue;
                const QPointF at = to_screen(QPointF(u, v + 1));
                const QString tile = QString::number(1001 + u + 10 * v);
                p.drawText(at + QPointF(4, fm.ascent() + 2), tile);
            }
    }

    p.setRenderHint(QPainter::Antialiasing);
    auto am = active_maps();
    for (int mi = 0; mi < meshes_.size(); ++mi) {
        const UvMesh& m = meshes_[mi];
        // UV Distortion: each face's share of the UVs against its share of the surface. Red where the texture
        // stretches (less UV than surface), blue where it squeezes, white where it is even.
        auto uv_area = [&m](const UvMesh::Face& fc) {
            double a = 0;
            for (int k = 0; k < fc.maps.size(); ++k) {
                const QPointF p = m.maps[fc.maps[k]], q = m.maps[fc.maps[(k + 1) % fc.maps.size()]];
                a += p.x() * q.y() - q.x() * p.y();
            }
            return std::abs(a) * 0.5;
        };
        double uv_total = 0, surf_total = 0;
        if (show_distortion)
            for (const auto& fc : m.faces) {
                uv_total += uv_area(fc);
                surf_total += fc.area3d;
            }
        // Faces.
        for (const auto& fc : m.faces) {
            QPolygonF poly;
            for (int id : fc.maps) poly << to_screen(m.maps[id]);
            if (show_distortion && uv_total > 0 && surf_total > 0 && fc.area3d > 0) {
                const double r = std::log2(std::max(uv_area(fc) / uv_total, 1e-12) / (fc.area3d / surf_total));
                const double t = std::clamp(std::abs(r) / 2.0, 0.0, 1.0);
                const QColor c = r < 0 ? QColor(255, int(255 * (1 - t)), int(255 * (1 - t)), 150) : QColor(int(255 * (1 - t)), int(255 * (1 - t)), 255, 150);
                p.setPen(Qt::NoPen);
                p.setBrush(c);
                p.drawPolygon(poly);
            }
            // A UV shell picked whole shows as a block, the way faces do.
            bool sel = (mode_ == "face" && m.sel_faces.contains(fc.id)) || (mode_ == "shell" && !fc.maps.isEmpty() && std::all_of(fc.maps.begin(), fc.maps.end(), [&](int id) { return am.value(mi).contains(id); }));
            if (sel) {
                p.setPen(Qt::NoPen);
                p.setBrush(QColor(235, 140, 40, 130));
                p.drawPolygon(poly);
            } else if (shaded) {
                p.setPen(Qt::NoPen);
                p.setBrush(QColor(60, 105, 180, 70));
                p.drawPolygon(poly);
            }
            p.setPen(QPen(QColor(170, 175, 195), 1));
            p.setBrush(Qt::NoBrush);
            p.drawPolygon(poly);
        }
        // Where UVs split: thicker, as the reference's texture borders; with shell borders on, each shell's own colour.
        if (show_texture_borders || show_shell_borders) {
            QHash<int, int> shell_of;
            if (show_shell_borders)
                for (const auto& fc : m.faces)
                    for (int id : fc.maps) shell_of.insert(id, fc.shell);
            for (auto& b : m.borders) {
                QColor c(235, 235, 245);
                if (show_shell_borders) c = QColor::fromHsv((shell_of.value(b[0]) * 67) % 360, 200, 255);
                p.setPen(QPen(c, show_texture_borders ? 2 : 1.2));
                p.drawLine(to_screen(m.maps[b[0]]), to_screen(m.maps[b[1]]));
            }
        }
        if (mode_ == "edge") {
            p.setPen(QPen(QColor(255, 150, 30), 2.5));
            for (const auto& fc : m.faces)
                for (int k = 0; k < fc.edges.size(); ++k)
                    if (m.sel_edges.contains(fc.edges[k])) p.drawLine(to_screen(m.maps[fc.maps[k]]), to_screen(m.maps[fc.maps[(k + 1) % fc.maps.size()]]));
        }
        if (mode_ == "uv" || mode_ == "shell") {
            const QSet<int> s = am.value(mi);
            for (int i = 0; i < m.maps.size(); ++i) {
                bool on = s.contains(i);
                p.setPen(Qt::NoPen);
                p.setBrush(on ? QColor(80, 240, 80) : QColor(190, 120, 230));
                double r = on ? 3 : 2;
                p.drawEllipse(to_screen(m.maps[i]), r, r);
            }
        }
    }
    // The manipulator at the selection's middle.
    QPointF c = selection_center();
    if (c.x() > -1e8 && tool_ != "select") {
        QPointF o = to_screen(c);
        p.setRenderHint(QPainter::Antialiasing);
        if (tool_ == "move") {
            p.setPen(QPen(QColor(230, 60, 60), 2));
            p.drawLine(o, o + QPointF(60, 0));
            p.setBrush(QColor(230, 60, 60));
            p.drawPolygon(QPolygonF({o + QPointF(66, 0), o + QPointF(56, -5), o + QPointF(56, 5)}));
            p.setPen(QPen(QColor(80, 220, 80), 2));
            p.drawLine(o, o + QPointF(0, -60));
            p.setBrush(QColor(80, 220, 80));
            p.drawPolygon(QPolygonF({o + QPointF(0, -66), o + QPointF(-5, -56), o + QPointF(5, -56)}));
            p.setPen(QPen(QColor(240, 220, 60), 1.5));
            p.setBrush(Qt::NoBrush);
            p.drawRect(QRectF(o.x() - 6, o.y() - 6, 12, 12));
        } else if (tool_ == "rotate") {
            p.setPen(QPen(QColor(80, 140, 240), 2));
            p.setBrush(Qt::NoBrush);
            p.drawEllipse(o, 50, 50);
        } else if (tool_ == "scale") {
            p.setPen(QPen(QColor(230, 60, 60), 2));
            p.drawLine(o, o + QPointF(60, 0));
            p.fillRect(QRectF(o.x() + 56, o.y() - 4, 8, 8), QColor(230, 60, 60));
            p.setPen(QPen(QColor(80, 220, 80), 2));
            p.drawLine(o, o + QPointF(0, -60));
            p.fillRect(QRectF(o.x() - 4, o.y() - 64, 8, 8), QColor(80, 220, 80));
            p.fillRect(QRectF(o.x() - 6, o.y() - 6, 12, 12), QColor(240, 220, 60));
        }
    }
    if (tool_ == "lattice" && !lat_box_.isNull() && lat_pts_.size() == lat_n_ * lat_n_) {
        p.setRenderHint(QPainter::Antialiasing);
        p.setPen(QPen(QColor(160, 90, 230), 1.5));
        for (int j = 0; j < lat_n_; ++j)
            for (int i = 0; i < lat_n_; ++i) {
                const QPointF a = to_screen(lat_pts_[j * lat_n_ + i]);
                if (i + 1 < lat_n_) p.drawLine(a, to_screen(lat_pts_[j * lat_n_ + i + 1]));
                if (j + 1 < lat_n_) p.drawLine(a, to_screen(lat_pts_[(j + 1) * lat_n_ + i]));
            }
        p.setBrush(QColor(160, 90, 230));
        for (const QPointF& q : lat_pts_) p.drawRect(QRectF(to_screen(q) - QPointF(3, 3), QSizeF(6, 6)));
    }
    if (has_pivot) {
        // The Transform section's own pivot.
        const QPointF o = to_screen(pivot_uv);
        p.setRenderHint(QPainter::Antialiasing);
        p.setPen(QPen(QColor(250, 200, 60), 1.5));
        p.setBrush(Qt::NoBrush);
        p.drawEllipse(o, 5, 5);
        p.drawLine(o + QPointF(-9, 0), o + QPointF(9, 0));
        p.drawLine(o + QPointF(0, -9), o + QPointF(0, 9));
    }
    if (is_brush() && rect().contains(hover_.toPoint())) {
        // The brush: its reach, and a dot where it presses.
        p.setRenderHint(QPainter::Antialiasing);
        p.setPen(QPen(QColor(250, 120, 40), 1.5));
        p.setBrush(Qt::NoBrush);
        p.drawEllipse(hover_, brush_px_, brush_px_);
        p.drawEllipse(hover_, 1.5, 1.5);
    }
    if (drag_ == Mode::Marquee) {
        p.setPen(QPen(QColor(230, 230, 230), 1, Qt::DashLine));
        p.setBrush(Qt::NoBrush);
        p.drawRect(QRectF(press_, last_).normalized());
    }
    if (meshes_.isEmpty()) {
        p.setPen(QColor(170, 170, 170));
        p.drawText(rect(), Qt::AlignCenter, "Select a mesh to see its UVs.");
    }
}

int UvCanvas::handle_at(QPointF pos) const {
    QPointF c = selection_center();
    if (c.x() < -1e8 || tool_ == "select") return -1;
    QPointF o = to_screen(c);
    QPointF d = pos - o;
    if (tool_ == "rotate") return std::abs(std::hypot(d.x(), d.y()) - 50) < 8 || std::hypot(d.x(), d.y()) < 50 ? 3 : -1;
    if (std::abs(d.x()) < 8 && std::abs(d.y()) < 8) return 3;
    if (d.x() > 4 && d.x() < 70 && std::abs(d.y()) < 7) return 1;
    if (-d.y() > 4 && -d.y() < 70 && std::abs(d.x()) < 7) return 2;
    return -1;
}

static bool inside(const QPolygonF& poly, QPointF p) {
    return poly.containsPoint(p, Qt::OddEvenFill);
}

static double seg_dist(QPointF p, QPointF a, QPointF b) {
    QPointF ab = b - a;
    double l2 = ab.x() * ab.x() + ab.y() * ab.y();
    double t = l2 < 1e-12 ? 0 : qBound(0.0, QPointF::dotProduct(p - a, ab) / l2, 1.0);
    QPointF q = a + ab * t;
    return std::hypot(p.x() - q.x(), p.y() - q.y());
}

void UvCanvas::pick(QPointF pos, QPointF to, bool rect, Qt::KeyboardModifiers mods) {
    bool shift = mods & Qt::ShiftModifier, ctrl = mods & Qt::ControlModifier;
    QRectF r = QRectF(pos, to).normalized();
    if (mode_ == "object") {
        // Object mode picks meshes by their UV faces; a click on nothing keeps the meshes shown, as the reference's.
        QJsonArray names;
        for (const UvMesh& m : meshes_) {
            bool hit = false;
            for (const auto& fc : m.faces) {
                QPolygonF poly;
                for (int id : fc.maps) poly << to_screen(m.maps[id]);
                if (rect ? r.intersects(poly.boundingRect()) : inside(poly, pos)) {
                    hit = true;
                    break;
                }
            }
            if (hit) names.append(m.path);
        }
        if (names.isEmpty()) return;
        QJsonObject call{{"args", names}};
        if (ctrl && shift) call["add"] = true;
        else if (ctrl) call["deselect"] = true;
        else if (shift) call["toggle"] = true;
        exec("select", call);
        return;
    }
    if (mode_ == "uv" || mode_ == "shell") {
        QHash<QString, QSet<int>> hit;
        for (const UvMesh& m : meshes_) {
            if (mode_ == "uv") {
                int best = -1;
                double bd = 8;
                for (int i = 0; i < m.maps.size(); ++i) {
                    QPointF s = to_screen(m.maps[i]);
                    if (rect) {
                        if (r.contains(s)) hit[m.path].insert(i);
                    } else {
                        double d = std::hypot(s.x() - pos.x(), s.y() - pos.y());
                        if (d < bd) bd = d, best = i;
                    }
                }
                if (best >= 0) hit[m.path] = {best};
            } else {
                QSet<int> shells;
                for (const auto& fc : m.faces) {
                    QPolygonF poly;
                    for (int id : fc.maps) poly << to_screen(m.maps[id]);
                    if (rect ? r.intersects(poly.boundingRect()) : inside(poly, pos)) shells.insert(fc.shell);
                }
                for (const auto& fc : m.faces)
                    if (shells.contains(fc.shell))
                        for (int id : fc.maps) hit[m.path].insert(id);
            }
        }
        // A single click on UV takes the nearest across meshes only.
        if (!shift && !ctrl) uv_sel_.clear();
        for (auto it = hit.begin(); it != hit.end(); ++it) {
            QSet<int>& s = uv_sel_[it.key()];
            for (int id : it.value()) {
                if (ctrl) s.remove(id);
                else if (shift && !ctrl && !rect && s.contains(id)) s.remove(id);
                else s.insert(id);
            }
        }
        update();
        emit picked();
        return;
    }
    // Faces and edges are the scene's selection.
    QJsonArray names;
    for (const UvMesh& m : meshes_) {
        QList<int> ids;
        if (mode_ == "face") {
            for (const auto& fc : m.faces) {
                QPolygonF poly;
                for (int id : fc.maps) poly << to_screen(m.maps[id]);
                if (rect ? r.contains(poly.boundingRect().center()) : inside(poly, pos)) ids << fc.id;
            }
            if (!rect && ids.size() > 1) ids = {ids.last()};
        } else {
            int best = -1;
            double bd = 7;
            for (const auto& fc : m.faces)
                for (int k = 0; k < fc.edges.size(); ++k) {
                    QPointF a = to_screen(m.maps[fc.maps[k]]), b = to_screen(m.maps[fc.maps[(k + 1) % fc.maps.size()]]);
                    if (rect) {
                        if (r.contains((a + b) / 2) && !ids.contains(fc.edges[k])) ids << fc.edges[k];
                    } else {
                        double d = seg_dist(pos, a, b);
                        if (d < bd) bd = d, best = fc.edges[k];
                    }
                }
            if (best >= 0) ids = {best};
        }
        for (const QString& run : id_runs(ids)) names.append(QString("%1.%2[%3]").arg(m.path, mode_ == "face" ? "f" : "e", run));
        if (!rect && ids.size() == 1) {
            pick_hist_.append(Pick{m.path, mode_, ids.first()});
            while (pick_hist_.size() > 2) pick_hist_.removeFirst();
        }
    }
    QJsonObject call{{"args", names}};
    if (ctrl && shift) call["add"] = true;
    else if (ctrl) call["deselect"] = true;
    else if (shift) call["toggle"] = true;
    if (names.isEmpty()) {
        if (shift || ctrl) return;
        // Nothing hit: keep the objects, drop the components.
        QJsonArray objs;
        for (const UvMesh& m : meshes_) objs.append(m.path);
        exec("select", QJsonObject{{"args", objs}});
        exec("selectMode", QJsonObject{{"args", QJsonArray{mode_}}});
        return;
    }
    exec("select", call);
}

void UvCanvas::mousePressEvent(QMouseEvent* e) {
    press_ = last_ = e->position();
    bool alt = e->modifiers() & Qt::AltModifier;
    if (alt && e->button() == Qt::MiddleButton) {
        remember_view();
        drag_ = Mode::Pan;
        return;
    }
    if (alt && e->button() == Qt::RightButton) {
        remember_view();
        drag_ = Mode::Zoom;
        return;
    }
    if (e->button() == Qt::RightButton) {
        mode_menu(e->globalPosition().toPoint());
        return;
    }
    if (e->button() == Qt::LeftButton && editing_pivot) {
        pivot_uv = to_uv(e->position());
        has_pivot = true;
        editing_pivot = false;
        update();
        return;
    }
    if (e->button() == Qt::LeftButton && tool_ == "lattice") {
        if (lat_box_.isNull()) lattice_reset();
        lat_drag_ = -1;
        double best = 9;
        for (int k = 0; k < lat_pts_.size(); ++k) {
            const double d = QLineF(to_screen(lat_pts_[k]), e->position()).length();
            if (d < best) best = d, lat_drag_ = k;
        }
        if (lat_drag_ >= 0) {
            drag_ = Mode::Lattice;
            drag_targets_ = targets();
            applied_ = false;
        } else {
            drag_ = Mode::Marquee;
        }
        return;
    }
    if (e->button() == Qt::LeftButton && is_brush()) {
        // B held: drag to size the brush. Otherwise a stroke.
        if (b_held_) {
            drag_ = Mode::BrushSize;
            return;
        }
        drag_ = Mode::Brush;
        stroke_ = QJsonArray();
        const QPointF uv = to_uv(e->position());
        stroke_ << uv.x() << uv.y() << 0.0 << 0.0;
        applied_ = false;
        brush_apply(false);
        return;
    }
    if (e->button() == Qt::LeftButton && (tool_ == "lasso" || tool_ == "window" || tool_ == "tweak" || tool_ == "cut" || tool_ == "sew" || tool_ == "path" || tool_ == "smooth")) {
        tool_press(e);
        return;
    }
    if (e->button() == Qt::LeftButton || e->button() == Qt::MiddleButton) {
        int h = handle_at(e->position());
        bool mid = e->button() == Qt::MiddleButton;
        if ((h >= 0 || mid) && tool_ != "select" && !active_maps().isEmpty()) {
            drag_ = Mode::Drag;
            handle_ = mid ? (h > 0 ? h : 3) : h;
            pivot_ = selection_center();
            applied_ = false;
            drag_targets_ = targets();
            return;
        }
        if (e->button() == Qt::LeftButton) drag_ = Mode::Marquee;
    }
}

void UvCanvas::drag_to(QPointF pos, bool final) {
    if (applied_) aztek_retract();
    applied_ = false;
    QJsonObject call{{"args", drag_targets_}};
    QPointF d = to_uv(pos) - to_uv(press_);
    double px = pos.x() - press_.x();
    if (tool_ == "move" || tool_ == "tweak") {
        if (handle_ == 1) d.setY(0);
        if (handle_ == 2) d.setX(0);
        if (std::abs(d.x()) < 1e-9 && std::abs(d.y()) < 1e-9) return;
        call["u"] = d.x();
        call["v"] = d.y();
    } else if (tool_ == "rotate") {
        double a = -px * 0.5;
        if (std::abs(a) < 1e-9) return;
        call["angle"] = a;
        call["pivotU"] = pivot_.x();
        call["pivotV"] = pivot_.y();
    } else if (tool_ == "scale") {
        double s = std::exp(px * 0.01);
        call["scaleU"] = handle_ == 2 ? 1.0 : s;
        call["scaleV"] = handle_ == 1 ? 1.0 : s;
        call["pivotU"] = pivot_.x();
        call["pivotV"] = pivot_.y();
    }
    QJsonValue r = final ? exec("polyEditUV", call) : exec_quiet("polyEditUV", call);
    applied_ = !final && !r.isUndefined();
    if (!final) {
        // Show the preview now rather than at the next poll.
        Host::get()->poke();
    }
}

// The stroke so far run as one brush command: the last preview taken back first, so a stroke is one undo.
void UvCanvas::brush_apply(bool final) {
    if (applied_) aztek_retract();
    applied_ = false;
    QJsonArray meshes;
    for (const UvMesh& m : meshes_) meshes.append(m.path);
    if (meshes.isEmpty() || stroke_.isEmpty()) return;
    const QJsonObject call{{"args", meshes}, {"mode", tool_.mid(6)}, {"radius", brush_px_ / scale_}, {"samples", stroke_}};
    QJsonValue r = final ? exec("texBrushUVs", call) : exec_quiet("texBrushUVs", call);
    applied_ = !final && !r.isUndefined();
    Host::get()->poke();
}

void UvCanvas::keyReleaseEvent(QKeyEvent* e) {
    if (e->key() == Qt::Key_B && !e->isAutoRepeat()) b_held_ = false;
    QWidget::keyReleaseEvent(e);
}

void UvCanvas::mouseMoveEvent(QMouseEvent* e) {
    QPointF pos = e->position();
    QPointF d = pos - last_;
    switch (drag_) {
    case Mode::Pan:
        center_ -= QPointF(d.x() / scale_, -d.y() / scale_);
        update();
        break;
    case Mode::Zoom: {
        double f = std::exp((d.x() - d.y()) * 0.01);
        scale_ = qBound(5.0, scale_ * f, 1e6);
        update();
        break;
    }
    case Mode::Marquee: update(); break;
    case Mode::Lasso:
        if (lasso_.isEmpty() || QLineF(lasso_.last(), pos).length() > 2) lasso_ << pos;
        update();
        break;
    case Mode::Drag: drag_to(pos, false); break;
    case Mode::Lattice:
        if (lat_drag_ >= 0 && lat_drag_ < lat_pts_.size()) {
            lat_pts_[lat_drag_] = to_uv(pos);
            lattice_apply(false);
            update();
        }
        break;
    case Mode::BrushSize:
        brush_px_ = qBound(4.0, brush_px_ + d.x(), 600.0);
        update();
        break;
    case Mode::Brush: {
        if (QLineF(last_, pos).length() < 2) return;
        const QPointF uv = to_uv(pos), was = to_uv(last_);
        stroke_ << uv.x() << uv.y() << uv.x() - was.x() << uv.y() - was.y();
        brush_apply(false);
        break;
    }
    default: break;
    }
    if (is_brush()) {
        hover_ = pos;
        update();
    }
    last_ = pos;
}

void UvCanvas::mouseReleaseEvent(QMouseEvent* e) {
    if (drag_ == Mode::Lasso) {
        pick_lasso(e->modifiers());
    } else if (drag_ == Mode::Marquee) {
        bool rect = QLineF(press_, e->position()).length() > 4;
        pick(press_, e->position(), rect, e->modifiers());
        if (tool_ == "lattice") lattice_reset();
    } else if (drag_ == Mode::Lattice) {
        lattice_apply(true);
        lat_drag_ = -1;
        // The next drag starts from a fresh lattice over where the UVs are now.
        QTimer::singleShot(150, this, [this] { lattice_reset(); update(); });
    } else if (drag_ == Mode::Brush) {
        brush_apply(true);
        stroke_ = QJsonArray();
    } else if (drag_ == Mode::Drag) {
        if (applied_) {
            aztek_retract();
            applied_ = false;
            drag_to(e->position(), true);
            // Pixel Snap: the moved UVs onto the image's pixel corners.
            if (pixel_snap && !image_.isNull() && !drag_targets_.isEmpty())
                exec("texMatchGrid", QJsonObject{{"args", drag_targets_}, {"gridU", image_.width()}, {"gridV", image_.height()}});
        }
    }
    drag_ = Mode::None;
    update();
}

void UvCanvas::wheelEvent(QWheelEvent* e) {
    remember_view();
    QPointF at = e->position();
    QPointF before = to_uv(at);
    scale_ = qBound(5.0, scale_ * std::exp(e->angleDelta().y() / 120.0 * 0.15), 1e6);
    QPointF after = to_uv(at);
    center_ += before - after;
    update();
}

// Double-click: the edge loop through the edge, as in the views. Shift+double-click: the loop or the shorter way
// between the last two edges or faces clicked on one mesh.
void UvCanvas::mouseDoubleClickEvent(QMouseEvent* e) {
    if (e->button() != Qt::LeftButton || (e->modifiers() & Qt::AltModifier)) return QWidget::mouseDoubleClickEvent(e);
    if (mode_ != "edge" && mode_ != "face") return;
    if ((e->modifiers() & Qt::ShiftModifier) && pick_hist_.size() == 2) {
        const Pick &a = pick_hist_[0], &b = pick_hist_[1];
        if (a.mesh == b.mesh && a.kind == b.kind && a.id != b.id) {
            exec("selectPathBetween", QJsonObject{{"args", QJsonArray{a.mesh}}, {"from", a.id}, {"to", b.id}, {"kind", a.kind}});
            return;
        }
    }
    if (mode_ == "edge") exec("selectLoop");
}

void UvCanvas::keyPressEvent(QKeyEvent* e) {
    // Undo and redo here too: the editor is a window of its own, out of the main window's shortcuts' reach.
    const bool ctrl = e->modifiers() & Qt::ControlModifier, shift = e->modifiers() & Qt::ShiftModifier;
    if ((e->key() == Qt::Key_Z && ctrl && shift) || (e->key() == Qt::Key_Y && ctrl)) {
        exec("redo");
        return;
    }
    if (e->key() == Qt::Key_Z && (ctrl || e->modifiers() == Qt::NoModifier)) {
        exec("undo");
        return;
    }
    if (e->key() == Qt::Key_B && !e->isAutoRepeat()) {
        b_held_ = true;
        return;
    }
    if (e->key() == Qt::Key_Insert) {
        // Edit Pivot on and off, as the reference's Insert.
        editing_pivot = !editing_pivot;
        return;
    }
    switch (e->key()) {
    case Qt::Key_4: shaded = false; update(); return;
    case Qt::Key_5: shaded = true; update(); return;
    case Qt::Key_6: show_image = !show_image; update(); return;
    case Qt::Key_7: show_distortion = !show_distortion; update(); return;
    case Qt::Key_8: show_shell_borders = !show_shell_borders; update(); return;
    case Qt::Key_9: show_texture_borders = !show_texture_borders; update(); return;
    case Qt::Key_F: frame(true); return;
    case Qt::Key_A: frame(false); return;
    case Qt::Key_Q: set_tool("select"); return;
    case Qt::Key_W: set_tool("move"); return;
    case Qt::Key_E: set_tool("rotate"); return;
    case Qt::Key_R: set_tool("scale"); return;
    case Qt::Key_F8: set_mode(mode_ == "object" ? "uv" : "object"); return;
    case Qt::Key_F10: set_mode("edge"); return;
    case Qt::Key_F11: set_mode("face"); return;
    case Qt::Key_F12: set_mode(e->modifiers() & Qt::ShiftModifier ? "shell" : "uv"); return;
    default: QWidget::keyPressEvent(e);
    }
}

void UvCanvas::contextMenuEvent(QContextMenuEvent* e) {
    // From the mouse it came up on the press already (mode_menu); this is the keyboard's.
    if (e->reason() == QContextMenuEvent::Mouse || (e->modifiers() & Qt::AltModifier)) return;
    mode_menu(e->globalPos());
}

// The right button's menu, up as the button goes down, so a press-drag-release picks (#99): Object Mode with the
// component types, as the reference's.
void UvCanvas::mode_menu(QPoint global) {
    QMenu m(this);
    for (auto [label, mode] : {std::pair{"Object Mode", "object"}, {"UV", "uv"}, {"UV Shell", "shell"}, {"Edge", "edge"}, {"Face", "face"}}) {
        QAction* a = m.addAction(label, [this, mode = QString(mode)] { set_mode(mode); });
        a->setCheckable(true);
        a->setChecked(mode_ == mode);
    }
    m.exec(global);
}

// ---------------------------------------------------------------------------------------------------------------------
// Editor

// Every UV Editor there is, and the tool they are on.
static QList<UvEditor*>& uv_editors() {
    static QList<UvEditor*> all;
    return all;
}
static QString g_uv_tool = "move";

void UvEditor::set_tool_everywhere(const QString& t) {
    g_uv_tool = t;
    for (UvEditor* e : uv_editors()) e->show_tool(t);
}

void UvEditor::show_tool(const QString& t) {
    canvas_->set_tool(t);
    QString shown = t == "shell" ? "move" : t;
    if (QAction* a = tool_buttons_.value(shown)) {
        a->setChecked(true);
    } else {
        // A tool with no button: none of the four stays lit.
        for (QAction* b : tool_buttons_) {
            if (b->actionGroup()) b->actionGroup()->setExclusive(false);
            b->setChecked(false);
            if (b->actionGroup()) b->actionGroup()->setExclusive(true);
        }
    }
}

void UvEditor::apply_flags(const QJsonObject& p) {
    if (p.contains("displayCheckered")) {
        canvas_->show_checker = p["displayCheckered"].toBool();
        canvas_->update();
        exec_quiet("uiPref", QJsonObject{{"args", QJsonArray{"uvChecker", canvas_->show_checker}}});
        Host::get()->poke();
        if (auto* a = findChild<QAction*>("uvCheckerAction")) a->setChecked(canvas_->show_checker);
    }
    if (p.contains("displayUVShaded")) {
        canvas_->shaded = p["displayUVShaded"].toBool();
        canvas_->update();
    }
    if (p["frameAll"].toBool()) canvas_->frame(false);
    if (p["frameSelected"].toBool()) canvas_->frame(true);
}

UvEditor::UvEditor(QWidget* parent) : QWidget(parent) {
    uv_editors().append(this);
    connect(this, &QObject::destroyed, [this] { uv_editors().removeOne(this); });
    // Made after a context chose a tool: on that tool.
    QTimer::singleShot(0, this, [this] {
        if (g_uv_tool != "move") show_tool(g_uv_tool);
    });
    setWindowTitle("UV Editor");
    auto* v = new QVBoxLayout(this);
    v->setContentsMargins(0, 0, 0, 0);
    v->setSpacing(0);
    canvas_ = new UvCanvas(this);
    connect(canvas_, &UvCanvas::picked, canvas_, &UvCanvas::share_uv_selection);
    auto* bar = new QToolBar(this);
    auto* modes = new QActionGroup(this);
    for (auto [label, mode] : {std::pair{"Object", "object"}, {"UV", "uv"}, {"Shell", "shell"}, {"Edge", "edge"}, {"Face", "face"}}) {
        QAction* a = bar->addAction(label, [this, mode = QString(mode)] { canvas_->set_mode(mode); });
        a->setCheckable(true);
        a->setChecked(QString(mode) == "object");
        modes->addAction(a);
        mode_buttons_[mode] = a;
    }
    bar->addSeparator();
    auto* tools = new QActionGroup(this);
    for (auto [label, tool] : {std::pair{"select", "select"}, {"move", "move"}, {"rotate", "rotate"}, {"scale", "scale"}}) {
        QAction* a = bar->addAction(icon(label), QString(label), [this, tool = QString(tool)] { canvas_->set_tool(tool); });
        a->setCheckable(true);
        a->setChecked(QString(tool) == "move");
        tools->addAction(a);
        tool_buttons_.insert(tool, a);
    }
    bar->addSeparator();
    bar->addAction("Planar", [this] { run("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}}); });
    bar->addAction("Automatic", [this] { run("polyAutoProjection"); });
    bar->addAction("Cylindrical", [this] { run("polyProjection", {{"type", "cylindrical"}}); });
    bar->addAction("Spherical", [this] { run("polyProjection", {{"type", "spherical"}}); });
    bar->addSeparator();
    bar->addAction("Unfold", [this] { run("u3dUnfold"); });
    bar->addAction("Layout", [this] { run("u3dLayout"); });
    bar->addAction("Cut", [this] { run("polyMapCut"); });
    bar->addAction("Split", [] { mel_action("SplitUV;"); });
    bar->addAction("Sew", [this] { run("polyMapSew"); });
    bar->addAction("Move and Sew", [this] { run("polyMapSewMove"); });
    bar->addSeparator();
    bar->addAction("Normalize", [this] { run("polyNormalizeUV", {{"normalizeType", 1}}); });
    bar->addAction("Flip U", [this] { run("polyFlipUV", {{"flipType", 0}}); });
    bar->addAction("Flip V", [this] { run("polyFlipUV", {{"flipType", 1}}); });
    bar->addAction("-90", [this] { run("polyEditUV", {{"angle", -90.0}}); });
    bar->addAction("+90", [this] { run("polyEditUV", {{"angle", 90.0}}); });
    bar->addSeparator();
    // Texel density: Get reads the selection's into the field, Set scales each shell to it, for the map size chosen.
    auto* td = new QDoubleSpinBox(bar);
    td->setRange(0.001, 100000.0);
    td->setDecimals(3);
    td->setValue(10.24);
    td->setToolTip("Texel density: pixels per scene unit");
    auto* map = new QComboBox(bar);
    for (int px : {256, 512, 1024, 2048, 4096, 8192}) map->addItem(QString::number(px), px);
    map->setCurrentIndex(2);
    map->setToolTip("Map size, pixels");
    bar->addWidget(new QLabel(" Texel Density ", bar));
    bar->addWidget(td);
    bar->addWidget(map);
    bar->addAction("Get", [this, td, map] {
        QJsonObject c{{"mapSize", map->currentData().toInt()}};
        QJsonArray t = canvas_->targets();
        if (!t.isEmpty()) c["args"] = t;
        QJsonValue v = ask("texGetTexelDensity", c);
        if (v.isDouble()) td->setValue(v.toDouble());
    });
    bar->addAction("Set", [this, td, map] { run("texSetTexelDensity", {{"texelDensity", td->value()}, {"mapSize", map->currentData().toInt()}}); });
    bar->addSeparator();
    QAction* display_image = bar->addAction("Image", [this](bool on) {
        canvas_->show_image = on;
        canvas_->update();
    });
    display_image->setCheckable(true);
    display_image->setChecked(true);
    display_image->setToolTip("Display Image: the material's texture behind the UVs");
    // The checker shows behind the UVs here and on the meshes in the views, as the reference's does.
    QAction* checker = bar->addAction("Checker", [this](bool on) {
        canvas_->show_checker = on;
        canvas_->update();
        exec_quiet("uiPref", QJsonObject{{"args", QJsonArray{"uvChecker", on}}});
        Host::get()->poke();
    });
    checker->setCheckable(true);
    checker->setObjectName("uvCheckerAction");
    checker->setToolTip("Checker Map: a checker on the selected meshes, here and in the views");
    QAction* shade = bar->addAction("Shaded", [this](bool on) {
        canvas_->shaded = on;
        canvas_->update();
    });
    shade->setCheckable(true);
    shade->setChecked(true);
    // The reference's display bar: what draws, the image's channels and strength, exposure and gamma.
    auto* show_bar = new QToolBar(this);
    auto flag = [this, show_bar](const QString& label, const QString& tip, bool* f) {
        QAction* a = show_bar->addAction(label, [this, f](bool on) {
            *f = on;
            canvas_->update();
        });
        a->setCheckable(true);
        a->setChecked(*f);
        a->setToolTip(tip);
        return a;
    };
    flag("Distortion", "UV Distortion (7): red where the texture stretches, blue where it squeezes", &canvas_->show_distortion);
    flag("Shell Borders", "Toggle Shell Borders (8): each shell's border in its own colour", &canvas_->show_shell_borders);
    flag("Borders", "Texture Borders (9)", &canvas_->show_texture_borders);
    flag("Grid", "Grid", &canvas_->show_grid);
    flag("Pixel Snap", "Pixel Snap: moved UVs land on the image's pixel corners", &canvas_->pixel_snap);
    show_bar->addSeparator();
    QAction* alpha = show_bar->addAction("A", [this](bool on) {
        canvas_->image_channels = on ? 2 : 1;
        canvas_->update();
    });
    alpha->setCheckable(true);
    alpha->setToolTip("The image's alpha channel");
    auto* dim = new QSlider(Qt::Horizontal, show_bar);
    dim->setRange(0, 100);
    dim->setValue(100);
    dim->setMaximumWidth(90);
    dim->setToolTip("How strongly the image shows");
    connect(dim, &QSlider::valueChanged, this, [this](int v) {
        canvas_->image_opacity = v / 100.0;
        canvas_->update();
    });
    show_bar->addWidget(dim);
    show_bar->addSeparator();
    auto* exposure = new QDoubleSpinBox(show_bar);
    exposure->setRange(-20, 20);
    exposure->setDecimals(2);
    exposure->setSingleStep(0.1);
    exposure->setToolTip("Exposure, in stops");
    connect(exposure, &QDoubleSpinBox::valueChanged, this, [this](double v) {
        canvas_->image_exposure = v;
        canvas_->update();
    });
    auto* gamma = new QDoubleSpinBox(show_bar);
    gamma->setRange(0.01, 10);
    gamma->setDecimals(2);
    gamma->setSingleStep(0.1);
    gamma->setValue(1.0);
    gamma->setToolTip("Gamma");
    connect(gamma, &QDoubleSpinBox::valueChanged, this, [this](double v) {
        canvas_->image_gamma = v;
        canvas_->update();
    });
    show_bar->addWidget(new QLabel(" Exposure ", show_bar));
    show_bar->addWidget(exposure);
    show_bar->addWidget(new QLabel(" Gamma ", show_bar));
    show_bar->addWidget(gamma);
    // The UV set shown: the current one of the mesh worked on. Picking another makes that one current.
    // On a bar of its own, so it is there however narrow the window is.
    auto* tools_bar = bar;
    bar = new QToolBar(this);
    bar->addWidget(new QLabel(" UV Set: ", bar));
    uv_set_ = new QComboBox(bar);
    uv_set_->setObjectName("uvSetChoice");
    uv_set_->setMinimumContentsLength(10);
    uv_set_->setToolTip("The UV set shown and edited. Windows > Modeling Editors has the UV Set Editor.");
    connect(uv_set_, &QComboBox::activated, this, [this](int) {
        if (uv_mesh_.isEmpty()) return;
        exec("polyUVSet", QJsonObject{{"args", QJsonArray{uv_mesh_}}, {"currentUVSet", true}, {"uvSet", uv_set_->currentText()}});
        Host::get()->poke();
    });
    bar->addWidget(uv_set_);
    bar->addSeparator();
    bar->addWidget(show_bar);
    toolkit_ = build_toolkit();
    v->addWidget(build_menu_bar());
    v->addWidget(tools_bar);
    v->addWidget(bar);
    // The canvas, and the UV Toolkit beside it (#108).
    auto* split = new QSplitter(Qt::Horizontal, this);
    split->addWidget(canvas_);
    split->addWidget(toolkit_);
    split->setStretchFactor(0, 1);
    split->setStretchFactor(1, 0);
    v->addWidget(split, 1);
    connect(Host::get(), &Host::sceneChanged, this, [this] {
        if (isVisible()) refresh();
    });
    resize(1060, 760);
}

void UvEditor::run(const QString& cmd, const QJsonObject& call, bool on_selection) {
    QJsonObject c = call;
    QJsonArray t = canvas_->targets();
    if (on_selection && !t.isEmpty()) c["args"] = t;
    if (cmd == "polyEditUV" && c.contains("angle")) {
        // Turns go about the middle of what turns.
        QJsonArray a{c["angle"]};
        for (const QJsonValue& x : t) a.append(x);
        exec("polyRotateUVs", QJsonObject{{"args", a}});
        return;
    }
    exec(cmd, c);
}

void UvEditor::refresh() {
    const QJsonObject data = query("uvData").toObject();
    canvas_->set_data(data);
    // The selection type is shared with the views: a view switched to UVs, UV shells, edges or faces brings the
    // editor along.
    const QString scene = data["uvShell"].toBool() ? "shell" : data["mode"].toString();
    if (scene == "object" || scene == "uv" || scene == "shell" || scene == "edge" || scene == "face") {
        if (QAction* a = mode_buttons_.value(scene); a && !a->isChecked()) a->setChecked(true);
        if (scene != canvas_->mode()) canvas_->follow_mode(scene);
    }
    if (uv_set_ && !uv_set_->view()->isVisible()) {
        const QJsonArray sel = state()["selection"].toArray();
        uv_mesh_ = sel.isEmpty() ? QString() : sel.last().toString().section('.', 0, 0);
        QStringList names;
        QString cur;
        if (!uv_mesh_.isEmpty()) {
            for (const QJsonValue& x : ask("polyUVSet", QJsonObject{{"args", QJsonArray{uv_mesh_}}, {"query", true}, {"queried", QJsonArray{"allUVSets"}}}).toArray()) names << x.toString();
            const QJsonValue c = ask("polyUVSet", QJsonObject{{"args", QJsonArray{uv_mesh_}}, {"query", true}, {"queried", QJsonArray{"currentUVSet"}}});
            cur = c.isArray() ? c.toArray().at(0).toString() : c.toString();
        }
        QStringList have;
        for (int i = 0; i < uv_set_->count(); ++i) have << uv_set_->itemText(i);
        if (have != names) {
            uv_set_->clear();
            uv_set_->addItems(names);
        }
        uv_set_->setEnabled(!names.isEmpty());
        if (uv_set_->currentText() != cur) uv_set_->setCurrentText(cur);
    }
    QJsonObject t = query("uvTexture").toObject();
    QJsonArray r = t["repeat"].toArray(), o = t["offset"].toArray();
    canvas_->set_image(t["path"].toString(), r.isEmpty() ? QPointF(1, 1) : QPointF(r.at(0).toDouble(), r.at(1).toDouble()), o.isEmpty() ? QPointF(0, 0) : QPointF(o.at(0).toDouble(), o.at(1).toDouble()));
    canvas_->update();
}

} // namespace az

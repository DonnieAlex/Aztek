// The UV Editor's UV Toolkit (#108), the side panel as the reference has it, built in batches as the issue asks. This
// one: Selection, Pinning, Select By Type, Soft Selection, Transform, Texel Density, Create, Cut and Sew, Unfold, and
// Arrange and Layout with what Aztek has commands for. The shell alignment, stacking and snapping tools come next.

#include "uveditor.h"
#include "host.h"
#include "icons.h"
#include "toolsettings.h"

#include <QButtonGroup>
#include <QCheckBox>
#include <QComboBox>
#include <QDoubleSpinBox>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QLabel>
#include <QMouseEvent>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QToolButton>
#include <QVBoxLayout>

namespace az {

namespace {

QWidget* fold(QVBoxLayout* page, const QString& title, QWidget* parent) {
    auto* head = new QToolButton(parent);
    head->setText(title);
    head->setCheckable(true);
    head->setChecked(true);
    head->setArrowType(Qt::DownArrow);
    head->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
    head->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    head->setObjectName("aeSection");
    auto* body = new QWidget(parent);
    QObject::connect(head, &QToolButton::toggled, body, [head, body](bool open) {
        body->setVisible(open);
        head->setArrowType(open ? Qt::DownArrow : Qt::RightArrow);
    });
    page->addWidget(head);
    page->addWidget(body);
    return body;
}

QDoubleSpinBox* number(double v, double lo, double hi, QWidget* parent) {
    auto* s = new QDoubleSpinBox(parent);
    s->setRange(lo, hi);
    s->setDecimals(4);
    s->setValue(v);
    s->setButtonSymbols(QAbstractSpinBox::NoButtons);
    return s;
}

} // namespace

QWidget* UvEditor::build_toolkit() {
    auto* scroll = new QScrollArea(this);
    scroll->setObjectName("uvToolkit");
    scroll->setWidgetResizable(true);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    scroll->setMinimumWidth(270);
    auto* w = new QWidget(scroll);
    auto* page = new QVBoxLayout(w);
    page->setContentsMargins(4, 2, 4, 4);
    page->setSpacing(3);

    // Two-to-a-row buttons, each running a command on what is selected here.
    auto buttons = [this](QWidget* body, QList<std::pair<QString, std::function<void()>>> items) {
        auto* g = new QGridLayout(body);
        g->setContentsMargins(0, 2, 0, 4);
        g->setSpacing(2);
        int i = 0;
        for (auto& [label, fn] : items) {
            auto* b = new QPushButton(label, body);
            b->setStyleSheet("QPushButton { text-align: left; padding: 3px 6px; }");
            connect(b, &QPushButton::clicked, this, fn);
            g->addWidget(b, i / 2, i % 2);
            ++i;
        }
        return g;
    };
    auto cmd = [this](const QString& c, const QJsonObject& call = QJsonObject()) {
        return [this, c, call] {
            run(c, call);
            Host::get()->poke();
        };
    };
    auto select_faces = [this](const QString& c, const QJsonObject& flags) {
        return [this, c, flags] {
            QJsonObject call = flags;
            QJsonArray meshes;
            for (const QString& m : canvas_->mesh_paths()) meshes.append(m);
            if (meshes.isEmpty()) return;
            call["args"] = meshes;
            const QJsonValue faces = ask(c, call);
            if (faces.toArray().isEmpty()) exec("select", QJsonObject{{"args", meshes}});
            else exec("select", QJsonObject{{"args", faces}});
            canvas_->set_mode("face");
            Host::get()->poke();
        };
    };

    // Selection: the types, how a click selects, all, none and the rest.
    {
        QWidget* body = fold(page, "Selection", w);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 2, 0, 4);
        auto* types = new QHBoxLayout();
        for (auto [ic, mode, tip] : {std::tuple{"edge", "edge", "Edge selection"}, {"face", "face", "Face selection"}, {"uv", "uv", "UV selection"},
                                     {"uvShellSel", "shell", "UV Shell selection"}, {"object", "object", "Object selection"}}) {
            auto* b = new QToolButton(body);
            b->setIcon(icon(ic));
            b->setIconSize(QSize(24, 24));
            b->setAutoRaise(true);
            b->setToolTip(tip);
            const QString m = mode;
            connect(b, &QToolButton::clicked, this, [this, m] { canvas_->set_mode(m); });
            types->addWidget(b);
        }
        types->addStretch(1);
        v->addLayout(types);
        auto* how = new QButtonGroup(body);
        int i = 0;
        for (auto [label, tool] : {std::pair{"Pick/Marquee", "move"}, {"Drag", "lasso"}, {"Tweak/Marquee", "tweak"}}) {
            auto* r = new QRadioButton(label, body);
            r->setChecked(i++ == 0);
            how->addButton(r);
            const QString t = tool;
            connect(r, &QRadioButton::clicked, this, [this, t] { canvas_->set_tool(t); });
            v->addWidget(r);
        }
        auto* row = new QHBoxLayout();
        auto* all = new QPushButton("All", body);
        auto* clear = new QPushButton("Clear", body);
        auto* inverse = new QPushButton("Inverse", body);
        connect(all, &QPushButton::clicked, this, [this] {
            canvas_->select_all_uvs(false);
        });
        connect(clear, &QPushButton::clicked, this, [this] {
            QJsonArray meshes;
            for (const QString& m : canvas_->mesh_paths()) meshes.append(m);
            exec("select", meshes.isEmpty() ? QJsonObject{{"clear", true}} : QJsonObject{{"args", meshes}});
            Host::get()->poke();
        });
        connect(inverse, &QPushButton::clicked, this, [this] { canvas_->select_all_uvs(true); });
        row->addWidget(all);
        row->addWidget(clear);
        row->addWidget(inverse);
        v->addLayout(row);
    }

    // Pinning: pinned UVs stay where they are when unfolded.
    buttons(fold(page, "Pinning", w), {{"Pin", cmd("polyPinUV", {{"value", 1.0}})},
                                       {"Unpin", cmd("polyPinUV", {{"value", 0.0}})},
                                       {"Invert Pinning", cmd("polyPinUV", {{"value", 1.0}, {"unpinned", true}})},
                                       {"Unpin All", [this] {
                                            QJsonArray meshes;
                                            for (const QString& m : canvas_->mesh_paths()) meshes.append(m);
                                            if (meshes.isEmpty()) return;
                                            exec("polyPinUV", QJsonObject{{"args", meshes}, {"operation", 2}});
                                            Host::get()->poke();
                                        }}});

    // Select By Type.
    buttons(fold(page, "Select By Type", w), {{"Overlapping", select_faces("polyUVOverlap", {{"overlappingComponents", true}})},
                                              {"Non-Overlapping", select_faces("polyUVOverlap", {{"nonOverlappingComponents", true}})}});

    // Soft Selection, the same setting as the views'.
    {
        QWidget* body = fold(page, "Soft Selection", w);
        auto* f = new QFormLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        auto* soft = new QCheckBox(body);
        soft->setChecked(ask("softSelect", QJsonObject{{"query", true}, {"softSelectEnabled", true}}).toInt() != 0);
        connect(soft, &QCheckBox::toggled, this, [](bool on) { exec("softSelect", QJsonObject{{"softSelectEnabled", on ? 1 : 0}}); });
        f->addRow("Soft Select:", soft);
        auto* row = new QHBoxLayout();
        auto* mode = new QComboBox(body);
        mode->addItems({"Volume", "Surface", "Global", "Object"});
        connect(mode, &QComboBox::activated, this, [](int i) { exec("softSelect", QJsonObject{{"softSelectFalloff", i}}); });
        auto* radius = number(5.0, 0.0, 1000.0, body);
        connect(radius, &QDoubleSpinBox::editingFinished, this, [radius] { exec_quiet("softSelect", QJsonObject{{"softSelectDistance", radius->value()}}); });
        row->addWidget(mode, 1);
        row->addWidget(radius);
        f->addRow(row);
        std::function<void()> reload;
        f->addRow(falloff_curve(body, &reload));
        auto* reset = new QPushButton("Reset Curve", body);
        connect(reset, &QPushButton::clicked, this, [reload] {
            exec_quiet("softSelectCurve", QJsonObject{{"preset", "Smooth"}});
            if (reload) reload();
        });
        f->addRow(reset);
    }

    // Transform: move by U and V, turn, scale, flip.
    {
        QWidget* body = fold(page, "Transform", w);
        auto* g = new QGridLayout(body);
        g->setContentsMargins(0, 2, 0, 4);
        g->setSpacing(3);
        auto* mu = number(0.0, -1000, 1000, body);
        auto* mv = number(0.0, -1000, 1000, body);
        auto* step = number(0.1, 0, 1000, body);
        g->addWidget(new QLabel("Move U", body), 0, 0);
        g->addWidget(mu, 0, 1);
        g->addWidget(new QLabel("V", body), 0, 2);
        g->addWidget(mv, 0, 3);
        auto* go = new QPushButton("Move", body);
        connect(go, &QPushButton::clicked, this, [this, mu, mv] {
            run("polyEditUV", {{"u", mu->value()}, {"v", mv->value()}});
            Host::get()->poke();
        });
        g->addWidget(go, 0, 4);
        // Nudges: a step along U or V.
        auto* nudges = new QHBoxLayout();
        for (auto [label, du, dv] : {std::tuple{"← U", -1, 0}, {"U →", 1, 0}, {"↓ V", 0, -1}, {"V ↑", 0, 1}}) {
            auto* b = new QPushButton(label, body);
            const int u = du, vv = dv;
            connect(b, &QPushButton::clicked, this, [this, step, u, vv] {
                run("polyEditUV", {{"u", u * step->value()}, {"v", vv * step->value()}});
                Host::get()->poke();
            });
            nudges->addWidget(b);
        }
        nudges->addWidget(new QLabel("by", body));
        nudges->addWidget(step);
        g->addLayout(nudges, 1, 0, 1, 5);
        auto* angle = number(90.0, -360, 360, body);
        g->addWidget(new QLabel("Rotate", body), 2, 0);
        g->addWidget(angle, 2, 1);
        auto* ccw = new QPushButton("↺", body);
        auto* cw = new QPushButton("↻", body);
        ccw->setToolTip("Turn counter-clockwise");
        cw->setToolTip("Turn clockwise");
        connect(ccw, &QPushButton::clicked, this, [this, angle] {
            run("polyEditUV", {{"angle", angle->value()}});
            Host::get()->poke();
        });
        connect(cw, &QPushButton::clicked, this, [this, angle] {
            run("polyEditUV", {{"angle", -angle->value()}});
            Host::get()->poke();
        });
        g->addWidget(ccw, 2, 2);
        g->addWidget(cw, 2, 3);
        auto* factor = number(2.0, 0.0001, 1000, body);
        auto* su = new QCheckBox("U", body);
        auto* sv = new QCheckBox("V", body);
        su->setChecked(true);
        sv->setChecked(true);
        g->addWidget(new QLabel("Scale", body), 3, 0);
        g->addWidget(factor, 3, 1);
        g->addWidget(su, 3, 2);
        g->addWidget(sv, 3, 3);
        auto* scale = new QPushButton("Scale", body);
        connect(scale, &QPushButton::clicked, this, [this, factor, su, sv] {
            const QPointF c = canvas_->selection_center();
            if (c.x() < -1e8) return;
            run("polyEditUV", {{"scaleU", su->isChecked() ? factor->value() : 1.0}, {"scaleV", sv->isChecked() ? factor->value() : 1.0}, {"pivotU", c.x()}, {"pivotV", c.y()}});
            Host::get()->poke();
        });
        g->addWidget(scale, 3, 4);
        auto* flip_u = new QPushButton("Flip U", body);
        auto* flip_v = new QPushButton("Flip V", body);
        connect(flip_u, &QPushButton::clicked, this, cmd("polyFlipUV", {{"flipType", 0}}));
        connect(flip_v, &QPushButton::clicked, this, cmd("polyFlipUV", {{"flipType", 1}}));
        g->addWidget(flip_u, 4, 0, 1, 2);
        g->addWidget(flip_v, 4, 2, 1, 2);
    }

    // Texel Density: Get reads the selection's, Set scales each shell to it.
    {
        QWidget* body = fold(page, "Texel Density", w);
        auto* g = new QGridLayout(body);
        g->setContentsMargins(0, 2, 0, 4);
        auto* td = number(32.0, 0.001, 100000, body);
        auto* map = new QComboBox(body);
        for (int px : {256, 512, 1024, 2048, 4096, 8192}) map->addItem(QString::number(px), px);
        map->setCurrentIndex(1);
        auto* get = new QPushButton("Get", body);
        auto* set = new QPushButton("Set", body);
        connect(get, &QPushButton::clicked, this, [this, td, map] {
            QJsonObject c{{"mapSize", map->currentData().toInt()}};
            QJsonArray t = canvas_->targets();
            if (!t.isEmpty()) c["args"] = t;
            QJsonValue v = ask("texGetTexelDensity", c);
            if (v.isDouble()) td->setValue(v.toDouble());
        });
        connect(set, &QPushButton::clicked, this, [this, td, map] {
            run("texSetTexelDensity", {{"texelDensity", td->value()}, {"mapSize", map->currentData().toInt()}});
            Host::get()->poke();
        });
        g->addWidget(get, 0, 0);
        g->addWidget(td, 0, 1);
        g->addWidget(set, 0, 2);
        g->addWidget(new QLabel("Map Size:", body), 1, 0);
        g->addWidget(map, 1, 1, 1, 2);
    }

    buttons(fold(page, "Create", w), {{"Automatic", cmd("polyAutoProjection")},
                                      {"Planar", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}})},
                                      {"Cylindrical", cmd("polyProjection", {{"type", "cylindrical"}})},
                                      {"Spherical", cmd("polyProjection", {{"type", "spherical"}})},
                                      {"Contour Stretch", cmd("polyContourProjection")}});

    buttons(fold(page, "Cut and Sew", w), {{"Auto-Seams", cmd("u3dAutoSeam", {{"splitShells", 0.5}})},
                                           {"Cut", cmd("polyMapCut")},
                                           {"Cut Tool", [this] { canvas_->set_tool("cut"); }},
                                           {"Split", [] { mel_action("SplitUV;"); }},
                                           {"Sew", cmd("polyMapSew")},
                                           {"Stitch Together", cmd("polyMapSewMove")}});

    buttons(fold(page, "Unfold", w), {{"Optimize", cmd("u3dOptimize")},
                                      {"Unfold", cmd("u3dUnfold")},
                                      {"Straighten Border", cmd("polyStraightenUVBorder")},
                                      {"Rectangle", cmd("polyUVRectangle")}});

    buttons(fold(page, "Arrange and Layout", w), {{"Layout", cmd("u3dLayout")},
                                                  {"Normalize", cmd("polyNormalizeUV", {{"normalizeType", 1}})},
                                                  {"Stack Similar", cmd("polyUVStackSimilarShells")}});

    page->addStretch(1);
    scroll->setWidget(w);
    return scroll;
}

} // namespace az

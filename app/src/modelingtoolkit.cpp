// The Modeling Toolkit (#107), as the reference's: a panel docked beside the Channel Box and Attribute Editor with
// everything a modeller reaches for in one place. Multi-Component and the five selection types at the top (Ctrl
// converts the selection, Ctrl+Shift to the perimeter, the right button clears it), how a click selects, the
// selection settings, Soft Selection with its falloff curve, then the Mesh, Components and Tools buttons.

#include "mainwindow.h"
#include "host.h"
#include "icons.h"
#include "panels.h"
#include "toolsettings.h"
#include "viewport.h"

#include <QButtonGroup>
#include <QCheckBox>
#include <QComboBox>
#include <QDesktopServices>
#include <QDockWidget>
#include <QDoubleSpinBox>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonDocument>
#include <QSpinBox>
#include <QLabel>
#include <QMenuBar>
#include <QMouseEvent>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QSettings>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>

namespace az {

namespace {

// A selection type button: a click selects in that type; Ctrl+click converts the selection to it, Ctrl+Shift+click to
// its perimeter; the right button clears the selection.
class TypeButton : public QToolButton {
public:
    TypeButton(const QString& type, const QString& tip, QWidget* parent) : QToolButton(parent), type_(type) {
        setCheckable(true);
        setAutoRaise(true);
        static const QHash<QString, QString> kIcons{{"object", "mtkObject"}, {"vertex", "mtkVertex"}, {"edge", "mtkEdge"}, {"face", "mtkFace"}, {"uv", "mtkUV"}};
        setIcon(az::icon(kIcons.value(type, type)));
        setIconSize(QSize(28, 28));
        setMinimumSize(40, 40);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
        setToolTip(tip);
    }
protected:
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() == Qt::RightButton) {
            exec("select", QJsonObject{{"clear", true}});
            Host::get()->poke();
            return;
        }
        const auto mods = e->modifiers() & (Qt::ControlModifier | Qt::ShiftModifier);
        if (e->button() == Qt::LeftButton && mods & Qt::ControlModifier && type_ != "object") {
            if (mods & Qt::ShiftModifier) {
                // The perimeter: the components of this type on the border of what is selected.
                static const QHash<QString, QString> flag = {{"vertex", "-tv"}, {"edge", "-te"}, {"face", "-tf"}, {"uv", "-tuv"}};
                mel(QString("{ string $p[] = `polyListComponentConversion %1 -bo`; if (size($p)) select -r $p; selectMode %2; }").arg(flag.value(type_), type_));
            } else {
                exec("convertSelection", QJsonObject{{"args", QJsonArray{type_}}});
            }
            Host::get()->poke();
            return;
        }
        if (e->button() == Qt::LeftButton && mods & Qt::ControlModifier && type_ == "object") {
            // To objects: the meshes the components are on.
            exec("selectMode", QJsonObject{{"args", QJsonArray{"object"}}});
            Host::get()->poke();
            return;
        }
        QToolButton::mousePressEvent(e);
    }
private:
    QString type_;
};

QWidget* section(QVBoxLayout* page, const QString& title, QWidget* parent) {
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


// Quad Draw's and Multi-Cut's options as the toolkit keeps them, with the reference's defaults (#140).
const QJsonObject& option_defaults(const QString& tool) {
    static const QJsonObject quad{{"autoWeld", true}, {"autoWeldDistance", 10.0}, {"relax", 0}, {"extend", 0}, {"stripWidth", 20.0}};
    static const QJsonObject cut{{"snapStep", 10.0}, {"smoothingAngle", 180.0}, {"edgeFlow", false}, {"edgeFlowAmount", 1.0}, {"subdivisions", 1},
                                 {"ignoreBackfaces", false}, {"deleteFaces", false}, {"extractFaces", false}, {"extractOffset", QJsonArray{0.5, 0.5, 0.5}}};
    return tool == "quadDraw" ? quad : cut;
}

} // namespace

QJsonObject tool_options(const QString& tool) {
    QJsonObject o = option_defaults(tool);
    QSettings s;
    s.beginGroup("mtk/" + tool);
    for (const QString& k : s.childKeys()) {
        const QJsonDocument d = QJsonDocument::fromJson(("[" + s.value(k).toString() + "]").toUtf8());
        if (d.isArray() && !d.array().isEmpty()) o[k] = d.array().first();
    }
    return o;
}

namespace {

void set_tool_option(MainWindow* mw, const QString& tool, const QString& key, const QJsonValue& v) {
    QSettings().setValue("mtk/" + tool + "/" + key, QString::fromUtf8(QJsonDocument(QJsonArray{v}).toJson(QJsonDocument::Compact)).mid(1).chopped(1));
    mw->push_tool_options(tool);
}

// The shortcut list: each row the keys, then what they do, wrapping in the toolkit's width.
void shortcut_rows(QVBoxLayout* v, const QList<QPair<QString, QString>>& rows) {
    for (const auto& [keys, what] : rows) {
        auto* l = new QLabel("<b>" + keys + ":</b> " + what);
        l->setWordWrap(true);
        v->addWidget(l);
    }
}

// Spin boxes that give way to the toolkit's width rather than widen it.
template <typename T>
T* narrow(T* w) {
    w->setMinimumWidth(40);
    w->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Fixed);
    return w;
}

QWidget* quad_draw_panel(MainWindow* mw, QWidget* parent) {
    auto* panel = new QWidget(parent);
    auto* page = new QVBoxLayout(panel);
    page->setContentsMargins(0, 0, 0, 0);
    page->setSpacing(4);
    const QJsonObject o = tool_options("quadDraw");
    {
        QWidget* body = section(page, "Quad Draw Options", panel);
        auto* f = new QFormLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        auto* row = new QHBoxLayout();
        auto* weld = new QCheckBox("Auto-weld", body);
        weld->setToolTip("A vertex dragged this many pixels from another welds to it when let go");
        weld->setChecked(o["autoWeld"].toBool());
        auto* dist = narrow(new QSpinBox(body));
        no_stray_wheel(dist);
        dist->setRange(1, 100);
        dist->setValue(o["autoWeldDistance"].toInt());
        dist->setEnabled(weld->isChecked());
        row->addWidget(weld);
        row->addWidget(dist, 1);
        f->addRow(row);
        QObject::connect(weld, &QCheckBox::toggled, body, [mw, dist](bool on) {
            dist->setEnabled(on);
            set_tool_option(mw, "quadDraw", "autoWeld", on);
        });
        QObject::connect(dist, &QSpinBox::valueChanged, body, [mw](int v) { set_tool_option(mw, "quadDraw", "autoWeldDistance", v); });
        auto* relax = narrow(new QComboBox(body));
        no_stray_wheel(relax);
        relax->addItems({"Auto-Lock", "Interior Vertices", "Border Vertices", "All Vertices"});
        relax->setToolTip("Which points Shift+drag relaxes. Auto-Lock keeps the border still while there are inner points to move.");
        relax->setCurrentIndex(o["relax"].toInt());
        QObject::connect(relax, &QComboBox::activated, body, [mw](int i) { set_tool_option(mw, "quadDraw", "relax", i); });
        f->addRow("Relax:", relax);
        auto* extend = narrow(new QComboBox(body));
        no_stray_wheel(extend);
        extend->addItems({"Edge", "Loop", "Border"});
        extend->setToolTip("What Tab+drag on a border edge extends: the edge, its run along the border, or the whole border");
        extend->setCurrentIndex(o["extend"].toInt());
        QObject::connect(extend, &QComboBox::activated, body, [mw](int i) { set_tool_option(mw, "quadDraw", "extend", i); });
        f->addRow("Extend:", extend);
        auto* width = narrow(new QSpinBox(body));
        no_stray_wheel(width);
        width->setRange(2, 400);
        width->setSuffix(" px");
        width->setValue(o["stripWidth"].toInt());
        width->setToolTip("How wide Tab+drag draws a quad strip; Tab+middle drag changes it in the view");
        QObject::connect(width, &QSpinBox::valueChanged, body, [mw](int v) { set_tool_option(mw, "quadDraw", "stripWidth", v); });
        f->addRow("Quad Strip Width:", width);
    }
    auto* clear = new QPushButton("Clear Dots", panel);
    QObject::connect(clear, &QPushButton::clicked, panel, [mw] {
        for (ViewportWidget* v : mw->panels()->views()) v->set("quadDrawAction", "\"clearDots\"");
    });
    page->addWidget(clear);
    auto* reset = new QPushButton("Reset Settings", panel);
    QObject::connect(reset, &QPushButton::clicked, panel, [mw] {
        QSettings().remove("mtk/quadDraw");
        mw->reset_tool_panel("quadDraw");
    });
    page->addWidget(reset);
    {
        QWidget* body = section(page, "Keyboard/Mouse Shortcuts", panel);
        auto* f = new QVBoxLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        f->addWidget(new QLabel("<i>Hovering outside the quad mesh</i>"));
        shortcut_rows(f, {{"LMB", "Click to drop dots"}, {"LMB", "Drag to tweak dots"}, {"Ctrl+Shift+LMB", "Delete dots"}, {"Tab+LMB", "Drag to draw quad strips"},
                          {"Tab+MMB", "Drag to resize quad strips"}, {"Shift+LMB", "Click between dots to create geometry"}});
        f->addWidget(new QLabel("<i>Hovering inside the quad mesh</i>"));
        shortcut_rows(f, {{"LMB", "Drag to tweak components"}, {"Ctrl+LMB", "Click edge to insert loop"}, {"Ctrl+MMB", "Click edge to insert centered loop"},
                          {"Shift+LMB", "Drag to relax points"}, {"Tab+LMB", "Drag to extend border edge"}, {"Tab+MMB", "Drag to extend border edge loop"},
                          {"Ctrl+Shift+LMB", "Click face to delete it"}, {"Ctrl+Shift+LMB", "Click edge to delete edge loop"}});
    }
    return panel;
}

QWidget* multi_cut_panel(MainWindow* mw, QWidget* parent) {
    auto* panel = new QWidget(parent);
    auto* page = new QVBoxLayout(panel);
    page->setContentsMargins(0, 0, 0, 0);
    page->setSpacing(4);
    const QJsonObject o = tool_options("multiCut");
    auto spin = [mw](QWidget* body, const QString& key, double lo, double hi, double v, int decimals) {
        auto* sb = narrow(new QDoubleSpinBox(body));
        no_stray_wheel(sb);
        sb->setRange(lo, hi);
        sb->setDecimals(decimals);
        sb->setValue(v);
        QObject::connect(sb, &QDoubleSpinBox::valueChanged, body, [mw, key](double x) { set_tool_option(mw, "multiCut", key, x); });
        return sb;
    };
    auto check = [mw](QWidget* body, const QString& label, const QString& key, bool v) {
        auto* cb = new QCheckBox(label, body);
        cb->setChecked(v);
        QObject::connect(cb, &QCheckBox::toggled, body, [mw, key](bool on) { set_tool_option(mw, "multiCut", key, on); });
        return cb;
    };
    {
        QWidget* body = section(page, "Multi-Cut Options", panel);
        auto* f = new QFormLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        QDoubleSpinBox* step = spin(body, "snapStep", 0, 100, o["snapStep"].toDouble(), 0);
        step->setSuffix(" %");
        step->setToolTip("Shift+click or drag snaps a point along an edge in steps of this much of its length");
        f->addRow("Snap Step %:", step);
    }
    {
        QWidget* body = section(page, "Insert Edge Loop Tool", panel);
        auto* f = new QFormLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        f->addRow("Smoothing Angle:", spin(body, "smoothingAngle", 0, 180, o["smoothingAngle"].toDouble(), 0));
        auto* row = new QHBoxLayout();
        QCheckBox* flow = check(body, "Edge Flow", "edgeFlow", o["edgeFlow"].toBool());
        flow->setToolTip("A new loop follows the curvature of the loops beside it; off on a live surface, where it follows the surface");
        QDoubleSpinBox* amount = spin(body, "edgeFlowAmount", -10, 10, o["edgeFlowAmount"].toDouble(), 2);
        amount->setEnabled(flow->isChecked());
        QObject::connect(flow, &QCheckBox::toggled, amount, &QWidget::setEnabled);
        row->addWidget(flow);
        row->addWidget(amount, 1);
        f->addRow(row);
        auto* subs = narrow(new QSpinBox(body));
        no_stray_wheel(subs);
        subs->setRange(1, 64);
        subs->setValue(o["subdivisions"].toInt());
        subs->setToolTip("How many evenly spaced loops Ctrl+click inserts");
        QObject::connect(subs, &QSpinBox::valueChanged, body, [mw](int v) { set_tool_option(mw, "multiCut", "subdivisions", v); });
        f->addRow("Subdivisions:", subs);
    }
    {
        QWidget* body = section(page, "Slice Tool", panel);
        auto* f = new QFormLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        f->addRow(check(body, "Ignore Backfaces", "ignoreBackfaces", o["ignoreBackfaces"].toBool()));
        QCheckBox* del = check(body, "Delete Faces", "deleteFaces", o["deleteFaces"].toBool());
        QCheckBox* extract = check(body, "Extract Faces", "extractFaces", o["extractFaces"].toBool());
        f->addRow(del);
        f->addRow(extract);
        auto* off = new QWidget(body);
        auto* h = new QHBoxLayout(off);
        h->setContentsMargins(0, 0, 0, 0);
        QList<QDoubleSpinBox*> parts;
        const QJsonArray now = o["extractOffset"].toArray();
        for (int k = 0; k < 3; ++k) {
            auto* sb = narrow(new QDoubleSpinBox(off));
            no_stray_wheel(sb);
            sb->setRange(-1e4, 1e4);
            sb->setDecimals(3);
            sb->setValue(now.size() == 3 ? now[k].toDouble() : 0.5);
            h->addWidget(sb);
            parts << sb;
        }
        for (QDoubleSpinBox* sb : parts)
            QObject::connect(sb, &QDoubleSpinBox::valueChanged, off, [mw, parts] {
                set_tool_option(mw, "multiCut", "extractOffset", QJsonArray{parts[0]->value(), parts[1]->value(), parts[2]->value()});
            });
        f->addRow(off);
        // Delete and Extract are one or the other; the offset is Extract's.
        auto sync = [del, extract, off] {
            extract->setEnabled(!del->isChecked());
            off->setEnabled(!del->isChecked() && extract->isChecked());
        };
        QObject::connect(del, &QCheckBox::toggled, body, sync);
        QObject::connect(extract, &QCheckBox::toggled, body, sync);
        sync();
        f->addRow(new QLabel("Slice Along Plane:"));
        auto* planes = new QHBoxLayout();
        for (const char* p : {"YZ", "ZX", "XY"}) {
            auto* b = narrow(new QPushButton(p, body));
            b->setToolTip(QString("Cut the selected objects through their middle along the world %1 plane").arg(p));
            const QString key = QString(p).toLower();
            QObject::connect(b, &QPushButton::clicked, body, [mw, key] {
                if (ViewportWidget* v = mw->active_view()) v->set("multiCutAction", "\"" + key + "\"");
                Host::get()->poke();
            });
            planes->addWidget(b);
        }
        f->addRow(planes);
    }
    auto* reset = new QPushButton("Reset Settings", panel);
    QObject::connect(reset, &QPushButton::clicked, panel, [mw] {
        QSettings().remove("mtk/multiCut");
        mw->reset_tool_panel("multiCut");
    });
    page->addWidget(reset);
    {
        QWidget* body = section(page, "Keyboard/Mouse Shortcuts", panel);
        auto* f = new QVBoxLayout(body);
        f->setContentsMargins(4, 2, 2, 4);
        shortcut_rows(f, {{"LMB", "Click to drop a point"}, {"Ctrl+LMB", "Insert edge loop"}, {"Ctrl+MMB", "Insert centered edge loop"},
                          {"MMB", "Drag across the mesh to slice it"}, {"Shift+MMB", "Slice at 45 degree steps"}, {"Backspace", "Undo last dropped point"},
                          {"Shift+LMB", "Snap a point along an edge by the snap step"}, {"Ctrl+Shift+LMB", "Constrain to the perpendicular of the last edge"},
                          {"Enter", "Commit cut"}});
    }
    return panel;
}

} // namespace

void MainWindow::show_modeling_toolkit() {
    if (!mtk_dock_) {
        mtk_dock_ = new QDockWidget("Modeling Toolkit", this);
        mtk_dock_->setObjectName("modelingToolkitDock");
        auto* scroll = new QScrollArea(mtk_dock_);
        scroll->setWidgetResizable(true);
        scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        auto* w = new QWidget(scroll);
        auto* page = new QVBoxLayout(w);
        page->setContentsMargins(4, 0, 4, 4);
        page->setSpacing(4);

        auto* bar = new QMenuBar(w);
        bar->setNativeMenuBar(false);
        // Object: display switches for the selected objects, as the reference's toolkit has them.
        QMenu* object = bar->addMenu("Object");
        object->addAction("Show/Hide", [] {
            mel_action("string $s[] = `ls -sl -o`; for ($o in $s) { if (`objExists ($o + \".visibility\")`) setAttr ($o + \".visibility\") (!`getAttr ($o + \".visibility\")`); }");
            Host::get()->poke();
        });
        object->addAction("Freeze/Unfreeze", [] {
            // Frozen: drawn as a reference, out of reach of clicks.
            mel_action("string $s[] = `ls -sl -o -tr`; for ($o in $s) { int $on = `getAttr ($o + \".overrideDisplayType\")` == 2 && `getAttr ($o + \".overrideEnabled\")`;"
                       " setAttr ($o + \".overrideEnabled\") (!$on); setAttr ($o + \".overrideDisplayType\") ($on ? 0 : 2); }");
            Host::get()->poke();
        });
        object->addAction("X-Ray On/Off", [] {
            mel_action("string $s[] = `ls -sl -o`; for ($o in $s) { int $x[] = `displaySurface -q -xRay $o`; displaySurface -xRay (!$x[0]) $o; }");
            Host::get()->poke();
        });
        object->addAction("Show/Hide Backfaces", [] {
            mel_action("string $s[] = `ls -sl -o`; for ($o in $s) { int $c = `displayCull -q -bfc $o`; displayCull -bfc (!$c) $o; }");
            Host::get()->poke();
        });
        object->addAction("Show/Hide Face Triangles", [] {
            mel_action("polyOptions -r -dt;");
            Host::get()->poke();
        });
        QMenu* help = bar->addMenu("Help");
        help->addAction("Modeling Toolkit", [] { QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek#readme")); });
        page->addWidget(bar);

        // Multi-Component, then the five types.
        auto* multi = new QPushButton("Multi-Component", w);
        multi->setCheckable(true);
        multi->setToolTip("Vertices, edges and faces picked together, whichever is nearest");
        page->addWidget(multi);
        auto* types = new QHBoxLayout();
        types->setSpacing(2);
        auto* group = new QButtonGroup(w);
        struct T { const char* type; const char* tip; };
        const T kinds[] = {
            {"object", "Object selection (F8)\nCtrl + LMB: Convert selection to objects\nRMB: Clear selection"},
            {"vertex", "Vertex selection (F9)\nCtrl + LMB: Convert selection to vertices\nCtrl + Shift + LMB: Convert selection to vertex perimeter\nRMB: Clear selection"},
            {"edge", "Edge selection (F10)\nCtrl + LMB: Convert selection to edges\nCtrl + Shift + LMB: Convert selection to edge perimeter\nRMB: Clear selection"},
            {"face", "Face selection (F11)\nCtrl + LMB: Convert selection to faces\nCtrl + Shift + LMB: Convert selection to face perimeter\nRMB: Clear selection"},
            {"uv", "UV selection (F12)\nCtrl + LMB: Convert selection to UVs\nCtrl + Shift + LMB: Convert selection to UV perimeter\nRMB: Clear selection"},
        };
        QHash<QString, QAbstractButton*> buttons;
        for (const T& k : kinds) {
            auto* b = new TypeButton(k.type, k.tip, w);
            group->addButton(b);
            buttons[k.type] = b;
            types->addWidget(b);
            const QString type = k.type;
            connect(b, &QToolButton::clicked, this, [type] {
                exec("selectMode", QJsonObject{{"args", QJsonArray{type}}});
                Host::get()->poke();
            });
        }
        connect(multi, &QPushButton::clicked, this, [multi] {
            exec("selectMode", QJsonObject{{"args", QJsonArray{multi->isChecked() ? "multi" : "vertex"}}});
            Host::get()->poke();
        });
        page->addLayout(types);

        // How a click selects: picks and marquees, a drag across paints, or a press on a component moves it.
        auto* how = new QButtonGroup(w);
        auto* pick = new QRadioButton("Pick/Marquee", w);
        auto* drag = new QRadioButton("Drag", w);
        auto* tweak = new QRadioButton("Tweak/Marquee", w);
        pick->setChecked(true);
        for (QRadioButton* r : {pick, drag, tweak}) {
            how->addButton(r);
            page->addWidget(r);
        }
        connect(pick, &QRadioButton::clicked, this, [this] {
            for (ViewportWidget* v : panels_->views()) v->set("tweakMode", "false");
            set_tool("select");
        });
        connect(drag, &QRadioButton::clicked, this, [this] { set_tool("paintSelect"); });
        connect(tweak, &QRadioButton::clicked, this, [this] {
            set_tool("move");
            for (ViewportWidget* v : panels_->views()) v->set("tweakMode", "true");
        });
        // Highlight Backfaces first, then Highlight Nearest Component, as the reference lists them.
        auto* backfaces = new QCheckBox("Highlight Backfaces", w);
        backfaces->setToolTip("On: back-facing components highlight under the cursor too");
        backfaces->setChecked(ask("selectPref", QJsonObject{{"query", true}, {"preSelectBackfacing", true}}).toBool(false));
        connect(backfaces, &QCheckBox::toggled, this, [](bool on) { exec_quiet("selectPref", QJsonObject{{"preSelectBackfacing", on}}); });
        page->addWidget(backfaces);
        auto* nearest = new QCheckBox("Highlight Nearest Component", w);
        nearest->setChecked(ask("selectPref", QJsonObject{{"query", true}, {"preSelectHilite", true}}).toBool(true));
        connect(nearest, &QCheckBox::toggled, this, [](bool on) { exec_quiet("selectPref", QJsonObject{{"preSelectHilite", on}}); });
        page->addWidget(nearest);

        auto* form = new QFormLayout();
        form->setContentsMargins(0, 4, 0, 4);
        auto* camera = new QComboBox(w);
        no_stray_wheel(camera);
        camera->addItems({"Off", "On"});
        camera->setToolTip("On: a marquee picks only what can be seen");
        camera->setCurrentIndex(ask("selectPref", QJsonObject{{"query", true}, {"useDepth", true}}).toBool() ? 1 : 0);
        connect(camera, &QComboBox::activated, this, [](int i) { exec_quiet("selectPref", QJsonObject{{"useDepth", i == 1}}); });
        form->addRow("Camera Based Selection", camera);
        auto* sym = new QComboBox(w);
        no_stray_wheel(sym);
        sym->addItems({"Off", "Object X", "Object Y", "Object Z", "World X", "World Y", "World Z", "Topology"});
        connect(sym, &QComboBox::activated, this, [](int i) {
            QJsonObject c{{"edit", true}, {"symmetry", i != 0}};
            if (i == 7) {
                c["topoSymmetry"] = true;
            } else if (i != 0) {
                c["about"] = i <= 3 ? "object" : "world";
                c["axis"] = QString(QChar('x' + (i - 1) % 3));
            }
            exec("symmetricModelling", c);
        });
        form->addRow("Symmetry", sym);
        auto* constraint = new QComboBox(w);
        no_stray_wheel(constraint);
        constraint->addItems({"Off", "Angle", "Border", "Edge Loop", "Edge Ring", "Shell"});
        connect(constraint, &QComboBox::activated, this, [](int i) {
            exec("polySelectConstraint", QJsonObject{{"disable", true}});
            static const char* flags[] = {"", "anglePropagation", "borderPropagation", "loopPropagation", "ringPropagation", "shell"};
            if (i > 0) exec("polySelectConstraint", QJsonObject{{"mode", 3}, {flags[i], true}});
        });
        form->addRow("Selection Constraint", constraint);
        auto* xform = new QComboBox(w);
        no_stray_wheel(xform);
        xform->addItem("Off", "none");
        xform->addItem("Edge", "edge");
        xform->addItem("Surface", "surface");
        connect(xform, &QComboBox::activated, this, [xform](int) { exec_quiet("xformConstraint", QJsonObject{{"type", xform->currentData().toString()}}); });
        form->addRow("Transform Constraint", xform);
        page->addLayout(form);

        // Soft Selection.
        {
            QWidget* body = section(page, "Soft Selection", w);
            auto* f = new QFormLayout(body);
            f->setContentsMargins(4, 2, 2, 4);
            auto* soft = new QCheckBox(body);
            soft->setChecked(ask("softSelect", QJsonObject{{"query", true}, {"softSelectEnabled", true}}).toInt() != 0);
            connect(soft, &QCheckBox::toggled, this, [](bool on) { exec("softSelect", QJsonObject{{"softSelectEnabled", on ? 1 : 0}}); });
            f->addRow("Soft Select:", soft);
            auto* row = new QHBoxLayout();
            auto* mode = new QComboBox(body);
        no_stray_wheel(mode);
            mode->addItems({"Volume", "Surface", "Global", "Object"});
            const QJsonValue m = ask("softSelect", QJsonObject{{"query", true}, {"queried", QJsonArray{"softSelectFalloff"}}});
            if (m.isDouble()) mode->setCurrentIndex(m.toInt());
            connect(mode, &QComboBox::activated, this, [](int i) { exec("softSelect", QJsonObject{{"softSelectFalloff", i}}); });
            auto* radius = new QDoubleSpinBox(body);
        no_stray_wheel(radius);
            radius->setRange(0, 1000);
            radius->setDecimals(2);
            const QJsonValue r = ask("softSelect", QJsonObject{{"query", true}, {"queried", QJsonArray{"softSelectDistance"}}});
            radius->setValue(r.isDouble() ? r.toDouble() : 5.0);
            connect(radius, &QDoubleSpinBox::editingFinished, this, [radius] { exec_quiet("softSelect", QJsonObject{{"softSelectDistance", radius->value()}}); });
            row->addWidget(mode, 1);
            row->addWidget(radius);
            f->addRow(row);
            std::function<void()> reload;
            QWidget* curve = falloff_curve(body, &reload);
            f->addRow(curve);
            auto* reset = new QPushButton("Reset Curve", body);
            connect(reset, &QPushButton::clicked, this, [reload] {
                exec_quiet("softSelectCurve", QJsonObject{{"preset", "Smooth"}});
                if (reload) reload();
            });
            f->addRow(reset);
        }

        // The tools, two to a row.
        auto grid = [this, page, w](const QString& title, QList<std::tuple<QString, QString, std::function<void()>>> items) {
            QWidget* body = section(page, title, w);
            auto* g = new QGridLayout(body);
            g->setContentsMargins(0, 2, 0, 4);
            g->setSpacing(2);
            int i = 0;
            for (auto& [label, ic, fn] : items) {
                auto* b = new QPushButton(icon(ic), label, body);
                b->setStyleSheet("QPushButton { text-align: left; padding: 3px 6px; }");
                connect(b, &QPushButton::clicked, this, fn);
                g->addWidget(b, i / 2, i % 2);
                ++i;
            }
        };
        auto run = [](const QString& cmd) { return [cmd] { exec(cmd); Host::get()->poke(); }; };
        grid("Mesh", {{"Combine", "combine", run("polyUnite")},
                      {"Separate", "separate", run("polySeparate")},
                      {"Smooth", "smooth", run("polySmooth")},
                      {"Boolean", "boolean", [] { exec("polyCBoolOp", QJsonObject{{"operation", 1}}); Host::get()->poke(); }}});
        grid("Components", {{"Extrude", "extrude", run("polyExtrude")},
                            {"Bevel", "bevel", run("polyBevel")},
                            {"Smart Extrude", "extrude", [this] {
                                 // Extruded and straight into the move manipulator, to drag out.
                                 exec("polyExtrude");
                                 set_tool("move");
                                 Host::get()->poke();
                             }},
                            {"Add Divisions", "insertLoop", run("polySubdivideFacet")},
                            {"Bridge", "bridge", run("polyBridge")}});
        // Tools: Multi-Cut and Quad Draw are mode tools, their buttons lit while they are on, with their options below
        // (#140).
        {
            QWidget* body = section(page, "Tools", w);
            auto* g = new QGridLayout(body);
            g->setContentsMargins(0, 2, 0, 4);
            g->setSpacing(2);
            const std::tuple<QString, QString, QString> tools[] = {
                {"Multi-Cut", "multiCut", "multiCut"}, {"Target Weld", "merge", "targetWeld"}, {"Connect", "insertLoop", ""}, {"Quad Draw", "quadDraw", "quadDraw"}};
            int i = 0;
            for (const auto& [label, ic, tool] : tools) {
                auto* b = new QPushButton(icon(ic), label, body);
                b->setStyleSheet("QPushButton { text-align: left; padding: 3px 6px; } QPushButton:checked { background: #3d6f99; }");
                if (tool.isEmpty()) {
                    connect(b, &QPushButton::clicked, this, run("polyConnectComponents"));
                } else {
                    b->setCheckable(true);
                    mtk_tools_[tool] = b;
                    const QString t = tool;
                    connect(b, &QPushButton::clicked, this, [this, t, b] {
                        // A second click on a lit tool puts it down, as the reference's toolkit does.
                        set_tool(b->isChecked() ? t : "select");
                    });
                }
                g->addWidget(b, i / 2, i % 2);
                ++i;
            }
        }
        mtk_tool_panels_["quadDraw"] = quad_draw_panel(this, w);
        mtk_tool_panels_["multiCut"] = multi_cut_panel(this, w);
        for (QWidget* p : mtk_tool_panels_) page->addWidget(p);
        page->addStretch(1);
        scroll->setWidget(w);
        mtk_dock_->setWidget(scroll);
        addDockWidget(Qt::RightDockWidgetArea, mtk_dock_);
        tabifyDockWidget(attr_dock_, mtk_dock_);
        lock_layout(QSettings().value("window/uiLocked", false).toBool());

        // The type buttons and Multi-Component follow the scene's selection type.
        auto follow = [buttons, multi] {
            const QString mode = state()["mode"].toString();
            multi->setChecked(mode == "multi");
            if (QAbstractButton* b = buttons.value(mode == "uvShell" ? "uv" : mode)) {
                if (!b->isChecked()) b->setChecked(true);
            }
        };
        connect(Host::get(), &Host::sceneChanged, mtk_dock_, follow);
        follow();
    }
    mtk_follow_tool(tool_);
    mtk_dock_->show();
    mtk_dock_->raise();
}

void MainWindow::mtk_follow_tool(const QString& tool) {
    for (auto it = mtk_tools_.begin(); it != mtk_tools_.end(); ++it) it.value()->setChecked(it.key() == tool);
    for (auto it = mtk_tool_panels_.begin(); it != mtk_tool_panels_.end(); ++it) it.value()->setVisible(it.key() == tool);
    if (tool == "quadDraw" || tool == "multiCut") push_tool_options(tool);
}

void MainWindow::reset_tool_panel(const QString& tool) {
    QWidget* old = mtk_tool_panels_.value(tool);
    if (!old) return;
    QWidget* fresh = tool == "quadDraw" ? quad_draw_panel(this, old->parentWidget()) : multi_cut_panel(this, old->parentWidget());
    if (auto* lay = qobject_cast<QVBoxLayout*>(old->parentWidget()->layout())) lay->insertWidget(lay->indexOf(old), fresh);
    mtk_tool_panels_[tool] = fresh;
    old->deleteLater();
    mtk_follow_tool(tool_);
}

void MainWindow::push_tool_options(const QString& tool) {
    const QJsonObject o = tool_options(tool);
    const QString text = QString::fromUtf8(QJsonDocument(o).toJson(QJsonDocument::Compact));
    for (ViewportWidget* v : panels_->views()) v->set(tool, text);
}

} // namespace az

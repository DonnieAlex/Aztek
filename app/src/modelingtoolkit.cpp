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
        setIcon(az::icon(type));
        setIconSize(QSize(28, 28));
        setMinimumSize(52, 40);
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
        auto* nearest = new QCheckBox("Highlight Nearest Component", w);
        nearest->setChecked(ask("selectPref", QJsonObject{{"query", true}, {"preSelectHilite", true}}).toBool(true));
        connect(nearest, &QCheckBox::toggled, this, [](bool on) { exec_quiet("selectPref", QJsonObject{{"preSelectHilite", on}}); });
        page->addWidget(nearest);
        auto* backfaces = new QCheckBox("Highlight Backfaces", w);
        backfaces->setToolTip("On: back-facing components highlight under the cursor too");
        backfaces->setChecked(ask("selectPref", QJsonObject{{"query", true}, {"preSelectBackfacing", true}}).toBool(false));
        connect(backfaces, &QCheckBox::toggled, this, [](bool on) { exec_quiet("selectPref", QJsonObject{{"preSelectBackfacing", on}}); });
        page->addWidget(backfaces);

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
        grid("Mesh", {{"Combine", "polyUnite", run("polyUnite")},
                      {"Separate", "polySeparate", run("polySeparate")},
                      {"Smooth", "polySmooth", run("polySmooth")},
                      {"Boolean", "polyBooleanUnion", [] { exec("polyCBoolOp", QJsonObject{{"operation", 1}}); Host::get()->poke(); }}});
        grid("Components", {{"Extrude", "polyExtrude", run("polyExtrude")},
                            {"Bevel", "polyBevel", run("polyBevel")},
                            {"Smart Extrude", "polyExtrude", [this] {
                                 // Extruded and straight into the move manipulator, to drag out.
                                 exec("polyExtrude");
                                 set_tool("move");
                                 Host::get()->poke();
                             }},
                            {"Bridge", "polyBridge", run("polyBridge")},
                            {"Add Divisions", "polySubdivideFacet", run("polySubdivideFacet")}});
        grid("Tools", {{"Multi-Cut", "multiCut", [this] { set_tool("multiCut"); }},
                       {"Target Weld", "targetWeld", [this] { set_tool("targetWeld"); }},
                       {"Connect", "polyConnectComponents", run("polyConnectComponents")},
                       {"Quad Draw", "quadDraw", [this] { set_tool("quadDraw"); }}});
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
    mtk_dock_->show();
    mtk_dock_->raise();
}

} // namespace az

// The UV Editor's UV Toolkit (#108), the side panel as the reference has it, built in batches as the issue asks:
// Selection, Pinning, Select By Type, Soft Selection, Transform, Texel Density, Create, Cut and Sew, Unfold, Align and
// Snap, and Arrange and Layout.

#include "uveditor.h"
#include "host.h"
#include "icons.h"
#include "toolsettings.h"

#include <QAbstractScrollArea>
#include <QApplication>
#include <QButtonGroup>
#include <QKeySequence>
#include <QMenu>
#include <QUrl>
#include <QDesktopServices>
#include <QFileDialog>
#include <QMenuBar>
#include <QCheckBox>
#include <QComboBox>
#include <QDoubleSpinBox>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QLabel>
#include <QSignalBlocker>
#include <QToolBar>
#include <QInputDialog>
#include <QActionGroup>
#include <QFileInfo>
#include <QSpinBox>
#include <QDialogButtonBox>
#include <QDialog>
#include <QLineEdit>
#include <cmath>
#include <QMouseEvent>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QToolButton>
#include <QVBoxLayout>

namespace az {

// Fields take the wheel only once clicked into, so scrolling the panel doesn't change them on the way past.
struct WheelOnlyWhenFocused : QObject {
    using QObject::QObject;
    bool eventFilter(QObject* o, QEvent* e) override {
        if (e->type() == QEvent::Wheel) {
            if (auto* w = qobject_cast<QWidget*>(o); w && !w->hasFocus()) {
                // The panel scrolls instead.
                for (QWidget* p = w->parentWidget(); p; p = p->parentWidget())
                    if (auto* area = qobject_cast<QAbstractScrollArea*>(p)) {
                        QCoreApplication::sendEvent(area->viewport(), e);
                        break;
                    }
                return true;
            }
        }
        return false;
    }
};

void no_stray_wheel(QWidget* w) {
    static WheelOnlyWhenFocused* filter = new WheelOnlyWhenFocused(qApp);
    w->setFocusPolicy(Qt::StrongFocus);
    w->installEventFilter(filter);
}

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
    no_stray_wheel(s);
    return s;
}

} // namespace

// Snap Together: the two UVs picked, one on each shell. The shell of the one listed second (by mesh, then number)
// moves so its UV lands on the other's; b_to_a moves the other one instead.
void UvEditor::snap_together(bool b_to_a) {
    const QJsonArray t = canvas_->targets();
    QJsonArray uvs;
    if (!t.isEmpty())
        for (const QJsonValue& x : ask("ls", QJsonObject{{"args", t}, {"flatten", true}}).toArray())
            if (x.toString().contains(".map[")) uvs.append(x);
    if (uvs.size() != 2) {
        log("// Warning: Snap Together: pick one UV on each of two shells //", 4);
        return;
    }
    exec("texSnapTogether", QJsonObject{{"args", uvs}, {"order", b_to_a ? "ba" : "ab"}});
    Host::get()->poke();
}

// Assign Checker Shader: a lambert with a checker on its colour on the selected meshes, made once and reused.
void UvEditor::assign_checker() {
    mel_action("if (!`objExists aztekUvChecker`) {"
               " shadingNode -asShader lambert -n aztekUvChecker;"
               " sets -renderable true -noSurfaceShader true -empty -n aztekUvCheckerSG;"
               " connectAttr -f aztekUvChecker.outColor aztekUvCheckerSG.surfaceShader;"
               " shadingNode -asTexture checker -n aztekUvCheckerMap;"
               " connectAttr -f aztekUvCheckerMap.outColor aztekUvChecker.color;"
               " setAttr aztekUvCheckerMap.repeatU 8; setAttr aztekUvCheckerMap.repeatV 8;"
               "}"
               " sets -e -forceElement aztekUvCheckerSG;");
    Host::get()->poke();
}

// Camera-based: planar through the camera of the view in use.
void UvEditor::camera_projection() {
    QString cam = ask("lookThru").toString();
    if (cam.isEmpty()) cam = "persp";
    run("polyProjection", {{"type", "planar"}, {"mapDirection", "c"}, {"camera", cam}});
    Host::get()->poke();
}

// Symmetrize: the UVs picked mirrored across the edge picked with them, onto the other side of the mesh.
void UvEditor::symmetrize() {
    const QJsonArray t = canvas_->targets();
    QJsonArray flat = t.isEmpty() ? QJsonArray() : ask("ls", QJsonObject{{"args", t}, {"flatten", true}}).toArray();
    if (flat.isEmpty()) flat = ask("ls", QJsonObject{{"selection", true}, {"flatten", true}}).toArray();
    QString edge;
    QJsonArray uvs;
    for (const QJsonValue& x : flat) {
        const QString n = x.toString();
        if (n.contains(".e[") && edge.isEmpty()) edge = n;
        else if (n.contains(".map[")) uvs.append(n);
    }
    if (edge.isEmpty() || uvs.isEmpty()) {
        log("// Warning: Symmetrize: pick the UVs to mirror and one edge on the mesh's line of symmetry //", 4);
        return;
    }
    exec("texSymmetrizeUVs", QJsonObject{{"args", uvs}, {"edge", edge}});
    Host::get()->poke();
}

QWidget* UvEditor::build_menu_bar() {
    auto* bar = new QMenuBar(this);
    bar->setNativeMenuBar(false);
    auto cmd = [this](const QString& c, const QJsonObject& call = QJsonObject()) {
        return [this, c, call] {
            run(c, call);
            Host::get()->poke();
        };
    };
    auto meshes = [this] {
        QJsonArray out;
        for (const QString& m : canvas_->mesh_paths()) out.append(m);
        return out;
    };

    QMenu* edit = bar->addMenu("Edit");
    edit->addAction("Copy", cmd("polyClipboard", {{"copy", true}, {"uv", true}}))->setShortcut(QKeySequence("Ctrl+C"));
    edit->addAction("Paste", cmd("polyClipboard", {{"paste", true}, {"uv", true}}))->setShortcut(QKeySequence("Ctrl+V"));
    edit->addAction("Delete", cmd("polyMapDel"));
    edit->addSection("Pinning");
    edit->addAction("Pin Selection", cmd("polyPinUV", {{"value", 1.0}}));
    edit->addAction("Invert Pinning", cmd("polyPinUV", {{"value", 1.0}, {"unpinned", true}}));
    edit->addAction("Unpin Selection", cmd("polyPinUV", {{"value", 0.0}}));
    edit->addAction("Unpin All", [meshes] {
        const QJsonArray m = meshes();
        if (!m.isEmpty()) exec("polyPinUV", QJsonObject{{"args", m}, {"operation", 2}});
        Host::get()->poke();
    });

    QMenu* create = bar->addMenu("Create");
    create->addSection("UV Mapping");
    create->addAction("Assign Checker Shader", [this] { assign_checker(); });
    create->addSeparator();
    create->addAction("Automatic", cmd("polyAutoProjection"));
    create->addAction("Camera-based", [this] { camera_projection(); });
    create->addAction("Normal-based", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "n"}}));
    create->addSeparator();
    create->addAction("Cylindrical", cmd("polyProjection", {{"type", "cylindrical"}}));
    create->addAction("Planar", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}}));
    create->addAction("Spherical", cmd("polyProjection", {{"type", "spherical"}}));
    create->addSeparator();
    create->addAction("Best Plane", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}}));
    create->addAction("Contour Stretch", cmd("polyContourProjection"));

    QMenu* select = bar->addMenu("Select");
    select->addAction("All", [this] { canvas_->select_all_uvs(false); })->setShortcut(QKeySequence("Ctrl+Shift+A"));
    select->addAction("Clear", [meshes] {
        const QJsonArray m = meshes();
        exec("select", m.isEmpty() ? QJsonObject{{"clear", true}} : QJsonObject{{"args", m}});
        Host::get()->poke();
    });
    select->addAction("Inverse", [this] { canvas_->select_all_uvs(true); })->setShortcut(QKeySequence("Ctrl+Shift+I"));
    select->addSeparator();
    auto pick = [this, meshes](const QString& type, const QString& mode) {
        return [this, meshes, type, mode] {
            const QJsonArray m = meshes();
            if (m.isEmpty()) return;
            const QJsonArray got = ask("texSelectByType", QJsonObject{{"args", m}, {"type", type}}).toArray();
            exec("select", got.isEmpty() ? QJsonObject{{"args", m}} : QJsonObject{{"args", got}});
            canvas_->set_mode(mode);
            Host::get()->poke();
        };
    };
    auto overlap = [this, meshes](bool over) {
        return [this, meshes, over] {
            const QJsonArray m = meshes();
            if (m.isEmpty()) return;
            const QJsonArray got = ask("polyUVOverlap", QJsonObject{{"args", m}, {over ? "overlappingComponents" : "nonOverlappingComponents", true}}).toArray();
            exec("select", got.isEmpty() ? QJsonObject{{"args", m}} : QJsonObject{{"args", got}});
            canvas_->set_mode("face");
            Host::get()->poke();
        };
    };
    select->addAction("Back-Facing", pick("backFacing", "face"));
    select->addAction("Front-Facing", pick("frontFacing", "face"));
    select->addAction("Overlapping", overlap(true));
    select->addAction("Non-Overlapping", overlap(false));
    select->addAction("Texture Borders", pick("textureBorders", "uv"));
    select->addAction("Unmapped Faces", pick("unmapped", "face"));
    select->addSeparator();
    select->addAction("Shortest Edge Path Tool", [this] { canvas_->set_tool("path"); });
    select->addSeparator();
    select->addAction("Grow\t>", [] { exec("growSelection"); Host::get()->poke(); });
    select->addAction("Shrink\t<", [] { exec("shrinkSelection"); Host::get()->poke(); });
    QMenu* convert = select->addMenu("Convert Selection");
    for (auto [label, to] : {std::pair{"To UVs", "uv"}, {"To Edges", "edge"}, {"To Faces", "face"}, {"To Vertices", "vertex"}}) {
        const QString t = to;
        convert->addAction(label, [t] { exec("convertSelection", QJsonObject{{"args", QJsonArray{t}}}); Host::get()->poke(); });
    }

    QMenu* sew = bar->addMenu("Cut/Sew");
    sew->addAction("Auto Seams", cmd("u3dAutoSeam", {{"splitShells", 0.5}}));
    sew->addSeparator();
    sew->addAction("Create UV Shell", cmd("texCreateShell"))->setShortcut(QKeySequence("Ctrl+Shift+X"));
    sew->addAction("Create Shell (Grid)", cmd("texCreateShell", {{"grid", true}}));
    sew->addSeparator();
    sew->addAction("Cut", cmd("polyMapCut"))->setShortcut(QKeySequence("Shift+X"));
    sew->addAction("Sew", cmd("polyMapSew"))->setShortcut(QKeySequence("Shift+S"));
    sew->addAction("Split", [] { mel_action("SplitUV;"); });
    sew->addSeparator();
    sew->addAction("Merge", cmd("polyMergeUV", {{"distance", 0.01}}));
    sew->addAction("Move and Sew", cmd("polyMapSewMove"));
    sew->addAction("Stitch Together", cmd("polyMapSewMove"));

    QMenu* modify = bar->addMenu("Modify");
    QMenu* align = modify->addMenu("Align");
    for (auto [label, to] : {std::pair{"Left", "left"}, {"Middle U", "centerU"}, {"Right", "right"}, {"Top", "top"}, {"Middle V", "centerV"}, {"Bottom", "bottom"}})
        align->addAction(label, cmd("texAlignShells", {{"alignTo", to}}));
    modify->addAction("Cycle", cmd("polyRotateUVsByVertex"));
    QMenu* dist_uvs = modify->addMenu("Distribute UVs");
    dist_uvs->addAction("Along U", cmd("texDistributeUVs", {{"direction", "u"}}));
    dist_uvs->addAction("Along V", cmd("texDistributeUVs", {{"direction", "v"}}));
    QMenu* flip = modify->addMenu("Flip");
    flip->addAction("U", cmd("polyFlipUV", {{"flipType", 0}}));
    flip->addAction("V", cmd("polyFlipUV", {{"flipType", 1}}));
    modify->addAction("Linear Align", cmd("texLinearAlignUVs"));
    modify->addAction("Match Grid", cmd("texMatchGrid", {{"gridU", 16.0}, {"gridV", 16.0}}));
    modify->addAction("Match UVs", cmd("texMatchUVs", {{"tolerance", 0.01}}));
    modify->addAction("Normalize", cmd("polyNormalizeUV", {{"normalizeType", 1}}));
    QMenu* rotate = modify->addMenu("Rotate");
    rotate->addAction("90 Counter-clockwise", cmd("polyEditUV", {{"angle", 90.0}}));
    rotate->addAction("90 Clockwise", cmd("polyEditUV", {{"angle", -90.0}}));
    modify->addAction("Symmetrize", [this] { symmetrize(); });
    modify->addAction("Unitize", cmd("polyForceUV", {{"unitize", true}}));
    modify->addSeparator();
    QMenu* dist = modify->addMenu("Distribute Shells");
    dist->addAction("Along U", cmd("texDistributeShells", {{"direction", "u"}, {"spacing", 0.0}}));
    dist->addAction("Along V", cmd("texDistributeShells", {{"direction", "v"}, {"spacing", 0.0}}));
    dist->addAction("Toward Target", cmd("texDistributeShellsToTarget"));
    modify->addAction("Gather Shells", cmd("texGatherShells"));
    modify->addAction("Layout", cmd("u3dLayout"))->setShortcut(QKeySequence("Ctrl+L"));
    QMenu* along = modify->addMenu("Layout Along");
    along->addAction("U", cmd("texLayoutAlong", {{"direction", "u"}}));
    along->addAction("V", cmd("texLayoutAlong", {{"direction", "v"}}));
    modify->addAction("Orient Shells", cmd("texOrientShells"));
    modify->addAction("Orient Shells to Edges", cmd("texOrientEdge"));
    modify->addAction("Randomize Shells", cmd("texRandomizeShells", {{"translate", 0.1}, {"rotate", 15.0}, {"scale", 10.0}, {"seed", 1}}));
    modify->addAction("Snap and Stack", cmd("texSnapStackShells"));
    modify->addAction("Snap Together", [this] { snap_together(false); });
    modify->addAction("Stack Shells", cmd("texStackShells"));
    modify->addAction("Stack and Orient Shells", [this] {
        run("texOrientShells");
        run("texStackShells");
        Host::get()->poke();
    });
    modify->addAction("Stack Similar Shells", cmd("polyUVStackSimilarShells"));
    modify->addAction("Unstack Shells", cmd("texUnstackShells"));
    modify->addSeparator();
    modify->addAction("Map Border", [this] {
        run("polyUVRectangle");
        run("polyNormalizeUV", {{"normalizeType", 0}, {"preserveAspectRatio", false}});
        Host::get()->poke();
    });
    modify->addAction("Optimize", cmd("u3dOptimize"));
    modify->addAction("Straighten Border", cmd("polyStraightenUVBorder"));
    modify->addAction("Straighten Shell", cmd("texStraightenShell"));
    modify->addAction("Straighten UVs", cmd("texStraightenUVs", {{"angle", 30.0}}));
    modify->addAction("Unfold", cmd("u3dUnfold"))->setShortcut(QKeySequence("Ctrl+U"));
    modify->addSeparator();
    modify->addAction("Warp Image...", [this, meshes] {
        // The image laid on by one UV set, drawn again for another.
        const QJsonArray m = meshes();
        if (m.isEmpty()) {
            log("// Warning: Warp Image: select a mesh //", 4);
            return;
        }
        const QString mesh = m.first().toString();
        QStringList sets;
        for (const QJsonValue& v : ask("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"query", true}, {"allUVSets", true}}).toArray()) sets << v.toString();
        QDialog d(this);
        d.setWindowTitle("Warp Image Options");
        auto* f = new QFormLayout(&d);
        auto* in = new QLineEdit(&d);
        auto* browse = new QPushButton("Browse...", &d);
        auto* row = new QHBoxLayout();
        row->addWidget(in, 1);
        row->addWidget(browse);
        connect(browse, &QPushButton::clicked, &d, [&d, in] {
            const QString p = QFileDialog::getOpenFileName(&d, "Image", QString(), "Images (*.png *.jpg *.jpeg *.tif *.tiff *.tga *.exr *.bmp)");
            if (!p.isEmpty()) in->setText(p);
        });
        f->addRow("Input image:", row);
        auto* iuv = new QComboBox(&d);
        auto* ouv = new QComboBox(&d);
        iuv->addItems(sets);
        ouv->addItems(sets);
        if (sets.size() > 1) ouv->setCurrentIndex(1);
        f->addRow("Input UV set:", iuv);
        f->addRow("Output UV set:", ouv);
        auto* xr = new QSpinBox(&d);
        auto* yr = new QSpinBox(&d);
        for (QSpinBox* b : {xr, yr}) {
            b->setRange(1, 16384);
            b->setValue(1024);
        }
        f->addRow("X resolution:", xr);
        f->addRow("Y resolution:", yr);
        auto* aa = new QCheckBox("Anti-alias", &d);
        f->addRow(aa);
        auto* out = new QLineEdit(&d);
        f->addRow("Output file:", out);
        auto* buttons = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel, &d);
        connect(buttons, &QDialogButtonBox::accepted, &d, &QDialog::accept);
        connect(buttons, &QDialogButtonBox::rejected, &d, &QDialog::reject);
        f->addRow(buttons);
        if (d.exec() != QDialog::Accepted || in->text().isEmpty()) return;
        QString o = out->text();
        if (o.isEmpty()) o = QFileInfo(in->text()).absolutePath() + "/" + QFileInfo(in->text()).completeBaseName() + "_warped.png";
        exec("polyWarpImage", QJsonObject{{"args", QJsonArray{mesh}}, {"inputName", in->text()}, {"outputName", o}, {"inputUvSetName", iuv->currentText()},
                                          {"outputUvSetName", ouv->currentText()}, {"xResolution", xr->value()}, {"yResolution", yr->value()}, {"antiAlias", aa->isChecked()}});
    });

    QMenu* tools = bar->addMenu("Tools");
    QAction* hide = tools->addAction("Hide UV Toolkit");
    hide->setCheckable(true);
    connect(hide, &QAction::toggled, this, [this](bool on) {
        if (toolkit_) toolkit_->setVisible(!on);
    });
    tools->addSeparator();
    for (auto [label, tool] : {std::pair{"Select\tQ", "select"}, {"Move\tW", "move"}, {"Rotate\tE", "rotate"}, {"Scale\tR", "scale"}}) {
        const QString t = tool;
        tools->addAction(label, [this, t] { canvas_->set_tool(t); });
    }
    tools->addSeparator();
    for (auto [label, tool] : {std::pair{"Lattice", "lattice"}, {"Lasso", "lasso"}, {"Move UV Shell", "shell"}, {"Smooth", "smooth"}, {"Smudge", "brush:smear"}, {"Tweak", "tweak"}, {"Shortest Edge Path", "path"}}) {
        const QString t = tool;
        tools->addAction(label, [this, t] { canvas_->set_tool(t); });
    }
    tools->addSection("Brushes");
    for (auto [label, tool] : {std::pair{"Optimize", "brush:optimize"}, {"Unfold", "brush:unfold"}}) {
        const QString t = tool;
        tools->addAction(label, [this, t] { canvas_->set_tool(t); });
    }
    tools->addSeparator();
    for (auto [label, tool] : {std::pair{"Cut", "cut"}, {"Grab", "brush:grab"}, {"Pin", "brush:pin"}, {"Pinch", "brush:pinch"}, {"Sew", "sew"}, {"Smear", "brush:smear"}}) {
        const QString t = tool;
        tools->addAction(label, [this, t] { canvas_->set_tool(t); });
    }
    tools->addAction("Symmetrize", [this] { symmetrize(); });

    // View: how the UVs draw, framing and its history, the grid, the editor's own bars.
    QMenu* view = bar->addMenu("View");
    view->addSection("Display Mode");
    auto* modes = new QActionGroup(view);
    QAction* wire = view->addAction("Wireframe\t4", [this] { canvas_->shaded = false; canvas_->update(); });
    QAction* shaded = view->addAction("Shaded\t5", [this] { canvas_->shaded = true; canvas_->update(); });
    for (QAction* a : {wire, shaded}) {
        a->setCheckable(true);
        modes->addAction(a);
    }
    shaded->setChecked(true);
    connect(view, &QMenu::aboutToShow, this, [this, wire, shaded] {
        (canvas_->shaded ? shaded : wire)->setChecked(true);
    });
    auto toggle = [this](QMenu* m, const QString& label, bool* flag) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(*flag);
        // The number keys change these too: the menu shows them as they are.
        connect(m, &QMenu::aboutToShow, a, [a, flag] {
            QSignalBlocker hold(a);
            a->setChecked(*flag);
        });
        connect(a, &QAction::toggled, this, [this, flag](bool on) {
            *flag = on;
            canvas_->update();
        });
        return a;
    };
    toggle(view, "UV Distortion\t7", &canvas_->show_distortion);
    view->addSeparator();
    toggle(view, "Toggle Shell Borders\t8", &canvas_->show_shell_borders);
    toggle(view, "Texture Borders\t9", &canvas_->show_texture_borders);
    view->addSeparator();
    toggle(view, "Grid", &canvas_->show_grid);
    QAction* toolbar = view->addAction("Toolbar");
    toolbar->setCheckable(true);
    toolbar->setChecked(true);
    connect(toolbar, &QAction::toggled, this, [this](bool on) {
        for (QToolBar* b : findChildren<QToolBar*>()) b->setVisible(on);
    });
    view->addSeparator();
    QMenu* iso = view->addMenu("Isolate Select");
    iso->addAction("View Set", [this] { canvas_->frame(true); });
    view->addSeparator();
    view->addAction("Frame All\tA", [this] { canvas_->frame(false); });
    view->addAction("Frame Selection\tF", [this] { canvas_->frame(true); });
    view->addSeparator();
    view->addAction("Undo View Change\tAlt+Z", [this] { canvas_->undo_view(false); })->setShortcut(QKeySequence("Alt+Z"));
    view->addAction("Redo View Change\tAlt+Y", [this] { canvas_->undo_view(true); })->setShortcut(QKeySequence("Alt+Y"));

    // Image: the texture behind the UVs and how it shows.
    QMenu* image = bar->addMenu("Image");
    QAction* display = toggle(image, "Display\t6", &canvas_->show_image);
    display->setChecked(canvas_->show_image);
    toggle(image, "Dim", &canvas_->dim_image);
    toggle(image, "Unfiltered", &canvas_->image_unfiltered);
    image->addSection("Channels");
    auto* channels = new QActionGroup(image);
    int k = 0;
    for (const char* label : {"RGB", "RGBA", "Alpha"}) {
        QAction* a = image->addAction(label);
        a->setCheckable(true);
        a->setChecked(k == canvas_->image_channels);
        channels->addAction(a);
        connect(a, &QAction::triggered, this, [this, k] {
            canvas_->image_channels = k;
            canvas_->update();
        });
        ++k;
    }
    image->addSeparator();
    image->addAction("Image Range...", [this] {
        QDialog d(this);
        d.setWindowTitle("Image Range");
        auto* f = new QFormLayout(&d);
        auto* lo = number(canvas_->image_lo, 0.0, 1.0, &d);
        auto* hi = number(canvas_->image_hi, 0.0, 1.0, &d);
        f->addRow("Minimum:", lo);
        f->addRow("Maximum:", hi);
        auto* b = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel | QDialogButtonBox::Reset, &d);
        connect(b, &QDialogButtonBox::accepted, &d, &QDialog::accept);
        connect(b, &QDialogButtonBox::rejected, &d, &QDialog::reject);
        connect(b->button(QDialogButtonBox::Reset), &QPushButton::clicked, &d, [lo, hi] {
            lo->setValue(0.0);
            hi->setValue(1.0);
        });
        f->addRow(b);
        if (d.exec() != QDialog::Accepted) return;
        canvas_->image_lo = lo->value();
        canvas_->image_hi = std::max(hi->value(), lo->value() + 1e-3);
        canvas_->update();
    });
    image->addSeparator();
    toggle(image, "Image Ratio", &canvas_->image_ratio);
    toggle(image, "Pixel Snap", &canvas_->pixel_snap);
    image->addSeparator();
    image->addAction("Measure Pixel Distance", [this] {
        const QJsonArray t = canvas_->targets();
        QJsonArray uvs;
        if (!t.isEmpty())
            for (const QJsonValue& x : ask("ls", QJsonObject{{"args", t}, {"flatten", true}}).toArray())
                if (x.toString().contains(".map[")) uvs.append(x);
        if (uvs.size() < 2) {
            log("// Warning: Measure Pixel Distance: pick two UVs //", 4);
            return;
        }
        const QJsonArray p = ask("polyEditUV", QJsonObject{{"args", QJsonArray{uvs[0]}}, {"query", true}}).toArray();
        const QJsonArray q = ask("polyEditUV", QJsonObject{{"args", QJsonArray{uvs[uvs.size() - 1]}}, {"query", true}}).toArray();
        if (p.size() < 2 || q.size() < 2) return;
        const QSize sz = canvas_->image_size().isEmpty() ? QSize(1024, 1024) : canvas_->image_size();
        const double du = (q[0].toDouble() - p[0].toDouble()) * sz.width(), dv = (q[1].toDouble() - p[1].toDouble()) * sz.height();
        log(QString("// Result: %1 pixels (%2 x %3 image) //").arg(std::hypot(du, dv), 0, 'f', 2).arg(sz.width()).arg(sz.height()), 0);
    });
    image->addSeparator();
    image->addAction("UV Snapshot...", [this, meshes] {
        const QJsonArray m = meshes();
        if (m.isEmpty()) return;
        const QString f = QFileDialog::getSaveFileName(this, "UV Snapshot", "uv_snapshot.png", "PNG (*.png)");
        if (f.isEmpty()) return;
        exec("select", QJsonObject{{"args", m}});
        exec("uvSnapshot", QJsonObject{{"name", f}, {"xResolution", 2048}, {"yResolution", 2048}});
    });

    // Textures: the checker, and the textures on the meshes shown, the first shown behind the UVs.
    QMenu* textures = bar->addMenu("Textures");
    connect(textures, &QMenu::aboutToShow, this, [this, textures, meshes] {
        textures->clear();
        QAction* chk = textures->addAction("Checker Map");
        chk->setCheckable(true);
        chk->setChecked(canvas_->show_checker);
        connect(chk, &QAction::toggled, this, [this](bool on) {
            canvas_->show_checker = on;
            canvas_->update();
            exec_quiet("uiPref", QJsonObject{{"args", QJsonArray{"uvChecker", on}}});
            Host::get()->poke();
        });
        textures->addSeparator();
        const QJsonArray m = meshes();
        QStringList shaders;
        for (const QJsonValue& mesh : m) {
            const QJsonArray sgs = ask("listConnections", QJsonObject{{"args", QJsonArray{mesh}}, {"type", "shadingEngine"}}).toArray();
            for (const QJsonValue& sg : sgs) {
                const QJsonArray mats = ask("listConnections", QJsonObject{{"args", QJsonArray{sg.toString() + ".surfaceShader"}}}).toArray();
                for (const QJsonValue& mat : mats)
                    if (!shaders.contains(mat.toString())) shaders << mat.toString();
            }
        }
        if (shaders.isEmpty()) textures->addAction("No textures")->setEnabled(false);
        for (const QString& sh : shaders) textures->addAction(sh)->setEnabled(false);
    });

    // UV Sets: the sets of the mesh worked on.
    QMenu* sets = bar->addMenu("UV Sets");
    connect(sets, &QMenu::aboutToShow, this, [this, sets, meshes] {
        sets->clear();
        sets->addAction("UV Set Editor", [] { mel_action("UVSetEditor;"); });
        const QJsonArray m = meshes();
        const QString mesh = m.isEmpty() ? QString() : m.first().toString();
        QStringList names;
        QString cur;
        if (!mesh.isEmpty()) {
            for (const QJsonValue& v : ask("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"query", true}, {"allUVSets", true}}).toArray()) names << v.toString();
            const QJsonArray c = ask("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"query", true}, {"currentUVSet", true}}).toArray();
            cur = c.isEmpty() ? QString() : c.first().toString();
        }
        QMenu* copy = sets->addMenu("Copy UVs to UV Set");
        for (const QString& n : names) {
            if (n == cur) continue;
            copy->addAction(n, [mesh, cur, n] {
                // Onto set n: it becomes current for the copy, then the one worked on is current again.
                exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"currentUVSet", true}, {"uvSet", n}});
                exec("polyCopyUV", QJsonObject{{"args", QJsonArray{mesh}}, {"uvSetNameInput", cur}});
                exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"currentUVSet", true}, {"uvSet", cur}});
                Host::get()->poke();
            });
        }
        copy->addAction("Copy into New UV Set", [mesh, cur] {
            exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"copy", true}, {"uvSet", cur}});
            Host::get()->poke();
        });
        sets->addAction("Create Empty UV Set", [mesh] {
            if (mesh.isEmpty()) return;
            exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"create", true}});
            Host::get()->poke();
        });
        sets->addAction("Delete Current UV Set", [mesh, cur] {
            if (mesh.isEmpty()) return;
            exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"delete", true}, {"uvSet", cur}});
            Host::get()->poke();
        });
        sets->addAction("Rename Current UV Set...", [this, mesh, cur] {
            if (mesh.isEmpty()) return;
            bool ok = false;
            const QString to = QInputDialog::getText(this, "Rename UV Set", "New name:", QLineEdit::Normal, cur, &ok);
            if (!ok || to.isEmpty() || to == cur) return;
            exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"rename", true}, {"uvSet", cur}, {"newUVSet", to}});
            Host::get()->poke();
        });
        sets->addSeparator();
        sets->addSection("Current UV Set");
        auto* group = new QActionGroup(sets);
        for (const QString& n : names) {
            QAction* a = sets->addAction(n, [mesh, n] {
                exec("polyUVSet", QJsonObject{{"args", QJsonArray{mesh}}, {"currentUVSet", true}, {"uvSet", n}});
                Host::get()->poke();
            });
            a->setCheckable(true);
            a->setChecked(n == cur);
            group->addAction(a);
        }
    });

    QMenu* help = bar->addMenu("Help");
    help->addAction("Help on UV Editor", [] { QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek#readme")); });
    help->addSeparator();
    help->addAction("MEL Command Reference", [] { QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek/blob/main/docs/COMMANDS.md")); });
    help->addAction("Python Command Reference", [] { QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek/blob/main/docs/COMMANDS.md")); });
    return bar;
}

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
    auto by_type = [this](const QString& type, const QString& mode) {
        return [this, type, mode] {
            QJsonArray meshes;
            for (const QString& m : canvas_->mesh_paths()) meshes.append(m);
            if (meshes.isEmpty()) return;
            const QJsonArray got = ask("texSelectByType", QJsonObject{{"args", meshes}, {"type", type}}).toArray();
            exec("select", got.isEmpty() ? QJsonObject{{"args", meshes}} : QJsonObject{{"args", got}});
            canvas_->set_mode(mode);
            Host::get()->poke();
        };
    };
    buttons(fold(page, "Select By Type", w), {{"Back-Facing", by_type("backFacing", "face")},
                                              {"Front-Facing", by_type("frontFacing", "face")},
                                              {"Overlapping", select_faces("polyUVOverlap", {{"overlappingComponents", true}})},
                                              {"Non-Overlapping", select_faces("polyUVOverlap", {{"nonOverlappingComponents", true}})},
                                              {"Texture Borders", by_type("textureBorders", "uv")},
                                              {"Unmapped", by_type("unmapped", "face")}});

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
        no_stray_wheel(mode);
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

    // Transform, as the reference's: the pivot (nine places on the selection or the UV area, or one put with Edit
    // Pivot), move with its step snap and Retain component spacing, distribute, rotate and scale with their step
    // snaps, Prevent Negative Scale, flip.
    {
        QWidget* body = fold(page, "Transform", w);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 2, 0, 4);
        v->setSpacing(3);
        // Pivot.
        auto* piv = new QHBoxLayout();
        piv->addWidget(new QLabel("Pivot", body));
        auto* grid9 = new QGridLayout();
        grid9->setSpacing(2);
        auto* where = new QButtonGroup(body);
        for (int i = 0; i < 9; ++i) {
            auto* r = new QRadioButton(body);
            r->setChecked(i == 4);
            where->addButton(r, i);
            grid9->addWidget(r, i / 3, i % 3);
        }
        piv->addLayout(grid9);
        auto* about = new QVBoxLayout();
        auto* on_sel = new QRadioButton("Selection", body);
        auto* on_area = new QRadioButton("UV Area:", body);
        on_sel->setChecked(true);
        auto* area = new QHBoxLayout();
        auto* au0 = number(0.0, -1000, 1000, body);
        auto* av0 = number(0.0, -1000, 1000, body);
        au0->setDecimals(0);
        av0->setDecimals(0);
        au0->setToolTip("The tile's U");
        av0->setToolTip("The tile's V");
        area->addWidget(new QLabel("U:", body));
        area->addWidget(au0);
        area->addWidget(new QLabel("V:", body));
        area->addWidget(av0);
        about->addWidget(on_sel);
        about->addWidget(on_area);
        about->addLayout(area);
        piv->addLayout(about);
        v->addLayout(piv);
        auto* pivot_buttons = new QHBoxLayout();
        auto* edit_pivot = new QPushButton("Edit Pivot", body);
        edit_pivot->setToolTip("Click in the editor to put the pivot (Insert)");
        auto* reset_pivot = new QPushButton("Reset", body);
        connect(edit_pivot, &QPushButton::clicked, this, [this] { canvas_->editing_pivot = true; });
        connect(reset_pivot, &QPushButton::clicked, this, [this] {
            canvas_->has_pivot = false;
            canvas_->editing_pivot = false;
            canvas_->update();
        });
        pivot_buttons->addWidget(edit_pivot);
        pivot_buttons->addWidget(reset_pivot);
        v->addLayout(pivot_buttons);
        // The pivot now: the one put by hand, or the place picked on the selection's bounds or the tile.
        auto pivot = [this, where, on_area, au0, av0]() -> QPointF {
            if (canvas_->has_pivot) return canvas_->pivot_uv;
            const int k = where->checkedId() < 0 ? 4 : where->checkedId();
            QRectF b = on_area->isChecked() ? QRectF(au0->value(), av0->value(), 1, 1) : canvas_->selection_bounds();
            if (b.isNull() && !on_area->isChecked()) return QPointF(-1e9, -1e9);
            const double x = b.left() + b.width() * (k % 3) * 0.5;
            const double y = b.bottom() - b.height() * (k / 3) * 0.5;
            return QPointF(x, y);
        };
        // A step snap: Off, Absolute (to multiples of the step) or Relative (by multiples of it).
        auto step_row = [this, body](QBoxLayout* into, double def) {
            auto* row = new QHBoxLayout();
            auto* mode = new QComboBox(body);
            no_stray_wheel(mode);
            mode->addItems({"Off", "Absolute", "Relative"});
            auto* step = number(def, 0.0, 1000, body);
            row->addWidget(new QLabel("Step Snap:", body));
            row->addWidget(mode, 1);
            row->addWidget(step);
            into->addLayout(row);
            return std::pair{mode, step};
        };
        auto snapped = [](double x, QComboBox* mode, QDoubleSpinBox* step) {
            const double s = step->value();
            if (mode->currentIndex() == 0 || s <= 0) return x;
            return std::round(x / s) * s;
        };
        // Move.
        auto* g = new QGridLayout();
        g->setSpacing(3);
        auto* mu = number(0.0, -1000, 1000, body);
        auto* mv = number(0.0, -1000, 1000, body);
        auto* step = number(0.1, 0, 1000, body);
        g->addWidget(new QLabel("Move U", body), 0, 0);
        g->addWidget(mu, 0, 1);
        g->addWidget(new QLabel("V", body), 0, 2);
        g->addWidget(mv, 0, 3);
        auto* go = new QPushButton("Move", body);
        g->addWidget(go, 0, 4);
        v->addLayout(g);
        auto* nudges = new QHBoxLayout();
        QList<std::tuple<QPushButton*, int, int>> nudge_buttons;
        for (auto [label, du, dv] : {std::tuple{"← U", -1, 0}, {"U →", 1, 0}, {"↓ V", 0, -1}, {"V ↑", 0, 1}}) {
            auto* b = new QPushButton(label, body);
            nudge_buttons.append({b, du, dv});
            nudges->addWidget(b);
        }
        nudges->addWidget(new QLabel("by", body));
        nudges->addWidget(step);
        v->addLayout(nudges);
        auto [move_mode, move_step] = step_row(v, 0.1);
        auto* retain = new QCheckBox("Retain component spacing", body);
        retain->setChecked(true);
        v->addWidget(retain);
        // A move by (du, dv): with Absolute snap, onto multiples of the step, the selection as a whole (its bounds' corner)
        // when spacing is kept, each UV on its own when it isn't; with Relative, by whole steps.
        auto move_by = [this, move_mode, move_step, retain](double du, double dv) {
            const double s = move_step->value();
            const int m = move_mode->currentIndex();
            if (m == 2 && s > 0) {
                du = std::round(du / s) * s;
                dv = std::round(dv / s) * s;
            }
            run("polyEditUV", {{"u", du}, {"v", dv}});
            if (m == 1 && s > 0) {
                if (retain->isChecked()) {
                    const QRectF b = canvas_->selection_bounds();
                    if (!b.isNull()) {
                        const double x = std::round(b.left() / s) * s - b.left(), y = std::round(b.top() / s) * s - b.top();
                        if (std::abs(x) > 1e-12 || std::abs(y) > 1e-12) run("polyEditUV", {{"u", x}, {"v", y}});
                    }
                } else {
                    run("texMatchGrid", {{"gridU", 1.0 / s}, {"gridV", 1.0 / s}});
                }
            }
            Host::get()->poke();
        };
        connect(go, &QPushButton::clicked, this, [move_by, mu, mv] { move_by(mu->value(), mv->value()); });
        for (auto& [b, du, dv] : nudge_buttons) {
            const int u = du, vv = dv;
            connect(b, &QPushButton::clicked, this, [move_by, step, u, vv] { move_by(u * step->value(), vv * step->value()); });
        }
        auto* dist = new QHBoxLayout();
        auto* dist_btn = new QPushButton("Distribute", body);
        dist_btn->setToolTip("Space the selected UVs evenly along U or V");
        auto* du_b = new QPushButton("U", body);
        auto* dv_b = new QPushButton("V", body);
        connect(du_b, &QPushButton::clicked, this, cmd("texDistributeUVs", {{"direction", "u"}}));
        connect(dv_b, &QPushButton::clicked, this, cmd("texDistributeUVs", {{"direction", "v"}}));
        connect(dist_btn, &QPushButton::clicked, this, cmd("texDistributeUVs", {{"direction", "u"}}));
        dist->addWidget(dist_btn, 1);
        dist->addWidget(du_b);
        dist->addWidget(dv_b);
        v->addLayout(dist);
        // Rotate.
        auto* rot = new QHBoxLayout();
        auto* angle = number(90.0, -360, 360, body);
        auto* ccw = new QPushButton("↺", body);
        auto* cw = new QPushButton("↻", body);
        ccw->setToolTip("Turn counter-clockwise");
        cw->setToolTip("Turn clockwise");
        rot->addWidget(new QLabel("Rotate", body));
        rot->addWidget(angle, 1);
        rot->addWidget(ccw);
        rot->addWidget(cw);
        v->addLayout(rot);
        auto [rot_mode, rot_step] = step_row(v, 15.0);
        auto turn = [this, pivot, angle, rot_mode, rot_step, snapped](double sign) {
            const QPointF c = pivot();
            if (c.x() < -1e8) return;
            const double a = sign * snapped(angle->value(), rot_mode, rot_step);
            QJsonObject call{{"angle", a}, {"pivotU", c.x()}, {"pivotV", c.y()}};
            const QJsonArray t = canvas_->targets();
            if (!t.isEmpty()) call["args"] = t;
            exec("polyEditUV", call);
            Host::get()->poke();
        };
        connect(ccw, &QPushButton::clicked, this, [turn] { turn(1.0); });
        connect(cw, &QPushButton::clicked, this, [turn] { turn(-1.0); });
        // Scale.
        auto* sc = new QHBoxLayout();
        auto* factor = number(2.0, -1000, 1000, body);
        auto* su = new QPushButton("U", body);
        auto* sv = new QPushButton("V", body);
        su->setCheckable(true);
        sv->setCheckable(true);
        su->setChecked(true);
        sv->setChecked(true);
        auto* scale = new QPushButton("Scale", body);
        sc->addWidget(new QLabel("Scale", body));
        sc->addWidget(factor, 1);
        sc->addWidget(su);
        sc->addWidget(sv);
        sc->addWidget(scale);
        v->addLayout(sc);
        auto [scale_mode, scale_step] = step_row(v, 1.0);
        auto* no_negative = new QCheckBox("Prevent Negative Scale", body);
        no_negative->setChecked(true);
        v->addWidget(no_negative);
        connect(scale, &QPushButton::clicked, this, [this, pivot, factor, su, sv, no_negative, scale_mode, scale_step, snapped] {
            const QPointF c = pivot();
            if (c.x() < -1e8) return;
            double f = snapped(factor->value(), scale_mode, scale_step);
            if (no_negative->isChecked()) f = std::max(f, 1e-4);
            QJsonObject call{{"scaleU", su->isChecked() ? f : 1.0}, {"scaleV", sv->isChecked() ? f : 1.0}, {"pivotU", c.x()}, {"pivotV", c.y()}};
            const QJsonArray t = canvas_->targets();
            if (!t.isEmpty()) call["args"] = t;
            exec("polyEditUV", call);
            Host::get()->poke();
        });
        auto* flips = new QHBoxLayout();
        auto* flip_u = new QPushButton("Flip U", body);
        auto* flip_v = new QPushButton("Flip V", body);
        connect(flip_u, &QPushButton::clicked, this, cmd("polyFlipUV", {{"flipType", 0}}));
        connect(flip_v, &QPushButton::clicked, this, cmd("polyFlipUV", {{"flipType", 1}}));
        flips->addWidget(flip_u);
        flips->addWidget(flip_v);
        v->addLayout(flips);
    }

    // Texel Density: Get reads the selection's, Set scales each shell to it.
    {
        QWidget* body = fold(page, "Texel Density", w);
        auto* g = new QGridLayout(body);
        g->setContentsMargins(0, 2, 0, 4);
        auto* td = number(32.0, 0.001, 100000, body);
        auto* map = new QComboBox(body);
        no_stray_wheel(map);
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
                                      {"Normal-Based", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "n"}})},
                                      {"Cylindrical", cmd("polyProjection", {{"type", "cylindrical"}})},
                                      {"Planar", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}})},
                                      {"Spherical", cmd("polyProjection", {{"type", "spherical"}})},
                                      {"Best Plane", cmd("polyProjection", {{"type", "planar"}, {"mapDirection", "b"}})},
                                      {"Camera-Based", [this] { camera_projection(); }},
                                      {"Contour Stretch", cmd("polyContourProjection")}});

    buttons(fold(page, "Cut and Sew", w), {{"Auto-Seams", cmd("u3dAutoSeam", {{"splitShells", 0.5}})},
                                           {"Split", [] { mel_action("SplitUV;"); }},
                                           {"Cut", cmd("polyMapCut")},
                                           {"Cut Tool", [this] { canvas_->set_tool("cut"); }},
                                           {"Create UV Shell", cmd("texCreateShell")},
                                           {"Create Shell (Grid)", cmd("texCreateShell", {{"grid", true}})},
                                           {"Sew", cmd("polyMapSew")},
                                           {"Sew Tool", [this] { canvas_->set_tool("sew"); }},
                                           {"Stitch Together", cmd("polyMapSewMove")}});

    {
        QWidget* body = fold(page, "Unfold", w);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 2, 0, 4);
        auto* top = new QWidget(body);
        v->addWidget(top);
        buttons(top, {{"Optimize", cmd("u3dOptimize")},
                      {"Optimize Tool", [this] { canvas_->set_tool("brush:optimize"); }},
                      {"Unfold", cmd("u3dUnfold")},
                      {"Unfold Tool", [this] { canvas_->set_tool("brush:unfold"); }},
                      {"Straighten Border", cmd("polyStraightenUVBorder")},
                      {"Straighten Shell", cmd("texStraightenShell")},
                      {"Rectangle", cmd("polyUVRectangle")}});
        auto* along = new QHBoxLayout();
        auto* label = new QPushButton("Unfold Along", body);
        label->setStyleSheet("QPushButton { text-align: left; padding: 3px 6px; }");
        auto* au = new QPushButton("U", body);
        auto* av = new QPushButton("V", body);
        au->setCheckable(true);
        av->setCheckable(true);
        au->setChecked(true);
        auto* uv_group = new QButtonGroup(body);
        uv_group->addButton(au);
        uv_group->addButton(av);
        connect(label, &QPushButton::clicked, this, [this, au] {
            run("unfold", {{"optimizeAxis", au->isChecked() ? 1 : 2}});
            Host::get()->poke();
        });
        along->addWidget(label, 1);
        along->addWidget(au);
        along->addWidget(av);
        v->addLayout(along);
        auto* st = new QHBoxLayout();
        auto* sbtn = new QPushButton("Straighten UVs", body);
        sbtn->setStyleSheet("QPushButton { text-align: left; padding: 3px 6px; }");
        auto* sang = number(30.0, 0.0, 90.0, body);
        sang->setDecimals(2);
        auto* su = new QPushButton("U", body);
        auto* sv = new QPushButton("V", body);
        su->setCheckable(true);
        sv->setCheckable(true);
        su->setChecked(true);
        sv->setChecked(true);
        connect(sbtn, &QPushButton::clicked, this, [this, sang, su, sv] {
            run("texStraightenUVs", {{"angle", sang->value()}, {"u", su->isChecked()}, {"v", sv->isChecked()}});
            Host::get()->poke();
        });
        st->addWidget(sbtn, 1);
        st->addWidget(sang);
        st->addWidget(su);
        st->addWidget(sv);
        v->addLayout(st);
    }

    // Align and Snap: shells lined up on the outermost one, UVs on a line, shells to a place on the tile.
    {
        QWidget* body = fold(page, "Align and Snap", w);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 2, 0, 4);
        auto* row = new QHBoxLayout();
        row->addWidget(new QLabel("Align", body));
        for (auto [label, to, tip] : {std::tuple{"⇤", "left", "Align shells left"}, {"↔", "centerU", "Align shells to their middle in U"},
                                      {"⇥", "right", "Align shells right"}, {"⤒", "top", "Align shells to the top"},
                                      {"↕", "centerV", "Align shells to their middle in V"}, {"⤓", "bottom", "Align shells to the bottom"}}) {
            auto* b = new QToolButton(body);
            b->setText(label);
            b->setToolTip(tip);
            connect(b, &QToolButton::clicked, this, cmd("texAlignShells", {{"alignTo", to}}));
            row->addWidget(b);
        }
        row->addStretch(1);
        v->addLayout(row);
        auto* more = new QWidget(body);
        v->addWidget(more);
        buttons(more, {{"Linear Align", cmd("texLinearAlignUVs")},
                       {"Match UVs", cmd("texMatchUVs", {{"tolerance", 0.01}})},
                       {"Match Grid", cmd("texMatchGrid", {{"gridU", 16.0}, {"gridV", 16.0}})},
                       {"Snap Together", [this] { snap_together(false); }},
                       {"Snap Together (B to A)", [this] { snap_together(true); }},
                       {"Snap and Stack", cmd("texSnapStackShells")},
                       {"Normalize U", cmd("polyNormalizeUV", {{"normalizeType", 1}, {"normalizeDirection", 1}})},
                       {"Normalize V", cmd("polyNormalizeUV", {{"normalizeType", 1}, {"normalizeDirection", 2}})}});
        // Snap: nine buttons, the places on the 0 to 1 tile.
        auto* snap = new QHBoxLayout();
        snap->addWidget(new QLabel("Snap", body));
        auto* grid = new QGridLayout();
        grid->setSpacing(2);
        static const char* where[] = {"top left", "top", "top right", "left", "middle", "right", "bottom left", "bottom", "bottom right"};
        for (int i = 0; i < 9; ++i) {
            auto* b = new QToolButton(body);
            b->setFixedSize(22, 22);
            b->setText(i == 4 ? "●" : "□");
            b->setToolTip(QString("Snap shells to the %1 of the 0 to 1 tile").arg(where[i]));
            connect(b, &QToolButton::clicked, this, cmd("texSnapShells", {{"position", i}}));
            grid->addWidget(b, i / 3, i % 3);
        }
        snap->addLayout(grid);
        snap->addStretch(1);
        v->addLayout(snap);
    }

    // Arrange and Layout.
    {
        QWidget* body = fold(page, "Arrange and Layout", w);
        auto* v = new QVBoxLayout(body);
        v->setContentsMargins(0, 2, 0, 4);
        auto* dist = new QHBoxLayout();
        dist->addWidget(new QLabel("Distribute", body));
        auto* gap = number(0.0, 0.0, 10.0, body);
        gap->setToolTip("Gap between shells, in UV units");
        dist->addWidget(gap);
        for (auto [label, dir] : {std::pair{"U", "u"}, {"V", "v"}}) {
            auto* b = new QPushButton(label, body);
            b->setToolTip(QString("Space the shells along %1, the gap apart").arg(label));
            const QString d = dir;
            connect(b, &QPushButton::clicked, this, [this, gap, d] {
                run("texDistributeShells", {{"direction", d}, {"spacing", gap->value()}});
                Host::get()->poke();
            });
            dist->addWidget(b);
        }
        auto* target = new QPushButton("Target", body);
        target->setToolTip("Space the shells evenly toward the last one picked");
        connect(target, &QPushButton::clicked, this, cmd("texDistributeShellsToTarget"));
        dist->addWidget(target);
        v->addLayout(dist);
        auto* holder = new QWidget(body);
        v->addWidget(holder);
        buttons(holder, {{"Orient Shells", cmd("texOrientShells")},
                         {"Orient to Edges", cmd("texOrientEdge")},
                         {"Stack Shells", cmd("texStackShells")},
                         {"Unstack Shells", cmd("texUnstackShells")},
                         {"Stack and Orient", [this] {
                              run("texOrientShells");
                              run("texStackShells");
                              Host::get()->poke();
                          }},
                         {"Stack Similar", cmd("polyUVStackSimilarShells")},
                         {"Gather Shells", cmd("texGatherShells")},
                         {"Randomize Shells", cmd("texRandomizeShells", {{"translate", 0.1}, {"rotate", 15.0}, {"scale", 10.0}, {"seed", 1}})},
                         {"Layout", cmd("u3dLayout")},
                         {"Normalize", cmd("polyNormalizeUV", {{"normalizeType", 1}})},
                         {"Layout Along U", cmd("texLayoutAlong", {{"direction", "u"}})},
                         {"Layout Along V", cmd("texLayoutAlong", {{"direction", "v"}})}});
        // Measure: between the two UVs picked.
        auto* measure = new QHBoxLayout();
        auto* mbtn = new QPushButton("Measure", body);
        auto* mkind = new QComboBox(body);
        no_stray_wheel(mkind);
        mkind->addItems({"U Distance", "V Distance", "Pixel Distance", "Angle Between"});
        mkind->setCurrentIndex(2);
        auto* mout = new QLineEdit(body);
        mout->setReadOnly(true);
        connect(mbtn, &QPushButton::clicked, this, [this, mkind, mout] {
            const QJsonArray t = canvas_->targets();
            QJsonArray uvs;
            if (!t.isEmpty())
                for (const QJsonValue& x : ask("ls", QJsonObject{{"args", t}, {"flatten", true}}).toArray())
                    if (x.toString().contains(".map[")) uvs.append(x);
            if (uvs.size() < 2) {
                log("// Warning: Measure: pick two UVs //", 4);
                return;
            }
            const QJsonArray p = ask("polyEditUV", QJsonObject{{"args", QJsonArray{uvs[0]}}, {"query", true}}).toArray();
            const QJsonArray q = ask("polyEditUV", QJsonObject{{"args", QJsonArray{uvs[uvs.size() - 1]}}, {"query", true}}).toArray();
            if (p.size() < 2 || q.size() < 2) return;
            const double du = q[0].toDouble() - p[0].toDouble(), dv = q[1].toDouble() - p[1].toDouble();
            QString out;
            switch (mkind->currentIndex()) {
            case 0: out = QString::number(std::abs(du), 'f', 4); break;
            case 1: out = QString::number(std::abs(dv), 'f', 4); break;
            case 2: {
                QStringList l;
                for (int px : {512, 1024, 2048, 4096}) l << QString("%1: %2").arg(px).arg(std::hypot(du, dv) * px, 0, 'f', 1);
                out = l.join("  ");
                break;
            }
            default: out = QString::number(std::atan2(dv, du) * 180.0 / M_PI, 'f', 2) + QString::fromUtf8("\u00b0");
            }
            mout->setText(out);
            mout->setCursorPosition(0);
        });
        measure->addWidget(mbtn);
        measure->addWidget(mkind, 1);
        v->addLayout(measure);
        v->addWidget(mout);
    }

    page->addStretch(1);
    scroll->setWidget(w);
    return scroll;
}

} // namespace az

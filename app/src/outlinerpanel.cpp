// The Outliner: the DAG with an icon per type, Display and Show menus, search, drag to parent, middle-drag to reorder.
#include <QPainter>
#include "panels.h"
#include <functional>
#include "host.h"
#include "usdui.h"
#include "icons.h"

#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QContextMenuEvent>
#include <QDrag>
#include <QSet>
#include <QDropEvent>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QJsonDocument>
#include <QLineEdit>
#include <QMenu>
#include <algorithm>
#include <QKeySequence>
#include <QUrl>
#include <QDesktopServices>
#include <QInputDialog>
#include <QMimeData>
#include <QMouseEvent>
#include <QScrollBar>
#include <QToolButton>
#include <QTimer>
#include <QTreeWidget>
#include <QVBoxLayout>

namespace az {

namespace {

// A tint picked for a dark list, kept readable on a light one (#115): the same hue, dark enough for the list's base.
QColor readable(const QColor& c) {
    const QColor base = qApp->palette().color(QPalette::Base);
    if (base.lightnessF() < 0.5) return c;
    QColor h = c.toHsl();
    const double sat = h.hslSaturationF();
    return QColor::fromHslF(h.hslHueF() < 0 ? 0 : h.hslHueF(), sat < 0.05 ? 0 : std::min(1.0, sat + 0.25), sat < 0.05 ? 0.38 : 0.3);
}

} // namespace

namespace {

QJsonObject args(const QJsonArray& a) {
    QJsonObject o;
    o["args"] = a;
    return o;
}

// As the reference's Outliner (#101): a left drag selects the rows it passes over; a middle drag moves the selection
// (or the row pressed, when it isn't selected), the selection left as it was. Dropping on an item parents to it, on the
// space between items reorders under the same parent, on empty space parents to the world. The parents of selected
// rows are faintly highlighted, and the right button opens the menu as it goes down, for a press-drag-release pick.
class OutlinerTree : public QTreeWidget {
public:
    using QTreeWidget::QTreeWidget;
    // F: the selected items opened out to and scrolled into view, as the reference's Outliner frames.
    void frame_selected() {
        QList<QTreeWidgetItem*> sel = selectedItems();
        if (sel.isEmpty()) return;
        for (QTreeWidgetItem* it : sel)
            for (QTreeWidgetItem* up = it->parent(); up; up = up->parent()) up->setExpanded(true);
        scrollToItem(sel.first(), QAbstractItemView::PositionAtCenter);
        setCurrentItem(sel.first(), 0, QItemSelectionModel::NoUpdate);
    }
    // Shift on an item's arrow: everything under it opens or closes with it.
    static void set_expanded_all(QTreeWidgetItem* it, bool open) {
        it->setExpanded(open);
        for (int i = 0; i < it->childCount(); ++i) set_expanded_all(it->child(i), open);
    }
protected:
    void keyPressEvent(QKeyEvent* e) override {
        if (e->key() == Qt::Key_F && !(e->modifiers() & (Qt::ControlModifier | Qt::AltModifier))) {
            frame_selected();
            return;
        }
        // Shift with Left or Right: the whole hierarchy under the current item closes or opens.
        if ((e->key() == Qt::Key_Left || e->key() == Qt::Key_Right) && (e->modifiers() & Qt::ShiftModifier) && currentItem()) {
            set_expanded_all(currentItem(), e->key() == Qt::Key_Right);
            return;
        }
        QTreeWidget::keyPressEvent(e);
    }
    void mousePressEvent(QMouseEvent* e) override {
        // A click on the arrow with Shift held opens or closes the whole hierarchy under it.
        if (e->button() == Qt::LeftButton && (e->modifiers() & Qt::ShiftModifier)) {
            const QPoint at = e->position().toPoint();
            if (QTreeWidgetItem* it = itemAt(at); it && it->childCount() > 0) {
                const QRect r = visualItemRect(it);
                if (at.x() < r.left() && at.x() >= r.left() - indentation()) {
                    set_expanded_all(it, !it->isExpanded());
                    return;
                }
            }
        }
        if (e->button() == Qt::MiddleButton) {
            if (QTreeWidgetItem* it = itemAt(e->position().toPoint())) {
                QStringList moving;
                if (it->isSelected())
                    for (QTreeWidgetItem* s : selectedItems()) moving << s->data(0, Qt::UserRole).toString();
                else
                    moving << it->data(0, Qt::UserRole).toString();
                auto* drag = new QDrag(this);
                auto* m = new QMimeData;
                m->setData(kMoving, moving.join('\n').toUtf8());
                m->setText(moving.join(' '));
                drag->setMimeData(m);
                drag->exec(Qt::MoveAction);
            }
            return;
        }
        if (e->button() == Qt::RightButton) {
            emit customContextMenuRequested(e->position().toPoint());
            return;
        }
        QTreeWidget::mousePressEvent(e);
    }
    // The menu comes up on the press (above); the release's context menu event, on systems that send one, is dropped.
    bool viewportEvent(QEvent* e) override {
        if (e->type() == QEvent::ContextMenu && static_cast<QContextMenuEvent*>(e)->reason() == QContextMenuEvent::Mouse) return true;
        return QTreeWidget::viewportEvent(e);
    }
    void selectionChanged(const QItemSelection& selected, const QItemSelection& deselected) override {
        QTreeWidget::selectionChanged(selected, deselected);
        parents_.clear();
        for (QTreeWidgetItem* it : selectedItems())
            for (QTreeWidgetItem* up = it->parent(); up; up = up->parent()) parents_.insert(up);
        viewport()->update();
    }
    void drawRow(QPainter* p, const QStyleOptionViewItem& o, const QModelIndex& index) const override {
        QTreeWidgetItem* it = itemFromIndex(index);
        if (it && !it->isSelected() && parents_.contains(it)) {
            QColor c = palette().color(QPalette::Highlight);
            c.setAlpha(55);
            p->fillRect(o.rect, c);
        }
        QTreeWidget::drawRow(p, o, index);
    }
    void dragEnterEvent(QDragEnterEvent* e) override { e->acceptProposedAction(); }
    void dragMoveEvent(QDragMoveEvent* e) override {
        QTreeWidget::dragMoveEvent(e);
        e->acceptProposedAction();
    }
    void dropEvent(QDropEvent* e) override {
        QPoint at = e->position().toPoint();
        QTreeWidgetItem* target = itemAt(at);
        QJsonArray names;
        if (e->mimeData() && e->mimeData()->hasFormat(kMoving)) {
            for (const QString& n : QString::fromUtf8(e->mimeData()->data(kMoving)).split('\n', Qt::SkipEmptyParts)) names.append(n);
        } else {
            for (auto* it : selectedItems()) names.append(it->data(0, Qt::UserRole).toString());
        }
        e->setDropAction(Qt::IgnoreAction);
        e->accept();
        if (names.isEmpty()) return;
        // Near a row's edge: between rows, a reorder among the target's siblings.
        bool between = false;
        if (target) {
            QRect r = visualItemRect(target);
            between = at.y() - r.top() < 4 || r.bottom() - at.y() < 4;
        }
        if (between) {
            QTreeWidgetItem* parent = target->parent();
            QString want = parent ? parent->data(0, Qt::UserRole).toString() : QString();
            QStringList selected_paths;
            for (const QJsonValue& n : names) selected_paths << n.toString();
            // Under the target's parent first, if they aren't yet.
            auto parent_of = [](const QString& p) { return p.left(qMax(0, int(p.lastIndexOf('|')))); };
            bool moved = false;
            for (const QString& p : selected_paths)
                if (parent_of(p) != want) moved = true;
            if (moved) {
                if (want.isEmpty()) {
                    QJsonObject c = args(names);
                    c["world"] = true;
                    exec("parent", c);
                } else {
                    QJsonArray a = names;
                    a.append(want);
                    exec("parent", args(a));
                }
            }
            if (has_reorder()) {
                int to = parent ? parent->indexOfChild(target) : indexOfTopLevelItem(target);
                for (const QJsonValue& n : names) {
                    QString leaf = n.toString().section('|', -1);
                    exec("reorder", QJsonObject{{"args", QJsonArray{want.isEmpty() ? "|" + leaf : want + "|" + leaf}}, {"front", false}, {"relative", to}});
                }
            }
            return;
        }
        if (target) {
            names.append(target->data(0, Qt::UserRole).toString());
            exec("parent", args(names));
        } else {
            QJsonObject call = args(names);
            call["world"] = true;
            exec("parent", call);
        }
    }
private:
    static constexpr const char* kMoving = "application/x-aztek-outliner-paths";
    QSet<QTreeWidgetItem*> parents_;
    static bool has_reorder() {
        for (const QJsonValue& c : query("commands").toArray())
            if (c.toObject()["name"].toString() == "reorder") return true;
        return false;
    }
};

const char* kTypes[][2] = {{"Joints", "joint"}, {"Polygons", "mesh"}, {"Curves", "nurbsCurve"}, {"Locators", "locator"}, {"Cameras", "camera"}, {"Groups", "group"}, {"IK Handles", "ik"}, {"Deformers", "deformHandle"}, {"Sets", "set"}};

} // namespace

Outliner::Outliner(QWidget* parent) : Panel(parent) {
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(2, 2, 2, 2);
    lay->setSpacing(2);
    auto* top = new QHBoxLayout();
    auto toggle = [this](QMenu* m, const QString& label, bool& flag) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(flag);
        bool* p = &flag;
        connect(a, &QAction::toggled, this, [this, p](bool on) {
            *p = on;
            refresh();
        });
        return a;
    };
    // Display, Show and Help, as the reference's Outliner has them.
    auto* display = new QToolButton(this);
    display->setText("Display");
    display->setPopupMode(QToolButton::InstantPopup);
    auto* menu = new QMenu(display);
    connect(menu, &QMenu::aboutToShow, this, [this, menu] { fill_display_menu(menu); });
    display->setMenu(menu);
    top->addWidget(display);
    auto* show = new QToolButton(this);
    show->setText("Show");
    show->setPopupMode(QToolButton::InstantPopup);
    auto* show_menu = new QMenu(show);
    connect(show_menu, &QMenu::aboutToShow, this, [this, show_menu] { fill_show_menu(show_menu); });
    show->setMenu(show_menu);
    top->addWidget(show);
    auto* help = new QToolButton(this);
    help->setText("Help");
    help->setPopupMode(QToolButton::InstantPopup);
    auto* help_menu = new QMenu(help);
    connect(help_menu, &QMenu::aboutToShow, this, [this, help_menu] { fill_help_menu(help_menu); });
    help->setMenu(help_menu);
    top->addWidget(help);
    Q_UNUSED(toggle);
    filter_ = new QLineEdit(this);
    filter_->setPlaceholderText("Search...");
    filter_->setClearButtonEnabled(true);
    connect(filter_, &QLineEdit::textChanged, this, [this] { refresh(); });
    top->addWidget(filter_, 1);
    lay->addLayout(top);

    tree_ = new OutlinerTree(this);
    tree_->setHeaderHidden(true);
    tree_->setSelectionMode(QAbstractItemView::ExtendedSelection);
    // Drops only: a left drag selects; the middle button starts the drags (OutlinerTree).
    tree_->setDragDropMode(QAbstractItemView::DropOnly);
    tree_->setDragEnabled(false);
    tree_->setEditTriggers(QAbstractItemView::DoubleClicked | QAbstractItemView::EditKeyPressed);
    tree_->setIconSize(QSize(16, 16));
    tree_->setIndentation(14);
    tree_->setUniformRowHeights(true);
    tree_->setObjectName("outliner");
    lay->addWidget(tree_, 1);
    connect(tree_, &QTreeWidget::itemSelectionChanged, this, &Outliner::selection_changed);
    connect(tree_, &QTreeWidget::itemExpanded, this, [this](QTreeWidgetItem* it) {
        const QString path = it->data(0, Qt::UserRole).toString();
        expanded_.insert(path);
        // Prims are listed when asked for: a stage can hold far more than a tree should build.
        if (!updating_ && it->childCount() == 1 && it->child(0)->data(0, Qt::UserRole).toString().isEmpty()) QTimer::singleShot(0, this, [this] {
            last_shape_.clear();
            refresh();
        });
    });
    connect(tree_, &QTreeWidget::itemCollapsed, this, [this](QTreeWidgetItem* it) { expanded_.remove(it->data(0, Qt::UserRole).toString()); });
    connect(tree_, &QTreeWidget::itemChanged, this, [this](QTreeWidgetItem* it, int) {
        if (updating_) return;
        QString path = it->data(0, Qt::UserRole).toString();
        QString name = it->text(0).trimmed();
        if (!name.isEmpty() && !path.endsWith("|" + name) && !path.endsWith("/" + name) && path != name) exec("rename", args(QJsonArray{path, name}));
    });
    tree_->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(tree_, &QWidget::customContextMenuRequested, this, [this](const QPoint& p) {
        QMenu m;
        if (auto* hit = tree_->itemAt(p)) {
            const QString named = hit->data(0, Qt::UserRole).toString();
            if (named.contains(",/")) {
                usd_prim_menu(&m, named, this);
                m.addSeparator();
            } else if (!named.isEmpty()) {
                usd_node_menu(&m, named);
                m.addSeparator();
            }
        }
        // The reference's Outliner menu, as it has it.
        auto check = [this, &m](const QString& label, bool& flag) {
            QAction* a = m.addAction(label);
            a->setCheckable(true);
            a->setChecked(flag);
            bool* f = &flag;
            connect(a, &QAction::toggled, this, [this, f](bool on) {
                *f = on;
                refresh();
            });
        };
        check("Show DAG Objects Only", dag_only_);
        check("Shapes", shapes_);
        m.addSeparator();
        m.addAction("Reveal Selected", [this] { static_cast<OutlinerTree*>(tree_)->frame_selected(); });
        QMenu* hide = m.addMenu("Hide In Outliner");
        hide->addAction("Hide Selected", [this] {
            for (QTreeWidgetItem* it : tree_->selectedItems()) {
                const QString n = it->data(0, Qt::UserRole).toString();
                if (!n.isEmpty() && !n.contains(",/")) exec("setAttr", QJsonObject{{"args", QJsonArray{n + ".hiddenInOutliner", true}}});
            }
        });
        hide->addAction("Unhide All", [] { mel("for ($n in `ls -dag`) if (`getAttr ($n + \".hiddenInOutliner\")`) setAttr ($n + \".hiddenInOutliner\") 0;"); });
        m.addSeparator();
        QMenu* ref = m.addMenu("Reference");
        ref->addAction("Create Reference...\tCtrl+R", [] { mel("CreateReference;"); });
        m.addMenu("Scene Assembly")->setEnabled(false);
        QMenu* sets = m.addMenu("Sets");
        sets->addAction("Create Set", [] { exec("sets"); });
        sets->addAction("Create Partition")->setEnabled(false);
        sets->addAction("Create Quick Select Set...", [this] {
            bool ok = false;
            const QString n = QInputDialog::getText(this, "Create Quick Select Set", "Name:", QLineEdit::Normal, "set1", &ok);
            if (ok && !n.isEmpty()) exec("createSet", QJsonObject{{"name", n}});
        });
        m.addMenu("Asset")->setEnabled(false);
        m.addSeparator();
        fill_display_menu(m.addMenu("Display"));
        fill_show_menu(m.addMenu("Show"));
        m.addSeparator();
        m.addMenu("Render Setup")->setEnabled(false);
        m.exec(tree_->viewport()->mapToGlobal(p));
    });
    auto* find = new QAction(this);
    find->setShortcut(QKeySequence("Ctrl+F"));
    find->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    connect(find, &QAction::triggered, this, [this] { filter_->setFocus(); filter_->selectAll(); });
    addAction(find);
}

// The reference's Display menu, built when it opens so its ticks are current.
void Outliner::fill_display_menu(QMenu* m) {
    m->clear();
    auto check = [this, m](const QString& label, bool& flag, bool enabled = true) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(flag);
        a->setEnabled(enabled);
        bool* f = &flag;
        connect(a, &QAction::toggled, this, [this, f](bool on) {
            *f = on;
            last_shape_.clear();
            refresh();
        });
        return a;
    };
    // What has no part in Aztek yet shows, greyed, where the reference has it.
    auto later = [m](const QString& label, bool checked = false) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(checked);
        a->setEnabled(false);
    };
    auto radio = [this, m](const QString& title, const QStringList& items, std::function<int()> get, std::function<void(int)> set) {
        QMenu* sub = m->addMenu(title);
        auto* group = new QActionGroup(sub);
        for (int i = 0; i < items.size(); ++i) {
            QAction* a = sub->addAction(items[i]);
            a->setCheckable(true);
            a->setChecked(get() == i);
            group->addAction(a);
            connect(a, &QAction::triggered, this, [this, set, i] {
                set(i);
                last_shape_.clear();
                refresh();
            });
        }
        return sub;
    };
    check("DAG Objects Only", dag_only_);
    check("Shapes", shapes_);
    check("Assigned Materials", materials_);
    later("Time Editor", true);
    later("Reference Nodes", true);
    check("Reference Members", ref_members_);
    check("Attributes (Channels)", attrs_);
    check("Set Members", set_members_);
    check("Connected", connected_);
    check("Show Namespace", namespaces_);
    check("Ignore 'Hidden In Outliner'", ignore_hidden_);
    check("Ignore 'Use Outliner Color'", ignore_color_);
    later("Show Mute Information");
    m->addSeparator();
    later("Organize by Layer", true);
    later("Organize by Clip", true);
    later("Auto-expand Layers", true);
    QMenu* layout = m->addMenu("Window Layout");
    layout->addAction("Split Horizontally")->setEnabled(false);
    layout->addAction("Split Vertically")->setEnabled(false);
    m->addSeparator();
    QMenu* asset = m->addMenu("Advanced Asset Contents");
    for (const char* t : {"Under Asset", "Under Parent", "Both", "None"}) asset->addAction(t)->setEnabled(false);
    const QStringList names{"nice", "long", "short"};
    radio("Channel Names", {"Nice", "Long", "Short"}, [this, names] { return int(names.indexOf(channel_names_)); }, [this, names](int i) { channel_names_ = names[i]; });
    radio("Sort Order", {"Scene Hierarchy", "Alphabetical Within Type"}, [this] { return sort_alpha_ ? 1 : 0; }, [this](int i) { sort_alpha_ = i == 1; });
    m->addSeparator();
    radio("Attribute Order", {"Default", "Alphabetical, ascend", "Alphabetical, descend"}, [this] { return attr_order_; }, [this](int i) { attr_order_ = i; });
    m->addSeparator();
    QMenu* layers = m->addMenu("Animation Layers Filter");
    for (const char* t : {"From Layer Editor Show Options", "All Affecting", "Active", "Selected", "Active and Selected", "Show Layer Weight"}) layers->addAction(t)->setEnabled(false);
    later("Render Setup Filter");
    // Aztek's own, after the reference's.
    m->addSeparator();
    check("Intermediate Objects", intermediates_);
    check("Hide Default Cameras", hide_cameras_);
    check("Show Icons", show_icons_);
}

namespace {
// The Show menu's kinds of object: label, key, and whether Aztek has them yet.
struct ShowKind { const char* label; const char* key; bool here; };
const ShowKind kShowKinds[] = {
    {"Asset", "asset", false}, {"Geometry", "geometry", true}, {"NURBS Objects", "nurbs", true}, {"Polygon Objects", "polygons", true},
    {"Subdiv Objects", "subdiv", false}, {"GPU Cache", "gpuCache", false}, {"Cameras", "cameras", true}, {"Joints", "joints", true},
    {"IkHandles", "ikHandles", true}, {"Characters", "characters", false}, {"Clips", "clips", false}, {"Sets", "sets", true},
    {"Lights", "lights", true}, {"Optical FX", "opticalFx", false}, {"Lights and Light Sets", "lightsAndSets", true}, {"Light Sets", "lightSets", false},
    {"Lights: Illum. Linked Objects", "illumLinked", false}, {"Lights: Illum. by Default", "illumDefault", false}, {"Not In Render Layers", "notInLayers", false},
    {"In Render Layers", "inLayers", false}, {"Not In Selected Render Layers", "notInSelLayers", false}, {"In Selected Render Layers", "inSelLayers", false}, {"Motion Graphics", "mash", false}};
const char* kShowAttrs[][2] = {{"Hidden", "hidden"}, {"Driven by Anim Curve", "animCurve"}, {"Driven by Expression", "expression"}, {"Driven by Driven Key", "drivenKey"},
                               {"Keyable", "keyable"}, {"Scale", "scale"}, {"Rotate", "rotate"}, {"Translate", "translate"}, {"Scale Rotate Translate", "srt"}, {"User Defined", "userDefined"}};

// Whether a row's node is of a Show kind.
bool of_kind(const QString& key, const QString& icon, const QString& type) {
    const bool light = type.endsWith("Light") || icon.endsWith("Light");
    const bool set = icon == "set" || type == "objectSet";
    if (key == "geometry") return icon == "mesh" || icon == "nurbsCurve" || icon == "nurbsSurface" || type == "mesh" || type == "nurbsCurve" || type == "nurbsSurface";
    if (key == "nurbs") return icon == "nurbsCurve" || icon == "nurbsSurface" || type == "nurbsCurve" || type == "nurbsSurface";
    if (key == "polygons") return icon == "mesh" || type == "mesh";
    if (key == "cameras") return icon == "camera" || type == "camera";
    if (key == "joints") return icon == "joint" || type == "joint";
    if (key == "ikHandles") return icon == "ik" || type == "ikHandle";
    if (key == "sets") return set;
    if (key == "lights") return light;
    if (key == "lightsAndSets") return light || set;
    return false;
}
} // namespace

// The reference's Show menu.
void Outliner::fill_show_menu(QMenu* m) {
    m->clear();
    auto refilter = [this] {
        last_shape_.clear();
        refresh();
    };
    QMenu* objects = m->addMenu("Objects");
    objects->addAction("Clear Below", this, [this, refilter] {
        show_kinds_.clear();
        refilter();
    });
    objects->addSeparator();
    for (const ShowKind& k : kShowKinds) {
        QAction* a = objects->addAction(k.label);
        a->setCheckable(true);
        a->setChecked(show_kinds_.contains(k.key));
        a->setEnabled(k.here);
        const QString key = k.key;
        connect(a, &QAction::toggled, this, [this, key, refilter](bool on) {
            if (on) show_kinds_.insert(key);
            else show_kinds_.remove(key);
            refilter();
        });
    }
    QMenu* attrs = m->addMenu("Attributes");
    attrs->setEnabled(attrs_);
    attrs->addAction("Clear Below", this, [this, refilter] {
        show_attrs_.clear();
        refilter();
    });
    attrs->addSeparator();
    for (auto& k : kShowAttrs) {
        QAction* a = attrs->addAction(k[0]);
        a->setCheckable(true);
        a->setChecked(show_attrs_.contains(k[1]));
        const QString key = k[1];
        connect(a, &QAction::toggled, this, [this, key, refilter](bool on) {
            if (on) show_attrs_.insert(key);
            else show_attrs_.remove(key);
            refilter();
        });
    }
    QAction* pub = attrs->addAction("Published");
    pub->setCheckable(true);
    pub->setEnabled(false);
    m->addSeparator();
    QAction* inv = m->addAction("Invert Shown");
    inv->setCheckable(true);
    inv->setChecked(invert_);
    inv->setEnabled(!show_kinds_.isEmpty());
    connect(inv, &QAction::toggled, this, [this, refilter](bool on) {
        invert_ = on;
        refilter();
    });
    m->addAction("Show All", this, [this, refilter] {
        show_kinds_.clear();
        show_attrs_.clear();
        hidden_types_.clear();
        invert_ = false;
        refilter();
    });
    m->addSeparator();
    m->addAction("Show Selected Type(s)", this, [this, refilter] {
        show_kinds_.clear();
        for (QTreeWidgetItem* it : tree_->selectedItems())
            for (const ShowKind& k : kShowKinds)
                if (k.here && QString(k.key) != "geometry" && QString(k.key) != "lightsAndSets" && of_kind(k.key, it->data(0, Qt::UserRole + 1).toString(), it->data(0, Qt::UserRole + 2).toString())) show_kinds_.insert(k.key);
        refilter();
    });
    for (const char* t : {"Select Attributes...", "Create Entry...", "Delete Entry..."}) m->addAction(t)->setEnabled(false);
    m->addSeparator();
    QAction* aux = m->addAction("Show Auxiliary Nodes");
    aux->setCheckable(true);
    aux->setEnabled(false);
    m->addAction("Auxiliary Nodes...")->setEnabled(false);
}

// The reference's Help menu, pointed at Aztek's own pages.
void Outliner::fill_help_menu(QMenu* m) {
    m->clear();
    auto open = [](const QString& url) { QDesktopServices::openUrl(QUrl(url)); };
    m->addAction("Search\tCtrl+F", this, [this] {
        filter_->setFocus();
        filter_->selectAll();
    });
    m->addAction("Find Menu")->setEnabled(false);
    m->addAction("Aztek Help", this, [open] { open("https://srlegrand.github.io/Aztek/"); });
    QMenu* news = m->addMenu("What's New");
    news->addAction("Release Notes", this, [open] { open("https://github.com/srlegrand/Aztek/releases"); });
    m->addSeparator();
    m->addAction("Help on Outliner", this, [open] { open("https://github.com/srlegrand/Aztek/tree/main/docs"); });
    m->addSeparator();
    m->addAction("MEL Command Reference", this, [open] { open("https://github.com/srlegrand/Aztek/blob/main/docs/COMMANDS.md"); });
    m->addAction("Python Command Reference", this, [open] { open("https://github.com/srlegrand/Aztek/blob/main/docs/COMMANDS.md"); });
    m->addAction("Node and Attribute Reference")->setEnabled(false);
    m->addSeparator();
    m->addAction("Aztek Home Page", this, [open] { open("https://srlegrand.github.io/Aztek/"); });
}

// Show > Objects: only rows of the kinds ticked, and what holds them; Invert Shown the other way round.
void Outliner::apply_show_filter() {
    if (show_kinds_.isEmpty()) return;
    std::function<bool(QTreeWidgetItem*)> walk = [&](QTreeWidgetItem* it) {
        const QString icon = it->data(0, Qt::UserRole + 1).toString(), type = it->data(0, Qt::UserRole + 2).toString();
        bool mine = false;
        for (const QString& k : show_kinds_) mine |= of_kind(k, icon, type);
        if (invert_) mine = !mine;
        bool below = false;
        for (int i = 0; i < it->childCount(); ++i) below |= walk(it->child(i));
        it->setHidden(!mine && !below);
        return mine || below;
    };
    for (int i = 0; i < tree_->topLevelItemCount(); ++i) walk(tree_->topLevelItem(i));
}

// Display > Attributes: the node's attributes as rows, as Connected, Show > Attributes, Attribute Order and Channel
// Names ask.
QStringList Outliner::attribute_rows(const QString& path) {
    QStringList names;
    if (connected_) {
        // Connected: those wired either way.
        const QJsonArray pairs = ask("listConnections", QJsonObject{{"args", QJsonArray{path}}, {"plugs", true}, {"connections", true}}).toArray();
        for (int i = 0; i + 1 < pairs.size(); i += 2) {
            const QString a = pairs.at(i).toString().section('.', 1);
            if (!a.isEmpty() && !names.contains(a)) names << a;
        }
    } else if (show_attrs_.isEmpty()) {
        for (const QJsonValue& a : ask("listAttr", QJsonObject{{"args", QJsonArray{path}}, {"keyable", true}}).toArray()) names << a.toString();
    } else {
        QSet<QString> want;
        auto list = [&](QJsonObject flags) {
            flags["args"] = QJsonArray{path};
            QStringList out;
            for (const QJsonValue& a : ask("listAttr", flags).toArray()) out << a.toString();
            return out;
        };
        const QStringList all = list({}), keyable = list({{"keyable", true}});
        if (show_attrs_.contains("keyable")) for (const QString& a : keyable) want.insert(a);
        if (show_attrs_.contains("userDefined")) for (const QString& a : list({{"userDefined", true}})) want.insert(a);
        if (show_attrs_.contains("hidden")) {
            const QStringList shown = list({{"visible", true}});
            for (const QString& a : all) if (!shown.contains(a)) want.insert(a);
        }
        for (const char* k : {"translate", "rotate", "scale"})
            if (show_attrs_.contains(k) || show_attrs_.contains("srt"))
                for (const QString& a : keyable) if (a.startsWith(k)) want.insert(a);
        // Driven: what feeds each attribute, by its node's type.
        if (show_attrs_.contains("animCurve") || show_attrs_.contains("expression") || show_attrs_.contains("drivenKey")) {
            const QJsonArray pairs = ask("listConnections", QJsonObject{{"args", QJsonArray{path}}, {"plugs", true}, {"connections", true}, {"destination", false}}).toArray();
            for (int i = 0; i + 1 < pairs.size(); i += 2) {
                const QString attr = pairs.at(i).toString().section('.', 1);
                const QString type = ask("nodeType", QJsonObject{{"args", QJsonArray{pairs.at(i + 1).toString().section('.', 0, 0)}}}).toString();
                const bool driven_key = type.startsWith("animCurveU");
                if ((show_attrs_.contains("animCurve") && type.startsWith("animCurveT")) || (show_attrs_.contains("drivenKey") && driven_key) || (show_attrs_.contains("expression") && type == "expression"))
                    want.insert(attr);
            }
        }
        for (const QString& a : all) if (want.contains(a)) names << a;
    }
    if (attr_order_ == 1) std::sort(names.begin(), names.end(), [](const QString& a, const QString& b) { return a.compare(b, Qt::CaseInsensitive) < 0; });
    if (attr_order_ == 2) std::sort(names.begin(), names.end(), [](const QString& a, const QString& b) { return a.compare(b, Qt::CaseInsensitive) > 0; });
    return names;
}

namespace {

// An icon with a dark edge round what it draws, so it reads on the row's own colour and on the selection's.
QIcon outlined(const QIcon& ic, qreal dpr) {
    const int px = int(16 * dpr + 0.5);
    QPixmap src = ic.pixmap(QSize(16, 16), dpr);
    QImage shadow(src.size(), QImage::Format_ARGB32_Premultiplied);
    shadow.fill(Qt::transparent);
    {
        QPainter p(&shadow);
        p.drawPixmap(0, 0, src);
        p.setCompositionMode(QPainter::CompositionMode_SourceIn);
        p.fillRect(shadow.rect(), QColor(20, 20, 20, 235));
    }
    QPixmap out(src.size());
    out.setDevicePixelRatio(src.devicePixelRatio());
    out.fill(Qt::transparent);
    {
        QPainter p(&out);
        const qreal o = 1.0 / src.devicePixelRatio() * qMax(1.0, dpr * 0.75);
        shadow.setDevicePixelRatio(src.devicePixelRatio());
        for (const QPointF& d : {QPointF(-o, 0), QPointF(o, 0), QPointF(0, -o), QPointF(0, o)}) p.drawImage(d, shadow);
        p.drawPixmap(0, 0, src);
    }
    Q_UNUSED(px);
    QIcon made;
    made.addPixmap(out, QIcon::Normal);
    made.addPixmap(out, QIcon::Selected);
    return made;
}

} // namespace

void Outliner::add(QTreeWidgetItem* parent, const QJsonObject& node) {
    QString filter = filter_->text().trimmed();
    QString ic = node["icon"].toString();
    bool cam_default = ic == "camera" && QStringList{"persp", "top", "front", "side"}.contains(node["name"].toString());
    if (hide_cameras_ && cam_default) return;
    if (hidden_types_.contains(ic == "objectSet" ? QString("set") : ic)) return;
    // Reference Members off: what came from references stays out.
    if (!ref_members_ && node["referenced"].toBool()) return;
    auto* it = parent ? new QTreeWidgetItem(parent) : new QTreeWidgetItem(tree_);
    QString path = node["path"].toString();
    QString shown_name = node["name"].toString();
    // Show Namespace off: the name without its namespaces.
    if (!namespaces_ && shown_name.contains(':')) shown_name = shown_name.section(':', -1);
    it->setText(0, shown_name);
    it->setData(0, Qt::UserRole, path);
    it->setData(0, Qt::UserRole + 1, ic);
    it->setData(0, Qt::UserRole + 2, node["type"].toString());
    if (node["material"].toBool() || node["member"].toBool()) it->setForeground(0, readable(QColor(190, 175, 140)));
    if (show_icons_) {
        QIcon shown = icon(ic, 16);
        // A node from a reference: a small blue diamond on its icon.
        if (node["referenced"].toBool()) {
            QPixmap pm = shown.pixmap(16, 16);
            QPainter p(&pm);
            p.setRenderHint(QPainter::Antialiasing);
            p.setBrush(QColor(70, 150, 255));
            p.setPen(QColor(20, 40, 80));
            const QPointF diamond[4] = {{4, 8}, {8, 12}, {4, 16}, {0, 12}};
            p.drawPolygon(diamond, 4);
            p.end();
            shown = QIcon(pm);
        }
        static QHash<QString, QIcon> edged;
        const QString key = ic + (node["referenced"].toBool() ? "@ref" : "");
        if (!edged.contains(key)) edged.insert(key, outlined(shown, devicePixelRatioF()));
        it->setIcon(0, edged.value(key));
    }
    it->setFlags(it->flags() | Qt::ItemIsEditable | Qt::ItemIsDragEnabled | Qt::ItemIsDropEnabled);
    // Default nodes dim, as the reference's are.
    if (cam_default) it->setForeground(0, readable(QColor(150, 150, 150)));
    if (!node["visible"].toBool()) it->setForeground(0, readable(QColor(130, 130, 130)));
    if (node["intermediate"].toBool()) it->setForeground(0, readable(QColor(110, 110, 160)));
    // Templated objects dim, with what is under them; their own outliner colour is kept unless hidden.
    if (node["display"].toInt() == 1) {
        it->setForeground(0, readable(QColor(140, 140, 140)));
        QFont font = it->font(0);
        font.setItalic(true);
        it->setFont(0, font);
    }
    if (!ignore_color_ && node["outlinerColor"].isArray() && node["visible"].toBool()) {
        QJsonArray c = node["outlinerColor"].toArray();
        it->setForeground(0, QColor::fromRgbF(c.at(0).toDouble(), c.at(1).toDouble(), c.at(2).toDouble()));
    }
    QJsonArray kids = node["children"].toArray();
    if (sort_alpha_) {
        // Alphabetical within type: by kind of node, then name.
        QList<QJsonObject> list;
        for (const QJsonValue& c : kids) list << c.toObject();
        std::sort(list.begin(), list.end(), [](const QJsonObject& a, const QJsonObject& b) {
            const int t = a["icon"].toString().compare(b["icon"].toString());
            return t != 0 ? t < 0 : a["name"].toString().compare(b["name"].toString(), Qt::CaseInsensitive) < 0;
        });
        kids = QJsonArray();
        for (const QJsonObject& o : list) kids.append(o);
    }
    for (const QJsonValue& c : kids) add(it, c.toObject());
    // A proxy lists its prims under the transform, or under the shape when shapes show.
    const QString stage = node["usdShape"].toString();
    if (!stage.isEmpty() && (path.section('|', -1) != stage.section('|', -1) || node["icon"].toString().isEmpty() == false)) {
        bool shape_shown = false;
        for (const QJsonValue& c : node["children"].toArray()) shape_shown |= !c.toObject()["usdShape"].toString().isEmpty();
        if (!shape_shown) add_prims(it, stage, "/", path);
    }
    if (attrs_) {
        // Channels as children, Display > Attributes.
        for (const QString& a : attribute_rows(path)) {
            auto* ch = new QTreeWidgetItem(it);
            QString label = a;
            if (channel_names_ == "short") {
                const QString sn = ask("attributeQuery", QJsonObject{{"args", QJsonArray{a}}, {"node", path}, {"shortName", true}}).toString();
                if (!sn.isEmpty()) label = sn;
            } else if (channel_names_ == "nice") {
                const QString nn = ask("attributeQuery", QJsonObject{{"args", QJsonArray{a}}, {"node", path}, {"niceName", true}}).toString();
                if (!nn.isEmpty()) label = nn;
            }
            ch->setText(0, label);
            ch->setForeground(0, readable(QColor(170, 190, 210)));
            ch->setFlags(Qt::ItemIsEnabled);
            ch->setData(0, Qt::UserRole, path);
        }
    }
    if (node["selected"].toBool()) it->setSelected(true);
    if (expanded_.contains(path)) it->setExpanded(true);
    if (!filter.isEmpty()) {
        // Glob search: *arm*, or plain text anywhere in the name.
        QRegularExpression re(QRegularExpression::wildcardToRegularExpression(filter.contains('*') || filter.contains('?') ? filter : "*" + filter + "*"), QRegularExpression::CaseInsensitiveOption);
        bool match = re.match(node["name"].toString()).hasMatch();
        bool child_visible = false;
        for (int i = 0; i < it->childCount(); ++i) child_visible |= !it->child(i)->isHidden();
        it->setHidden(!match && !child_visible);
        if (child_visible) it->setExpanded(true);
    }
}

void Outliner::set_editor_options(const QJsonObject& p) {
    if (p.contains("showShapes")) shapes_ = p["showShapes"].toBool();
    if (p.contains("showDagOnly")) dag_only_ = p["showDagOnly"].toBool();
    if (p.contains("showAttributes")) attrs_ = p["showAttributes"].toBool();
    if (p.contains("mainListConnection")) list_connection_ = p["mainListConnection"].toString();
    if (p.contains("setFilter")) item_filter_ = p["setFilter"].toString();
    if (p.contains("filter")) item_filter_ = p["filter"].toString();
    schedule();
}

// Only what the editor's connection or filter names, with the items above them.
void Outliner::apply_list_limit() {
    QSet<QString> keep;
    bool limited = false;
    if (!list_connection_.isEmpty()) {
        QString kind = ask("selectionConnection", QJsonObject{{"args", QJsonArray{list_connection_}}, {"query", true}, {"queried", QJsonArray{"identify"}}}).toString();
        if (kind == "objectList") {
            limited = true;
            for (const QJsonValue& v : ask("selectionConnection", QJsonObject{{"args", QJsonArray{list_connection_}}, {"query", true}, {"queried", QJsonArray{"object"}}}).toArray()) keep.insert(v.toString().section('|', -1));
        }
    }
    if (!item_filter_.isEmpty()) {
        QSet<QString> passed;
        for (const QJsonValue& v : ask("lsThroughFilter", QJsonObject{{"args", QJsonArray{item_filter_}}}).toArray()) passed.insert(v.toString().section('|', -1));
        keep = limited ? keep.intersect(passed) : passed;
        limited = true;
    }
    if (!limited) return;
    std::function<bool(QTreeWidgetItem*)> walk = [&](QTreeWidgetItem* it) {
        bool any = keep.contains(it->data(0, Qt::UserRole).toString().section('|', -1));
        for (int i = 0; i < it->childCount(); ++i) any |= walk(it->child(i));
        it->setHidden(!any);
        return any;
    };
    for (int i = 0; i < tree_->topLevelItemCount(); ++i) walk(tree_->topLevelItem(i));
}

// Prims under a parent. A row that isn't open holds a placeholder for its arrow, and asks when opened.
void Outliner::add_prims(QTreeWidgetItem* parent, const QString& stage, const QString& prim, const QString& key) {
    if (!expanded_.contains(key) && prim == "/") {
        new QTreeWidgetItem(parent);
        return;
    }
    QJsonArray kids = ask("usdPrim", QJsonObject{{"args", QJsonArray{stage + "," + prim}}, {"query", true}, {"children", true}}).toArray();
    if (prim == "/" && kids.isEmpty()) return;
    QSet<QString> selected;
    for (const QJsonValue& s : ask("ls", QJsonObject{{"selection", true}}).toArray()) selected.insert(s.toString());
    for (const QJsonValue& v : kids) {
        const QJsonObject o = v.toObject();
        const QString path = o["path"].toString();
        const QString full = stage + "," + path;
        auto* it = new QTreeWidgetItem(parent);
        const QString type = o["type"].toString();
        it->setText(0, o["name"].toString());
        it->setData(0, Qt::UserRole, full);
        it->setToolTip(0, type.isEmpty() ? path : QString("%1 (%2)").arg(path, type));
        it->setFlags(it->flags() | Qt::ItemIsEditable);
        if (show_icons_) it->setIcon(0, icon(type == "Mesh" ? "mesh" : type == "Camera" ? "camera" : "transform", 16));
        it->setForeground(0, readable(!o["active"].toBool() ? QColor(120, 120, 120) : !o["loaded"].toBool() ? QColor(170, 150, 100) : QColor(150, 200, 230)));
        if (selected.contains(full)) it->setSelected(true);
        if (o["kids"].toBool()) {
            if (expanded_.contains(full)) {
                add_prims(it, stage, path, full);
                it->setExpanded(true);
            } else {
                new QTreeWidgetItem(it);
            }
        }
    }
}

namespace {

// The tree without its selection, and the selected paths: a change of selection alone needn't rebuild the tree.
QJsonArray without_selection(const QJsonArray& nodes, QSet<QString>& selected) {
    QJsonArray out;
    for (const QJsonValue& v : nodes) {
        QJsonObject o = v.toObject();
        if (o["selected"].toBool()) selected.insert(o["path"].toString());
        o.remove("selected");
        if (o.contains("children")) o["children"] = without_selection(o["children"].toArray(), selected);
        out.append(o);
    }
    return out;
}

} // namespace

void Outliner::refresh() {
    QString words = QString(intermediates_ ? "shapes intermediate" : shapes_ ? "shapes" : "") + (dag_only_ ? "" : " dg");
    if (materials_) words += " materials";
    if (set_members_) words += " setMembers";
    if (ignore_hidden_) words += " showHidden";
    QJsonArray nodes = query("outliner", words).toArray();
    if (sort_alpha_) {
        QList<QJsonObject> list;
        for (const QJsonValue& c : nodes) list << c.toObject();
        std::stable_sort(list.begin(), list.end(), [](const QJsonObject& a, const QJsonObject& b) {
            const int t = a["icon"].toString().compare(b["icon"].toString());
            return t != 0 ? t < 0 : a["name"].toString().compare(b["name"].toString(), Qt::CaseInsensitive) < 0;
        });
        nodes = QJsonArray();
        for (const QJsonObject& o : list) nodes.append(o);
    }
    QSet<QString> selected;
    const QByteArray shape = QJsonDocument(without_selection(nodes, selected)).toJson(QJsonDocument::Compact);
    // Only the selection changed (or nothing did): mark it on the rows there are, as moving an object changes nothing here.
    const QString view = filter_->text() + "|" + item_filter_ + "|" + list_connection_ + "|" + QString::number(attrs_) + QString::number(show_icons_) + QString::number(hide_cameras_) + QString::number(dag_only_) + QStringList(hidden_types_.values()).join(",") + "|" + QStringList(show_kinds_.values()).join(",") + QString::number(invert_) + QString::number(ref_members_) + QString::number(namespaces_) + QString::number(ignore_color_) + QString::number(sort_alpha_);
    if (shape == last_shape_ && view == last_view_ && !attrs_ && list_connection_.isEmpty() && item_filter_.isEmpty()) {
        updating_ = true;
        std::function<void(QTreeWidgetItem*)> walk = [&](QTreeWidgetItem* it) {
            const bool want = selected.contains(it->data(0, Qt::UserRole).toString());
            if (it->isSelected() != want) it->setSelected(want);
            for (int i = 0; i < it->childCount(); ++i) walk(it->child(i));
        };
        for (int i = 0; i < tree_->topLevelItemCount(); ++i) walk(tree_->topLevelItem(i));
        updating_ = false;
        return;
    }
    last_shape_ = shape;
    last_view_ = view;
    updating_ = true;
    int scroll = tree_->verticalScrollBar()->value();
    QSet<QString> before;
    std::function<void(QTreeWidgetItem*)> note = [&](QTreeWidgetItem* it) {
        before.insert(it->data(0, Qt::UserRole).toString());
        for (int i = 0; i < it->childCount(); ++i) note(it->child(i));
    };
    for (int i = 0; i < tree_->topLevelItemCount(); ++i) note(tree_->topLevelItem(i));
    tree_->clear();
    for (const QJsonValue& n : nodes) add(nullptr, n.toObject());
    apply_list_limit();
    apply_show_filter();
    tree_->verticalScrollBar()->setValue(scroll);
    // A node just made and selected (a group, a duplicate) is scrolled into view, so the list doesn't lose it.
    if (!before.isEmpty())
        for (QTreeWidgetItem* it : tree_->selectedItems())
            if (!before.contains(it->data(0, Qt::UserRole).toString())) {
                tree_->scrollToItem(it, QAbstractItemView::EnsureVisible);
                break;
            }
    updating_ = false;
}

void Outliner::selection_changed() {
    if (updating_) return;
    QJsonArray names;
    for (auto* it : tree_->selectedItems()) names.append(it->data(0, Qt::UserRole).toString());
    if (names.isEmpty())
        exec("select", QJsonObject{{"clear", true}});
    else
        exec("select", args(names));
}

} // namespace az

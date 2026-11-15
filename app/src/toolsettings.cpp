#include "toolsettings.h"
#include <QSettings>
#include "host.h"

#include <QButtonGroup>
#include <QCheckBox>
#include <QComboBox>
#include <QDoubleSpinBox>
#include <QFormLayout>
#include <QLabel>
#include <QListWidget>
#include <QTreeWidget>
#include <QTreeWidgetItemIterator>
#include <QHeaderView>
#include <QLineEdit>
#include <QToolButton>
#include <QStandardItemModel>
#include <QSignalBlocker>
#include <QSet>
#include <algorithm>
#include "icons.h"
#include <QColorDialog>
#include <QSpinBox>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QSlider>
#include <QStackedLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QVBoxLayout>

namespace az {

namespace {
// A titled part of the page that folds away.
void fold(QVBoxLayout* page, const QString& title, QFormLayout*& form) {
    auto* head = new QToolButton();
    head->setText(title);
    head->setCheckable(true);
    head->setChecked(true);
    head->setArrowType(Qt::DownArrow);
    head->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
    head->setAutoRaise(true);
    head->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    auto* body = new QWidget();
    form = new QFormLayout(body);
    form->setContentsMargins(8, 2, 2, 6);
    QObject::connect(head, &QToolButton::toggled, body, [head, body](bool open) {
        body->setVisible(open);
        head->setArrowType(open ? Qt::DownArrow : Qt::RightArrow);
    });
    page->addWidget(head);
    page->addWidget(body);
}

QJsonValue paint_query(const char* flag) {
    return ask("artAttrSkinPaintCtx", QJsonObject{{"query", true}, {"queried", QJsonArray{flag}}});
}
} // namespace

ToolSettings::ToolSettings(QWidget* parent) : Panel(parent) {
    auto* root = new QVBoxLayout(this);
    root->setContentsMargins(6, 6, 6, 6);
    title_ = new QLabel(this);
    title_->setObjectName("toolTitle");
    QFont f = title_->font();
    f.setBold(true);
    title_->setFont(f);
    root->addWidget(title_);
    auto* holder = new QWidget(this);
    stack_ = new QStackedLayout(holder);
    stack_->setContentsMargins(0, 0, 0, 0);
    transform_page_ = build_transform_page();
    paint_page_ = build_paint_page();
    joint_page_ = build_joint_page();
    curve_page_ = build_curve_page();
    stack_->addWidget(curve_page_);
    sculpt_page_ = build_sculpt_page();
    stack_->addWidget(sculpt_page_);
    stack_->addWidget(transform_page_);
    stack_->addWidget(paint_page_);
    stack_->addWidget(joint_page_);
    attr_page_ = build_attr_page();
    stack_->addWidget(attr_page_);
    member_page_ = build_member_page();
    stack_->addWidget(member_page_);
    root->addWidget(holder, 1);
    set_tool("select");
}

QDoubleSpinBox* ToolSettings::slider_row(QFormLayout* form, const QString& label, double lo, double hi, double step, std::function<void(double)> on) {
    auto* row = new QWidget(this);
    auto* h = new QHBoxLayout(row);
    h->setContentsMargins(0, 0, 0, 0);
    auto* spin = new QDoubleSpinBox(row);
    spin->setRange(lo, hi);
    spin->setSingleStep(step);
    spin->setDecimals(4);
    spin->setFixedWidth(84);
    auto* slider = new QSlider(Qt::Horizontal, row);
    slider->setRange(0, 1000);
    h->addWidget(spin);
    h->addWidget(slider, 1);
    connect(slider, &QSlider::valueChanged, this, [=](int v) {
        if (loading_) return;
        double x = lo + (hi - lo) * v / 1000.0;
        spin->blockSignals(true);
        spin->setValue(x);
        spin->blockSignals(false);
        on(x);
    });
    connect(spin, &QDoubleSpinBox::valueChanged, this, [=](double x) {
        slider->blockSignals(true);
        slider->setValue(int((x - lo) / (hi - lo) * 1000.0));
        slider->blockSignals(false);
        if (!loading_) on(x);
    });
    form->addRow(label, row);
    return spin;
}

QWidget* ToolSettings::build_sculpt_page() {
    auto* w = new QWidget(this);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    auto* form = new QFormLayout();
    auto set = [this](const char* flag, const QJsonValue& val) {
        if (loading_) return;
        QJsonObject c{{"edit", true}};
        c[flag] = val;
        exec_quiet("sculptMeshCacheCtx", c);
        Host::get()->poke();
    };
    sculpt_tool_ = new QComboBox(w);
    sculpt_tool_->addItems({"sculpt", "smooth", "relax", "grab", "pinch", "flatten", "foamy", "spray", "repeat", "imprint", "wax", "scrape", "fill", "knife", "smear", "bulge", "amplify", "freeze"});
    connect(sculpt_tool_, &QComboBox::currentTextChanged, this, [set](const QString& t) { set("sculptTool", t); });
    form->addRow("Brush:", sculpt_tool_);
    sculpt_size_ = slider_row(form, "Size:", 0.001, 20.0, 0.05, [set](double x) { set("size", x); });
    sculpt_strength_ = slider_row(form, "Strength:", 0.0, 1.0, 0.05, [set](double x) { set("strength", x); });
    sculpt_invert_ = new QCheckBox("Invert", w);
    connect(sculpt_invert_, &QCheckBox::toggled, this, [set](bool on) { set("invert", on); });
    form->addRow("", sculpt_invert_);
    sculpt_sym_ = new QCheckBox("Symmetry across X", w);
    connect(sculpt_sym_, &QCheckBox::toggled, this, [set](bool on) { set("symmetry", on); });
    form->addRow("", sculpt_sym_);
    auto* clear = new QPushButton("Unfreeze All", w);
    connect(clear, &QPushButton::clicked, this, [] { exec_quiet("sculptMeshCacheCtx", QJsonObject{{"edit", true}, {"clearFreeze", true}}); });
    form->addRow("", clear);
    v->addLayout(form);
    v->addStretch(1);
    return w;
}

void ToolSettings::refresh_sculpt() {
    loading_ = true;
    auto q = [](const char* flag) { return ask("sculptMeshCacheCtx", QJsonObject{{"query", true}, {"queried", QJsonArray{flag}}}); };
    sculpt_tool_->setCurrentText(q("sculptTool").toString());
    if (!sculpt_size_->hasFocus()) sculpt_size_->setValue(q("size").toDouble());
    if (!sculpt_strength_->hasFocus()) sculpt_strength_->setValue(q("strength").toDouble());
    sculpt_invert_->setChecked(q("invert").toBool());
    sculpt_sym_->setChecked(q("symmetry").toBool());
    loading_ = false;
}

QWidget* ToolSettings::build_curve_page() {
    auto* w = new QWidget(this);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    auto* form = new QFormLayout();
    auto* degree = new QComboBox(w);
    for (auto [label, d] : {std::pair{"1 Linear", 1}, {"2", 2}, {"3 Cubic", 3}, {"5", 5}, {"7", 7}}) degree->addItem(label, d);
    degree->setCurrentIndex(2);
    connect(degree, &QComboBox::currentIndexChanged, this, [this, degree] { emit curveDegree(degree->currentData().toInt()); });
    form->addRow("Curve degree:", degree);
    v->addLayout(form);
    v->addWidget(new QLabel("Click to place points. Backspace takes the last back.\nEnter makes the curve.", w));
    v->addStretch(1);
    return w;
}

QWidget* ToolSettings::build_joint_page() {
    auto* w = new QWidget(this);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    v->addWidget(new QLabel("Click to place joints, each under the last.\nEnter ends the chain.", w));
    v->addStretch(1);
    return w;
}

void ToolSettings::paint_set(const QString& flag, const QJsonValue& val) {
    if (loading_) return;
    QJsonObject c{{"edit", true}};
    c[flag] = val;
    exec_quiet("artAttrSkinPaintCtx", c);
    Host::get()->poke();
}

QWidget* ToolSettings::build_paint_page() {
    // As the reference lays it out: Skin Clusters, Influences with the weight buttons, Brush, Visualization, Stroke and
    // Display, each folding away. What isn't here yet shows greyed.
    auto* scroll = new QScrollArea(this);
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    auto* w = new QWidget(scroll);
    scroll->setWidget(w);
    auto* page = new QVBoxLayout(w);
    page->setContentsMargins(0, 0, 0, 0);
    page->setSpacing(2);
    QFormLayout* form = nullptr;

    fold(page, "Skin Clusters", form);
    skins_ = new QListWidget(w);
    skins_->setMaximumHeight(60);
    connect(skins_, &QListWidget::itemClicked, this, [this](QListWidgetItem* it) {
        // The skinned mesh of that skinCluster becomes the one painted.
        QJsonArray geo = ask("skinCluster", QJsonObject{{"args", QJsonArray{it->text()}}, {"query", true}, {"geometry", true}}).toArray();
        if (!geo.isEmpty()) exec("select", QJsonObject{{"args", QJsonArray{geo.at(0)}}});
    });
    form->addRow(skins_);

    fold(page, "Influences", form);
    auto* sorts = new QWidget(w);
    auto* sl = new QGridLayout(sorts);
    sl->setContentsMargins(0, 0, 0, 0);
    sort_ = new QButtonGroup(sorts);
    int k = 0;
    for (const char* label : {"Alphabetically", "By Hierarchy", "Flat"}) {
        auto* r = new QRadioButton(label, sorts);
        r->setChecked(k == 1);
        sort_->addButton(r, k);
        sl->addWidget(r, k / 2, k % 2);
        ++k;
    }
    connect(sort_, &QButtonGroup::idClicked, this, [this](int) {
        last_influences_.clear();
        refresh_paint();
    });
    form->addRow("Sort:", sorts);
    search_ = new QLineEdit(w);
    search_->setPlaceholderText("Search...");
    search_->setClearButtonEnabled(true);
    connect(search_, &QLineEdit::textChanged, this, [this] { filter_influences(); });
    form->addRow(search_);
    influences_ = new QTreeWidget(w);
    influences_->setColumnCount(1);
    influences_->setHeaderHidden(true);
    influences_->setMinimumHeight(160);
    influences_->setSelectionMode(QAbstractItemView::ExtendedSelection);
    influences_->setRootIsDecorated(true);
    connect(influences_, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem* cur, QTreeWidgetItem*) {
        if (cur) exec_quiet("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"influence", cur->data(0, Qt::UserRole).toString()}});
    });
    // The order influences are picked in: the first is the one Move weights takes from.
    connect(influences_, &QTreeWidget::itemSelectionChanged, this, [this] {
        QStringList now;
        for (QTreeWidgetItem* it : influences_->selectedItems()) now << it->data(0, Qt::UserRole).toString();
        QStringList kept;
        for (const QString& n : pick_order_)
            if (now.contains(n)) kept << n;
        for (const QString& n : now)
            if (!kept.contains(n)) kept << n;
        pick_order_ = kept;
    });
    connect(influences_, &QTreeWidget::itemChanged, this, [this](QTreeWidgetItem* it, int col) {
        if (loading_ || col != 0 || last_skin_.isEmpty()) return;
        bool lock = it->checkState(0) == Qt::Checked;
        exec("skinCluster", QJsonObject{{"args", QJsonArray{last_skin_}}, {"edit", true}, {"influence", it->data(0, Qt::UserRole).toString()}, {"lockWeights", lock}});
    });
    form->addRow(influences_);

    // The buttons under the list.
    auto* buttons = new QWidget(w);
    auto* bl = new QHBoxLayout(buttons);
    bl->setContentsMargins(0, 0, 0, 0);
    bl->setSpacing(2);
    auto button = [&](const QString& ic, const QString& tip, std::function<void()> fn) {
        auto* b = new QToolButton(buttons);
        b->setIcon(icon(ic));
        b->setIconSize(QSize(20, 20));
        b->setAutoRaise(true);
        b->setToolTip(tip);
        connect(b, &QToolButton::clicked, this, fn);
        bl->addWidget(b);
    };
    bl->addStretch(1);
    button("copyWeights", "Copy the weights of the selected vertex", [] { exec("artAttrSkinWeightCopy"); });
    button("pasteWeights", "Paste the copied weights onto the selected vertices", [] { exec("artAttrSkinWeightPaste"); });
    button("hammerWeights", "Weight Hammer: the selected vertices take the weights around them", [] { exec("weightHammerVerts"); });
    button("moveWeights", "Move weights: the first selected influence gives the selected vertices' weight to the other selected influences",
           [this] { move_weights(); });
    button("showInfluences", "Show influences on selected vertices", [this] { show_vertex_influences(); });
    bl->addSpacing(16);
    button("revealSelected", "Reveal Selected", [this] {
        if (QTreeWidgetItem* it = influences_->currentItem()) influences_->scrollToItem(it);
        else if (!influences_->selectedItems().isEmpty()) influences_->scrollToItem(influences_->selectedItems().first());
    });
    button("invertSelection", "Invert Selection", [this] {
        QSignalBlocker block(influences_);
        for (QTreeWidgetItemIterator i(influences_); *i; ++i)
            if (!(*i)->isHidden()) (*i)->setSelected(!(*i)->isSelected());
    });
    bl->addStretch(1);
    form->addRow(buttons);
    auto* holds = new QHBoxLayout();
    for (auto [label, on] : {std::pair{"Hold All", true}, {"Release All", false}}) {
        auto* b = new QPushButton(label, w);
        bool lock = on;
        connect(b, &QPushButton::clicked, this, [this, lock] {
            if (!last_skin_.isEmpty()) exec("skinCluster", QJsonObject{{"args", QJsonArray{last_skin_}}, {"edit", true}, {"lockWeights", lock}});
        });
        holds->addWidget(b);
    }
    form->addRow(holds);

    fold(page, "Brush", form);
    auto radios = [&](const QStringList& labels, QButtonGroup*& group) {
        auto* box = new QWidget(w);
        auto* l = new QGridLayout(box);
        l->setContentsMargins(0, 0, 0, 0);
        group = new QButtonGroup(box);
        for (int i = 0; i < labels.size(); ++i) {
            auto* r = new QRadioButton(labels[i], box);
            r->setChecked(i == 0);
            group->addButton(r, i);
            l->addWidget(r, i / 2, i % 2);
        }
        return box;
    };
    // Mode: -spm 1 paints, 0 selects with a marquee, 2 paint selects; the points are picked as vertices.
    form->addRow("Mode:", radios({"Paint", "Select", "Paint Select"}, skin_mode_));
    QWidget* ps = radios({"Add", "Remove", "Toggle"}, paint_select_);
    form->addRow("Paint Select:", ps);
    connect(skin_mode_, &QButtonGroup::idClicked, this, [this, ps](int id) {
        const int spm[] = {1, 0, 2};
        exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"skinPaintMode", spm[id]}});
        if (id != 0) exec("selectMode", QJsonObject{{"args", QJsonArray{"vertex"}}});
        ps->setEnabled(id == 2);
    });
    ps->setEnabled(false);
    connect(paint_select_, &QButtonGroup::idClicked, this, [](int id) { exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"paintSelectMode", id + 1}}); });
    auto* sel_geo = new QPushButton("Select Geometry", w);
    connect(sel_geo, &QPushButton::clicked, this, [] {
        exec("selectMode", QJsonObject{{"args", QJsonArray{"object"}}});
    });
    form->addRow("", sel_geo);
    op_ = new QButtonGroup(w);
    auto* ops = new QWidget(w);
    auto* grid = new QGridLayout(ops);
    grid->setContentsMargins(0, 0, 0, 0);
    int i = 0;
    for (auto [label, name] : {std::pair{"Replace", "absolute"}, {"Add", "additive"}, {"Scale", "scale"}, {"Smooth", "smooth"}}) {
        auto* r = new QRadioButton(label, ops);
        r->setProperty("op", QString(name));
        op_->addButton(r, i);
        grid->addWidget(r, i / 2, i % 2);
        ++i;
    }
    connect(op_, &QButtonGroup::idClicked, this, [this](int id) { paint_set("selectedattroper", op_->button(id)->property("op").toString()); });
    form->addRow("Paint operation:", ops);
    stamp_ = new QComboBox(w);
    stamp_->addItems({"gaussian", "poly", "solid", "square"});
    connect(stamp_, &QComboBox::currentTextChanged, this, [this](const QString& t) { paint_set("stampProfile", t); });
    form->addRow("Profile:", stamp_);
    auto* weight_type = new QComboBox(w);
    weight_type->addItems({"Skin Weight", "DQ Blend Weight"});
    if (auto* m = qobject_cast<QStandardItemModel*>(weight_type->model())) m->item(1)->setEnabled(false);
    form->addRow("Weight Type:", weight_type);
    normalize_ = new QComboBox(w);
    normalize_->addItems({"Off", "Interactive", "Post"});
    connect(normalize_, &QComboBox::activated, this, [this](int n) {
        if (!loading_ && !last_skin_.isEmpty()) exec("setAttr", QJsonObject{{"args", QJsonArray{last_skin_ + ".normalizeWeights", n}}});
    });
    form->addRow("Normalize Weights:", normalize_);
    opacity_ = slider_row(form, "Opacity:", 0.0, 1.0, 0.05, [this](double x) { paint_set("opacity", x); });
    value_ = slider_row(form, "Value:", 0.0, 1.0, 0.05, [this](double x) { paint_set("value", x); });
    // Min/Max value: the range the value slider covers.
    auto* range = new QWidget(w);
    auto* rl = new QHBoxLayout(range);
    rl->setContentsMargins(0, 0, 0, 0);
    auto* lo = new QDoubleSpinBox(range);
    auto* hi = new QDoubleSpinBox(range);
    for (QDoubleSpinBox* b : {lo, hi}) {
        b->setDecimals(4);
        b->setRange(-1000, 1000);
        b->setSingleStep(0.1);
        rl->addWidget(b);
    }
    lo->setValue(0.0);
    hi->setValue(1.0);
    rl->addStretch(1);
    auto set_range = [this, lo, hi] {
        if (hi->value() > lo->value()) value_->setRange(lo->value(), hi->value());
    };
    connect(lo, &QDoubleSpinBox::valueChanged, this, set_range);
    connect(hi, &QDoubleSpinBox::valueChanged, this, set_range);
    form->addRow("Min/Max value:", range);
    auto* flood = new QPushButton("Flood", w);
    connect(flood, &QPushButton::clicked, this, [] {
        exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"clear", true}});
    });
    form->addRow("", flood);

    fold(page, "Visualization", form);
    feedback_ = new QCheckBox("Color feedback", w);
    connect(feedback_, &QCheckBox::toggled, this, [this](bool on) { paint_set("colorfeedback", on); });
    form->addRow("", feedback_);
    auto* ramp = new QCheckBox("Color Ramp", w);
    ramp->setEnabled(false);
    form->addRow("", ramp);
    // Numeric: the painted influence's weight beside each point.
    auto* numbers = new QCheckBox("Show Numbers", w);
    connect(numbers, &QCheckBox::toggled, this, [](bool on) { exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"useNumericDisplay", on}}); });
    form->addRow("", numbers);
    auto* decimals = new QSpinBox(w);
    decimals->setRange(0, 6);
    decimals->setValue(2);
    connect(decimals, &QSpinBox::valueChanged, this, [](int n) { exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"numericDisplayPrecision", n}}); });
    form->addRow("Decimal Places:", decimals);
    auto* number_color = new QPushButton(w);
    number_color->setFixedWidth(60);
    number_color->setStyleSheet("background: white;");
    connect(number_color, &QPushButton::clicked, this, [this, number_color] {
        const QColor c = QColorDialog::getColor(Qt::white, this, "Number Color");
        if (!c.isValid()) return;
        number_color->setStyleSheet(QString("background: %1;").arg(c.name()));
        exec("artAttrSkinPaintCtx", QJsonObject{{"edit", true}, {"numericDisplayColor", QJsonArray{c.redF(), c.greenF(), c.blueF()}}});
    });
    form->addRow("Number Color:", number_color);

    fold(page, "Stroke", form);
    radius_ = slider_row(form, "Radius(U):", 0.001, 50.0, 0.05, [this](double x) { paint_set("radius", x); });
    // Radius(L) is a tablet's; Reflection is off for this tool, as the reference's (Skin > Mirror Skin Weights).
    for (const char* label : {"Screen projection", "Reflection"}) {
        auto* c = new QCheckBox(label, w);
        c->setEnabled(false);
        form->addRow("", c);
    }

    fold(page, "Stylus Pressure", form);
    auto* stylus = new QCheckBox("Stylus pressure", w);
    stylus->setEnabled(false);
    form->addRow("", stylus);

    fold(page, "Display", form);
    for (const char* label : {"Draw brush", "Show wireframe"}) {
        auto* c = new QCheckBox(label, w);
        c->setChecked(true);
        c->setEnabled(false);
        form->addRow("", c);
    }
    auto* xray = new QCheckBox("X-Ray Joints", w);
    connect(xray, &QCheckBox::toggled, this, [](bool on) {
        for (const QJsonValue& p : ask("getPanel", QJsonObject{{"type", "modelPanel"}}).toArray())
            exec_quiet("modelEditor", QJsonObject{{"args", QJsonArray{p}}, {"edit", true}, {"jointXray", on}});
    });
    form->addRow("", xray);
    page->addStretch(1);
    return scroll;
}

void ToolSettings::move_weights() {
    // As the reference: the first influence picked gives its weight on the selected vertices to the others picked.
    QStringList picked;
    for (const QString& n : pick_order_)
        for (QTreeWidgetItem* it : influences_->selectedItems())
            if (it->data(0, Qt::UserRole).toString() == n) picked << n;
    if (picked.size() < 2 || last_skin_.isEmpty()) {
        log("// Warning: Move weights: select the influence to take weight from, then the ones to give it to. //", 4);
        return;
    }
    QJsonArray tmw;
    for (const QString& n : picked) tmw.append(n);
    QJsonArray call{last_skin_};
    for (const QJsonValue& v : ask("filterExpand", QJsonObject{{"selectionMask", 31}, {"expand", true}}).toArray()) call.append(v);
    if (call.size() < 2) return;
    exec("skinPercent", QJsonObject{{"args", call}, {"transformMoveWeights", tmw}});
}

void ToolSettings::show_vertex_influences() {
    // Selects in the list the influences weighting the selected vertices.
    QJsonArray verts = ask("filterExpand", QJsonObject{{"selectionMask", 31}, {"expand", true}}).toArray();
    if (verts.isEmpty() || last_skin_.isEmpty()) return;
    QJsonArray call{last_skin_};
    for (const QJsonValue& v : verts) call.append(v);
    QSet<QString> on;
    for (const QJsonValue& v : ask("skinPercent", QJsonObject{{"args", call}, {"query", true}, {"transform", true}, {"ignoreBelow", 1e-6}}).toArray())
        on.insert(v.toString());
    QSignalBlocker block(influences_);
    QTreeWidgetItem* first = nullptr;
    for (QTreeWidgetItemIterator i(influences_); *i; ++i) {
        QString n = (*i)->data(0, Qt::UserRole).toString();
        bool hit = on.contains(n) || on.contains(n.section('|', -1));
        (*i)->setSelected(hit);
        if (hit && !first) first = *i;
    }
    if (first) influences_->scrollToItem(first);
}

void ToolSettings::filter_influences() {
    QString t = search_->text().trimmed();
    for (QTreeWidgetItemIterator i(influences_); *i; ++i) {
        bool show = t.isEmpty() || (*i)->text(0).contains(t, Qt::CaseInsensitive);
        // A parent stays to hold a matching child.
        if (!show)
            for (QTreeWidgetItemIterator c(*i); *c; ++c) {
                QTreeWidgetItem* p = *c;
                bool under = false;
                for (QTreeWidgetItem* up = p->parent(); up; up = up->parent()) under |= up == *i;
                if (!under) break;
                if (p->text(0).contains(t, Qt::CaseInsensitive)) {
                    show = true;
                    break;
                }
            }
        (*i)->setHidden(!show);
    }
}

void ToolSettings::attr_set(const QString& flag, const QJsonValue& val) {
    if (loading_) return;
    QJsonObject c{{"edit", true}};
    c[flag] = val;
    exec_quiet("artAttrCtx", c);
    Host::get()->poke();
}

QWidget* ToolSettings::build_attr_page() {
    auto* w = new QWidget(this);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    auto* form = new QFormLayout();
    attr_list_ = new QComboBox(w);
    connect(attr_list_, &QComboBox::currentTextChanged, this, [this](const QString& t) { if (!t.isEmpty()) attr_set("attrSelected", t); });
    form->addRow("Attribute:", attr_list_);
    attr_op_ = new QButtonGroup(w);
    auto* ops = new QWidget(w);
    auto* grid = new QGridLayout(ops);
    grid->setContentsMargins(0, 0, 0, 0);
    int i = 0;
    for (auto [label, name] : {std::pair{"Replace", "absolute"}, {"Add", "additive"}, {"Scale", "scale"}, {"Smooth", "smooth"}}) {
        auto* r = new QRadioButton(label, ops);
        r->setProperty("op", QString(name));
        attr_op_->addButton(r, i);
        grid->addWidget(r, i / 2, i % 2);
        ++i;
    }
    connect(attr_op_, &QButtonGroup::idClicked, this, [this](int id) { attr_set("selectedattroper", attr_op_->button(id)->property("op").toString()); });
    form->addRow("Paint operation:", ops);
    attr_stamp_ = new QComboBox(w);
    attr_stamp_->addItems({"gaussian", "poly", "solid", "square"});
    connect(attr_stamp_, &QComboBox::currentTextChanged, this, [this](const QString& t) { attr_set("stampProfile", t); });
    form->addRow("Profile:", attr_stamp_);
    attr_value_ = slider_row(form, "Value:", 0.0, 1.0, 0.05, [this](double x) { attr_set("value", x); });
    attr_opacity_ = slider_row(form, "Opacity:", 0.0, 1.0, 0.05, [this](double x) { attr_set("opacity", x); });
    attr_radius_ = slider_row(form, "Radius(U):", 0.001, 20.0, 0.05, [this](double x) { attr_set("radius", x); });
    attr_feedback_ = new QCheckBox("Color feedback", w);
    connect(attr_feedback_, &QCheckBox::toggled, this, [this](bool on) { attr_set("colorfeedback", on); });
    form->addRow("", attr_feedback_);
    auto* flood = new QPushButton("Flood", w);
    connect(flood, &QPushButton::clicked, this, [] { exec("artAttrCtx", QJsonObject{{"edit", true}, {"clear", true}}); });
    form->addRow("", flood);
    v->addLayout(form);
    v->addStretch(1);
    return w;
}

void ToolSettings::refresh_attr() {
    loading_ = true;
    auto q = [](const char* flag) { return ask("artAttrCtx", QJsonObject{{"query", true}, {"queried", QJsonArray{flag}}}); };
    QStringList names;
    for (const QJsonValue& x : q("paintableAttrs").toArray()) names << x.toString();
    QStringList had;
    for (int i = 0; i < attr_list_->count(); ++i) had << attr_list_->itemText(i);
    if (had != names) {
        attr_list_->clear();
        attr_list_->addItems(names);
    }
    QString current = q("attrSelected").toString();
    int at = attr_list_->findText(current);
    if (at >= 0) attr_list_->setCurrentIndex(at);
    QString op = q("selectedattroper").toString();
    for (QAbstractButton* b : attr_op_->buttons())
        if (b->property("op").toString() == op) b->setChecked(true);
    attr_stamp_->setCurrentText(q("stampProfile").toString());
    if (!attr_value_->hasFocus()) attr_value_->setValue(q("value").toDouble());
    if (!attr_opacity_->hasFocus()) attr_opacity_->setValue(q("opacity").toDouble());
    if (!attr_radius_->hasFocus()) attr_radius_->setValue(q("radius").toDouble());
    attr_feedback_->setChecked(q("colorfeedback").toBool());
    loading_ = false;
    // With nothing chosen yet the brush is on the first the mesh offers: say so to the tool.
    if (current.isEmpty() && !names.isEmpty()) attr_set("attrSelected", names.first());
}

QWidget* ToolSettings::build_member_page() {
    auto* w = new QWidget(this);
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    auto* form = new QFormLayout();
    member_list_ = new QComboBox(w);
    form->addRow("Deformer:", member_list_);
    v->addLayout(form);
    auto set_of = [this]() -> QString {
        QString d = member_list_->currentText();
        if (d.isEmpty()) return QString();
        QJsonArray sets = ask("listConnections", QJsonObject{{"args", QJsonArray{d}}, {"type", "objectSet"}}).toArray();
        return sets.isEmpty() ? d + "Set" : sets.at(0).toString();
    };
    auto* select = new QPushButton("Select Members", w);
    connect(select, &QPushButton::clicked, this, [set_of] {
        QString s = set_of();
        if (!s.isEmpty()) exec("selectSetMembers", QJsonObject{{"args", QJsonArray{s}}});
    });
    v->addWidget(select);
    auto* add = new QPushButton("Add Selected Points", w);
    connect(add, &QPushButton::clicked, this, [set_of] {
        QString s = set_of();
        if (!s.isEmpty()) exec("sets", QJsonObject{{"addElement", s}});
    });
    v->addWidget(add);
    auto* rm = new QPushButton("Remove Selected Points", w);
    connect(rm, &QPushButton::clicked, this, [set_of] {
        QString s = set_of();
        if (!s.isEmpty()) exec("sets", QJsonObject{{"remove", s}});
    });
    v->addWidget(rm);
    v->addStretch(1);
    return w;
}

void ToolSettings::refresh_member() {
    QStringList names;
    for (const QJsonValue& x : ask("ls", QJsonObject{{"type", "geometryFilter"}}).toArray()) names << x.toString();
    QStringList had;
    for (int i = 0; i < member_list_->count(); ++i) had << member_list_->itemText(i);
    if (had != names) {
        QString keep = member_list_->currentText();
        member_list_->clear();
        member_list_->addItems(names);
        if (names.contains(keep)) member_list_->setCurrentText(keep);
    }
}

void ToolSettings::set_tool(const QString& tool) {
    tool_ = tool;
    if (tool == "paintAttr") {
        title_->setText("Paint Attribute Tool");
        stack_->setCurrentWidget(attr_page_);
    } else if (tool == "editMembership") {
        title_->setText("Edit Membership Tool");
        stack_->setCurrentWidget(member_page_);
    } else if (tool == "paintSkin") {
        title_->setText("Paint Skin Weights Tool");
        stack_->setCurrentWidget(paint_page_);
    } else if (tool == "sculpt") {
        title_->setText("Sculpting Tools");
        stack_->setCurrentWidget(sculpt_page_);
    } else if (tool == "joint") {
        title_->setText("Joint Tool");
        stack_->setCurrentWidget(joint_page_);
    } else if (tool == "curveCv" || tool == "curveEp") {
        title_->setText(tool == "curveCv" ? "CV Curve Tool" : "EP Curve Tool");
        stack_->setCurrentWidget(curve_page_);
    } else {
        QString n = tool.left(1).toUpper() + tool.mid(1);
        title_->setText(n + " Tool");
        stack_->setCurrentWidget(transform_page_);
    }
    schedule();
}

void ToolSettings::refresh() {
    if (tool_ == "paintAttr") refresh_attr();
    else if (tool_ == "editMembership") refresh_member();
    else if (tool_ == "paintSkin") refresh_paint();
    else if (tool_ == "sculpt") refresh_sculpt();
    else if (stack_->currentWidget() == transform_page_) refresh_transform();
}

void ToolSettings::refresh_paint() {
    loading_ = true;
    QString op = paint_query("selectedattroper").toString();
    for (QAbstractButton* b : op_->buttons())
        if (b->property("op").toString() == op) b->setChecked(true);
    stamp_->setCurrentText(paint_query("stampProfile").toString());
    if (!value_->hasFocus()) value_->setValue(paint_query("value").toDouble());
    if (!opacity_->hasFocus()) opacity_->setValue(paint_query("opacity").toDouble());
    if (!radius_->hasFocus()) radius_->setValue(paint_query("radius").toDouble());
    feedback_->setChecked(paint_query("colorfeedback").toBool());
    // The skinClusters of the scene, the selected mesh's current.
    QStringList skins;
    for (const QJsonValue& x : ask("ls", QJsonObject{{"type", "skinCluster"}}).toArray()) skins << x.toString();
    QJsonArray sel = state()["selection"].toArray();
    QString sk;
    for (const QJsonValue& v : sel) {
        sk = ask("findRelatedSkinCluster", QJsonObject{{"args", QJsonArray{v.toString().section('.', 0, 0)}}}).toString();
        if (!sk.isEmpty()) break;
    }
    QStringList had;
    for (int i = 0; i < skins_->count(); ++i) had << skins_->item(i)->text();
    if (had != skins) {
        skins_->clear();
        skins_->addItems(skins);
    }
    for (int i = 0; i < skins_->count(); ++i) skins_->item(i)->setSelected(skins_->item(i)->text() == sk.section('|', -1) || skins_->item(i)->text() == sk);
    QStringList names;
    if (!sk.isEmpty())
        for (const QJsonValue& v : ask("skinCluster", QJsonObject{{"args", QJsonArray{sk}}, {"query", true}, {"influence", true}}).toArray()) names << v.toString();
    if (!sk.isEmpty() && !normalize_->hasFocus()) normalize_->setCurrentIndex(ask("getAttr", QJsonObject{{"args", QJsonArray{sk + ".normalizeWeights"}}}).toInt(1));
    QString current = paint_query("influence").toString();
    if (sk != last_skin_ || names != last_influences_) {
        last_skin_ = sk;
        last_influences_ = names;
        influences_->clear();
        int sort = sort_->checkedId();
        QStringList order = names;
        if (sort == 0) std::sort(order.begin(), order.end(), [](const QString& a, const QString& b) { return a.section('|', -1).compare(b.section('|', -1), Qt::CaseInsensitive) < 0; });
        QHash<QString, QTreeWidgetItem*> made;
        // By Hierarchy: under the nearest parent that is an influence too.
        auto parent_of = [&](const QString& n) -> QString {
            QString up = n;
            for (int guard = 0; guard < 64; ++guard) {
                QJsonArray p = ask("listRelatives", QJsonObject{{"args", QJsonArray{up}}, {"parent", true}, {"fullPath", true}}).toArray();
                if (p.isEmpty()) return QString();
                up = p.at(0).toString();
                for (const QString& m : names)
                    if (m == up || m == up.section('|', -1) || ("|" + m) == up || up.endsWith("|" + m)) return m;
            }
            return QString();
        };
        std::function<QTreeWidgetItem*(const QString&)> item_for = [&](const QString& n) -> QTreeWidgetItem* {
            if (QTreeWidgetItem* it = made.value(n)) return it;
            QTreeWidgetItem* parent = nullptr;
            if (sort == 1) {
                QString p = parent_of(n);
                if (!p.isEmpty() && p != n) parent = item_for(p);
            }
            auto* it = parent ? new QTreeWidgetItem(parent) : new QTreeWidgetItem(influences_);
            it->setText(0, n.section('|', -1));
            it->setData(0, Qt::UserRole, n);
            it->setFlags(it->flags() | Qt::ItemIsUserCheckable);
            it->setCheckState(0, Qt::Unchecked);
            it->setToolTip(0, "Ticked holds the influence: the brush leaves its weights alone");
            made.insert(n, it);
            return it;
        };
        for (const QString& n : order) item_for(n);
        influences_->setRootIsDecorated(sort == 1);
        influences_->expandAll();
        filter_influences();
    }
    for (QTreeWidgetItemIterator i(influences_); *i; ++i) {
        QTreeWidgetItem* it = *i;
        QString n = it->data(0, Qt::UserRole).toString();
        bool locked = ask("skinCluster", QJsonObject{{"args", QJsonArray{sk}}, {"query", true}, {"influence", n}, {"lockWeights", true}}).toBool();
        it->setCheckState(0, locked ? Qt::Checked : Qt::Unchecked);
        it->setForeground(0, locked ? QBrush(QColor(120, 160, 230)) : QBrush(palette().color(QPalette::Text)));
        if (n == current || (current.isEmpty() && influences_->currentItem() == nullptr)) {
            if (influences_->currentItem() != it) influences_->setCurrentItem(it);
        }
    }
    loading_ = false;
}

} // namespace az

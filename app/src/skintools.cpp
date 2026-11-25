// Skin Tools: the layers of the selected skinned mesh, what the brush paints on the current layer, and the layer tools
// (flood, closest joint, unify, mirror, import, export, transfer), over the skinLayers and skinLayersTool commands.
#include "host.h"

#include <QCheckBox>
#include <QComboBox>
#include <QDoubleSpinBox>
#include <QFileDialog>
#include <QFormLayout>
#include <QGroupBox>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QLabel>
#include <QListWidget>
#include <QPushButton>
#include <QSignalBlocker>
#include <QSpinBox>
#include <QSplitter>
#include <QTreeWidget>
#include <QVBoxLayout>

#include <functional>

namespace az {

namespace {

QWidget* window_shell(QWidget* parent, const QString& title, int w, int h) {
    auto* win = new QWidget(parent, editor_window_flags());
    win->setWindowTitle(title);
    win->resize(w, h);
    return win;
}

void follow_scene(QWidget* w, std::function<void()> fn) {
    QObject::connect(Host::get(), &Host::sceneChanged, w, [w, fn] {
        if (w->isVisible()) fn();
    });
}

// The skinned mesh the window works on: the first selected object with a skinCluster.
QString skinned_target() {
    for (const QJsonValue& v : state()["selection"].toArray()) {
        QString obj = v.toString().section('.', 0, 0);
        if (!ask("findRelatedSkinCluster", QJsonObject{{"args", QJsonArray{obj}}}).toString().isEmpty()) return obj;
    }
    return QString();
}

QJsonValue layers_q(const QString& t, const QJsonObject& more) {
    QJsonObject c = more;
    c["args"] = QJsonArray{t};
    c["query"] = true;
    return ask("skinLayers", c);
}

}  // namespace

QWidget* skin_tools_window(QWidget* parent) {
    auto* win = window_shell(parent, "Skin Tools", 420, 760);
    auto* v = new QVBoxLayout(win);
    auto* target_label = new QLabel(win);
    v->addWidget(target_label);

    auto* init_row = new QHBoxLayout();
    auto* init = new QPushButton("Initialize Skin Layers", win);
    auto* remove_all = new QPushButton("Delete Skin Layers", win);
    init_row->addWidget(init);
    init_row->addWidget(remove_all);
    v->addLayout(init_row);

    auto* split = new QSplitter(Qt::Vertical, win);
    v->addWidget(split, 1);

    // Layers, top first as the stack reads.
    auto* layers_box = new QGroupBox("Layers", split);
    auto* lv = new QVBoxLayout(layers_box);
    auto* tree = new QTreeWidget(layers_box);
    tree->setHeaderLabels({"Layer", "Opacity"});
    tree->header()->setStretchLastSection(false);
    tree->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    lv->addWidget(tree, 1);
    auto* lrow = new QHBoxLayout();
    QPushButton* add = new QPushButton("Add", layers_box);
    QPushButton* add_child = new QPushButton("Add Child", layers_box);
    QPushButton* del = new QPushButton("Delete", layers_box);
    QPushButton* dup = new QPushButton("Duplicate", layers_box);
    QPushButton* merge = new QPushButton("Merge Down", layers_box);
    QPushButton* up = new QPushButton("Up", layers_box);
    QPushButton* down = new QPushButton("Down", layers_box);
    for (QPushButton* b : {add, add_child, del, dup, merge, up, down}) lrow->addWidget(b);
    lv->addLayout(lrow);
    auto* opacity_row = new QHBoxLayout();
    opacity_row->addWidget(new QLabel("Opacity", layers_box));
    auto* opacity = new QDoubleSpinBox(layers_box);
    opacity->setRange(0, 1);
    opacity->setSingleStep(0.05);
    opacity->setDecimals(3);
    opacity_row->addWidget(opacity, 1);
    lv->addLayout(opacity_row);

    // What the brush paints: the mask, the dual quaternion weights or an influence.
    auto* inf_box = new QGroupBox("Paint Target", split);
    auto* iv = new QVBoxLayout(inf_box);
    auto* infs = new QListWidget(inf_box);
    iv->addWidget(infs, 1);

    // The brush and the flood.
    auto* paint_box = new QGroupBox("Paint", split);
    auto* pf = new QFormLayout(paint_box);
    auto* mode = new QComboBox(paint_box);
    mode->addItems({"Replace", "Add", "Scale", "Smooth", "Sharpen"});
    auto* intensity = new QDoubleSpinBox(paint_box);
    intensity->setRange(0, 10);
    intensity->setSingleStep(0.05);
    intensity->setDecimals(3);
    intensity->setValue(1.0);
    auto* iterations = new QSpinBox(paint_box);
    iterations->setRange(1, 100);
    auto* brush_mirror = new QCheckBox("Mirror what is painted", paint_box);
    auto* selected_only = new QCheckBox("Limit to selected vertices", paint_box);
    pf->addRow("Mode", mode);
    pf->addRow("Intensity", intensity);
    pf->addRow("Iterations", iterations);
    pf->addRow(brush_mirror);
    pf->addRow(selected_only);
    auto* prow = new QHBoxLayout();
    auto* paint = new QPushButton("Paint", paint_box);
    auto* flood = new QPushButton("Flood", paint_box);
    prow->addWidget(paint);
    prow->addWidget(flood);
    pf->addRow(prow);

    // Tools.
    auto* tools_box = new QGroupBox("Tools", split);
    auto* tf = new QFormLayout(tools_box);
    auto* closest = new QPushButton("Assign From Closest Joint", tools_box);
    auto* unify = new QPushButton("Unify Weights", tools_box);
    tf->addRow(closest);
    tf->addRow(unify);
    auto* axis = new QComboBox(tools_box);
    axis->addItems({"X", "Y", "Z"});
    auto* direction = new QComboBox(tools_box);
    direction->addItems({"Positive to negative", "Negative to positive", "Flip"});
    auto* seam = new QDoubleSpinBox(tools_box);
    seam->setRange(0, 1000);
    seam->setDecimals(3);
    tf->addRow("Mirror axis", axis);
    tf->addRow("Direction", direction);
    tf->addRow("Seam width", seam);
    auto* mrow = new QHBoxLayout();
    auto* mw = new QCheckBox("Weights", tools_box);
    auto* mm = new QCheckBox("Mask", tools_box);
    auto* mdq = new QCheckBox("DQ", tools_box);
    for (QCheckBox* c : {mw, mm, mdq}) {
        c->setChecked(true);
        mrow->addWidget(c);
    }
    auto* mirror = new QPushButton("Mirror", tools_box);
    mrow->addWidget(mirror);
    tf->addRow(mrow);
    auto* frow = new QHBoxLayout();
    auto* exp = new QPushButton("Export...", tools_box);
    auto* imp = new QPushButton("Import...", tools_box);
    auto* transfer = new QPushButton("Transfer", tools_box);
    transfer->setToolTip("Copy the first selected skinned mesh's layers onto the others.");
    frow->addWidget(exp);
    frow->addWidget(imp);
    frow->addWidget(transfer);
    tf->addRow(frow);

    auto* target = new QString();
    auto* filling = new bool(false);
    const char* modes[] = {"replace", "add", "scale", "smooth", "sharpen"};

    auto current_id = [tree]() -> int {
        QTreeWidgetItem* it = tree->currentItem();
        return it ? it->data(0, Qt::UserRole).toInt() : -1;
    };
    auto run_layers = [target](QJsonObject c) {
        if (target->isEmpty()) return QJsonValue();
        c["args"] = QJsonArray{*target};
        return exec("skinLayers", c);
    };
    auto run_tool = [target, selected_only](QJsonObject c) {
        if (target->isEmpty()) return QJsonValue();
        c["args"] = QJsonArray{*target};
        if (selected_only->isChecked()) c["selectedComponents"] = true;
        return exec("skinLayersTool", c);
    };

    std::function<void()> refresh;
    refresh = [=] {
        if (*filling) return;
        *filling = true;
        *target = skinned_target();
        bool has = !target->isEmpty() && layers_q(*target, QJsonObject{{"exists", true}}).toBool();
        target_label->setText(target->isEmpty() ? "Select a skinned mesh." : (has ? *target : *target + ": no skin layers yet."));
        init->setEnabled(!target->isEmpty() && !has);
        remove_all->setEnabled(has);
        for (QWidget* w : std::initializer_list<QWidget*>{layers_box, inf_box, paint_box, tools_box}) w->setEnabled(has);
        {
            QSignalBlocker b1(tree), b2(infs);
            tree->clear();
            infs->clear();
            if (has) {
                QJsonArray ls = layers_q(*target, QJsonObject{{"layers", true}}).toArray();
                int cur = layers_q(*target, QJsonObject{{"current", true}}).toInt(-1);
                QHash<int, QTreeWidgetItem*> made;
                QJsonObject cur_layer;
                // Parents before children; siblings top first.
                std::function<void(int, QTreeWidgetItem*)> place = [&](int parent, QTreeWidgetItem* under) {
                    QList<QJsonObject> kids;
                    for (const QJsonValue& x : ls)
                        if (x["parent"].toInt(-1) == parent) kids.prepend(x.toObject());
                    for (const QJsonObject& l : kids) {
                        auto* it = under ? new QTreeWidgetItem(under) : new QTreeWidgetItem(tree);
                        it->setText(0, l["name"].toString());
                        it->setText(1, QString::number(l["opacity"].toDouble(), 'f', 2));
                        it->setData(0, Qt::UserRole, l["id"].toInt());
                        it->setFlags(it->flags() | Qt::ItemIsEditable | Qt::ItemIsUserCheckable);
                        it->setCheckState(0, l["enabled"].toBool() ? Qt::Checked : Qt::Unchecked);
                        made[l["id"].toInt()] = it;
                        if (l["id"].toInt() == cur) cur_layer = l;
                        place(l["id"].toInt(), it);
                    }
                };
                place(-1, nullptr);
                tree->expandAll();
                if (made.contains(cur)) tree->setCurrentItem(made[cur]);
                opacity->setValue(cur_layer["opacity"].toDouble(1.0));
                QJsonValue pt = cur_layer["paintTarget"];
                auto* mask = new QListWidgetItem("[Mask]", infs);
                mask->setData(Qt::UserRole, "mask");
                auto* dq = new QListWidgetItem("[DQ Weights]", infs);
                dq->setData(Qt::UserRole, "dq");
                if (pt.toString() == "mask") infs->setCurrentItem(mask);
                if (pt.toString() == "dq") infs->setCurrentItem(dq);
                QJsonArray used = cur_layer["influences"].toArray();
                for (const QJsonValue& i : layers_q(*target, QJsonObject{{"influences", true}}).toArray()) {
                    int e = i["index"].toInt();
                    auto* it = new QListWidgetItem(i["name"].toString(), infs);
                    it->setData(Qt::UserRole, e);
                    QFont f = it->font();
                    f.setBold(used.contains(e));
                    it->setFont(f);
                    if (pt.isDouble() && pt.toInt() == e) infs->setCurrentItem(it);
                }
                QJsonObject m = layers_q(*target, QJsonObject{{"mirrorAxis", true}}).toObject();
                QSignalBlocker b3(axis), b4(direction), b5(seam);
                axis->setCurrentIndex(QString("xyz").indexOf(m["axis"].toString("x")));
                QString d = m["direction"].toString();
                direction->setCurrentIndex(d == "negativeToPositive" ? 1 : d == "flip" ? 2 : 0);
                seam->setValue(m["seamWidth"].toDouble());
            }
        }
        QJsonObject b = ask("skinLayersTool", QJsonObject{{"query", true}, {"mode", true}}).toObject();
        QSignalBlocker b6(mode), b7(intensity), b8(iterations), b9(brush_mirror);
        for (int k = 0; k < 5; ++k)
            if (b["mode"].toString() == modes[k]) mode->setCurrentIndex(k);
        intensity->setValue(b["intensity"].toDouble(1.0));
        iterations->setValue(b["iterations"].toInt(1));
        brush_mirror->setChecked(b["mirror"].toBool());
        *filling = false;
    };

    QObject::connect(init, &QPushButton::clicked, win, [=] {
        QString t = skinned_target();
        if (t.isEmpty()) return log("// Warning: select a skinned mesh //", 4);
        exec("skinLayers", QJsonObject{{"args", QJsonArray{t}}, {"initialize", true}});
    });
    QObject::connect(remove_all, &QPushButton::clicked, win, [=] { run_layers(QJsonObject{{"delete", true}}); });
    QObject::connect(add, &QPushButton::clicked, win, [=] { run_layers(QJsonObject{{"add", "New Layer"}}); });
    QObject::connect(add_child, &QPushButton::clicked, win, [=] {
        int id = current_id();
        QJsonObject c{{"add", "New Layer"}};
        if (id >= 0) c["parent"] = id;
        run_layers(c);
    });
    QObject::connect(del, &QPushButton::clicked, win, [=] { if (current_id() >= 0) run_layers(QJsonObject{{"remove", current_id()}}); });
    QObject::connect(dup, &QPushButton::clicked, win, [=] { if (current_id() >= 0) run_layers(QJsonObject{{"duplicate", current_id()}}); });
    QObject::connect(merge, &QPushButton::clicked, win, [=] { if (current_id() >= 0) run_layers(QJsonObject{{"mergeDown", current_id()}}); });
    auto move = [=](int by) {
        int id = current_id();
        if (id < 0) return;
        int idx = layers_q(*target, QJsonObject{{"layer", id}}).toObject()["index"].toInt();
        // The list shows the top first: up is a higher index.
        run_layers(QJsonObject{{"edit", true}, {"layer", id}, {"index", std::max(0, idx + by)}});
    };
    QObject::connect(up, &QPushButton::clicked, win, [=] { move(1); });
    QObject::connect(down, &QPushButton::clicked, win, [=] { move(-1); });
    QObject::connect(tree, &QTreeWidget::currentItemChanged, win, [=](QTreeWidgetItem* it) {
        if (*filling || !it) return;
        run_layers(QJsonObject{{"current", it->data(0, Qt::UserRole).toInt()}});
    });
    QObject::connect(tree, &QTreeWidget::itemChanged, win, [=](QTreeWidgetItem* it, int col) {
        if (*filling || col != 0) return;
        int id = it->data(0, Qt::UserRole).toInt();
        QJsonObject now = layers_q(*target, QJsonObject{{"layer", id}}).toObject();
        bool on = it->checkState(0) == Qt::Checked;
        if (now["enabled"].toBool() != on) run_layers(QJsonObject{{"edit", true}, {"layer", id}, {"enabled", on}});
        if (now["name"].toString() != it->text(0)) run_layers(QJsonObject{{"edit", true}, {"layer", id}, {"name", it->text(0)}});
    });
    QObject::connect(opacity, &QDoubleSpinBox::editingFinished, win, [=] {
        if (current_id() >= 0) run_layers(QJsonObject{{"edit", true}, {"layer", current_id()}, {"opacity", opacity->value()}});
    });
    QObject::connect(infs, &QListWidget::currentItemChanged, win, [=](QListWidgetItem* it) {
        if (*filling || !it || current_id() < 0) return;
        run_layers(QJsonObject{{"edit", true}, {"layer", current_id()}, {"paintTarget", it->data(Qt::UserRole).toJsonValue()}});
    });
    auto brush = [=] {
        if (*filling) return;
        exec_quiet("skinLayersTool", QJsonObject{{"mode", modes[mode->currentIndex()]}, {"intensity", intensity->value()}, {"iterations", iterations->value()}, {"brushMirror", brush_mirror->isChecked()}});
    };
    QObject::connect(mode, &QComboBox::currentIndexChanged, win, brush);
    QObject::connect(intensity, &QDoubleSpinBox::valueChanged, win, brush);
    QObject::connect(iterations, &QSpinBox::valueChanged, win, brush);
    QObject::connect(brush_mirror, &QCheckBox::toggled, win, brush);
    QObject::connect(paint, &QPushButton::clicked, win, [] { mel("ArtPaintSkinWeightsTool;"); });
    QObject::connect(flood, &QPushButton::clicked, win, [=] {
        run_tool(QJsonObject{{"flood", true}, {"mode", modes[mode->currentIndex()]}, {"intensity", intensity->value()}, {"iterations", iterations->value()}, {"brushMirror", brush_mirror->isChecked()}});
    });
    QObject::connect(closest, &QPushButton::clicked, win, [=] { run_tool(QJsonObject{{"assignFromClosestJoint", true}}); });
    QObject::connect(unify, &QPushButton::clicked, win, [=] {
        QJsonObject c{{"unify", true}};
        c["args"] = QJsonArray{*target};
        c["selectedComponents"] = true;
        if (!target->isEmpty()) exec("skinLayersTool", c);
    });
    auto mirror_settings = [=] {
        if (*filling) return;
        const char* dirs[] = {"positiveToNegative", "negativeToPositive", "flip"};
        run_layers(QJsonObject{{"mirrorAxis", QString("xyz").mid(axis->currentIndex(), 1)}, {"mirrorDirection", dirs[direction->currentIndex()]}, {"seamWidth", seam->value()}});
    };
    QObject::connect(axis, &QComboBox::currentIndexChanged, win, mirror_settings);
    QObject::connect(direction, &QComboBox::currentIndexChanged, win, mirror_settings);
    QObject::connect(seam, &QDoubleSpinBox::editingFinished, win, mirror_settings);
    QObject::connect(mirror, &QPushButton::clicked, win, [=] {
        run_tool(QJsonObject{{"mirror", true}, {"mirrorWeights", mw->isChecked()}, {"mirrorMask", mm->isChecked()}, {"mirrorDq", mdq->isChecked()}});
    });
    QObject::connect(exp, &QPushButton::clicked, win, [=] {
        if (target->isEmpty()) return;
        QString f = QFileDialog::getSaveFileName(win, "Export Skin Layers", *target + ".json", "Skin layers (*.json)");
        if (!f.isEmpty()) exec("skinLayersTool", QJsonObject{{"args", QJsonArray{*target}}, {"exportFile", f}});
    });
    QObject::connect(imp, &QPushButton::clicked, win, [=] {
        QString t = skinned_target();
        if (t.isEmpty()) return log("// Warning: select a skinned mesh //", 4);
        QString f = QFileDialog::getOpenFileName(win, "Import Skin Layers", QString(), "Skin layers (*.json)");
        if (!f.isEmpty()) exec("skinLayersTool", QJsonObject{{"args", QJsonArray{t}}, {"importFile", f}});
    });
    QObject::connect(transfer, &QPushButton::clicked, win, [=] {
        QStringList objs;
        for (const QJsonValue& s : state()["selection"].toArray()) {
            QString o = s.toString().section('.', 0, 0);
            if (!objs.contains(o)) objs << o;
        }
        if (objs.size() < 2) return log("// Warning: select the source skinned mesh, then the destinations //", 4);
        for (int k = 1; k < objs.size(); ++k) exec("skinLayersTool", QJsonObject{{"args", QJsonArray{objs[k]}}, {"transferFrom", objs[0]}});
    });

    refresh();
    follow_scene(win, refresh);
    return win;
}

}  // namespace az

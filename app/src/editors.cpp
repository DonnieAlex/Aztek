#include "editors.h"
#include "shapeeditor.h"
#include "host.h"
extern "C" {
#include "aztek.h"
}
#include <algorithm>
#include <cmath>
#include "icons.h"
#include "nodeeditor.h"
#include "viewport.h"

#include <QApplication>
#include <QCheckBox>
#include <QColorDialog>
#include <QDialogButtonBox>
#include <QFileDialog>
#include <QMenu>
#include <QMenuBar>
#include <QSpinBox>
#include <QComboBox>
#include <QDialog>
#include <QDoubleSpinBox>
#include <QFormLayout>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QJsonDocument>
#include <QLabel>
#include <QLineEdit>
#include <QListWidget>
#include <QMouseEvent>
#include <QPainter>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QScrollArea>
#include <QSettings>
#include <QSignalBlocker>
#include <QSlider>
#include <QTableWidget>
#include <QTabWidget>
#include <QToolBar>
#include <QTimer>
#include <QTreeWidget>
#include <QVBoxLayout>
#include <algorithm>

namespace az {
QWidget* human_ik_panel(QWidget* parent);

namespace {

QJsonObject args(const QJsonArray& a) {
    QJsonObject o;
    o["args"] = a;
    return o;
}

QStringList selection_names() {
    QStringList out;
    for (const QJsonValue& v : state()["selection"].toArray()) out << v.toString();
    return out;
}

QJsonValue editors_query(const QJsonObject& q) {
    return query("editors", QString::fromUtf8(QJsonDocument(q).toJson(QJsonDocument::Compact)));
}

QWidget* window_shell(QWidget* parent, const QString& title, int w, int h) {
    auto* win = new QWidget(parent, editor_window_flags());
    win->setWindowTitle(title);
    win->resize(w, h);
    return win;
}

// Keeps a window's contents up with the scene while it's open.
void follow_scene(QWidget* w, std::function<void()> fn) {
    QObject::connect(Host::get(), &Host::sceneChanged, w, [w, fn] {
        if (w->isVisible()) fn();
    });
}

// ---------------------------------------------------------------------------------------------------------------------

QWidget* spread_sheet(QWidget* parent) {
    auto* win = window_shell(parent, "Attribute Spread Sheet", 860, 340);
    auto* lay = new QVBoxLayout(win);
    auto* tabs = new QTabWidget(win);
    auto* keyable = new QTableWidget(win);
    auto* shape = new QTableWidget(win);
    tabs->addTab(keyable, "Keyable");
    tabs->addTab(shape, "Shape");
    lay->addWidget(tabs);
    auto* updating = new bool(false);
    auto fill = [updating](QTableWidget* t, const char* section) {
        if (QWidget* fw = QApplication::focusWidget()) { if (t->isAncestorOf(fw) && qobject_cast<QLineEdit*>(fw)) return; }
        *updating = true;
        QJsonArray rows = editors_query(QJsonObject{{"what", "spreadSheet"}}).toArray();
        QStringList cols;
        for (const QJsonValue& r : rows)
            for (const QJsonValue& c : r.toObject()[section].toArray())
                if (!cols.contains(c.toObject()["attr"].toString())) cols << c.toObject()["attr"].toString();
        t->clear();
        t->setColumnCount(cols.size());
        t->setRowCount(rows.size());
        t->setHorizontalHeaderLabels(cols);
        QStringList names;
        for (int i = 0; i < rows.size(); ++i) {
            QJsonObject r = rows[i].toObject();
            names << r["node"].toString().section('|', -1);
            for (const QJsonValue& cv : r[section].toArray()) {
                QJsonObject c = cv.toObject();
                int col = cols.indexOf(c["attr"].toString());
                QJsonValue v = c["value"];
                QString text = v.isDouble() ? QString::number(v.toDouble(), 'f', 3) : v.isBool() ? (v.toBool() ? "on" : "off") : v.toString();
                auto* it = new QTableWidgetItem(text);
                it->setData(Qt::UserRole, c["plug"].toString());
                it->setData(Qt::UserRole + 1, c["type"].toString());
                t->setItem(i, col, it);
            }
        }
        t->setVerticalHeaderLabels(names);
        *updating = false;
    };
    for (QTableWidget* t : {keyable, shape})
        QObject::connect(t, &QTableWidget::itemChanged, t, [updating](QTableWidgetItem* it) {
            if (*updating) return;
            QString ty = it->data(Qt::UserRole + 1).toString(), s = it->text().trimmed();
            QJsonValue v = ty == "bool" ? QJsonValue(s == "on" || s == "1" || s == "true") : ty == "string" ? QJsonValue(s) : ty == "int" || ty == "enum" ? QJsonValue(qint64(s.toDouble())) : QJsonValue(s.toDouble());
            QJsonObject call = args(QJsonArray{it->data(Qt::UserRole).toString(), v});
            if (ty == "string") call["type"] = "string";
            exec("setAttr", call);
        });
    auto refresh = [fill, keyable, shape] {
        fill(keyable, "channels");
        fill(shape, "shape");
    };
    refresh();
    follow_scene(win, refresh);
    return win;
}

// Skin weights of the selected vertices per influence, each cell editable.
QWidget* component_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Component Editor", 720, 360);
    auto* lay = new QVBoxLayout(win);
    auto* note = new QLabel("Smooth Skins: weights of the selected vertices.", win);
    lay->addWidget(note);
    auto* t = new QTableWidget(win);
    lay->addWidget(t, 1);
    auto* loading = new bool(false);
    auto refresh = [t, loading, note] {
        if (QWidget* fw = QApplication::focusWidget()) { if (t->isAncestorOf(fw) && qobject_cast<QLineEdit*>(fw)) return; }
        *loading = true;
        t->clear();
        QJsonArray pts = ask("ls", QJsonObject{{"selection", true}, {"flatten", true}}).toArray();
        QJsonArray verts;
        for (const QJsonValue& p : pts)
            if (p.toString().contains(".vtx[")) verts.append(p);
        QString skin;
        if (!verts.isEmpty()) skin = ask("findRelatedSkinCluster", args(QJsonArray{verts[0].toString().section('.', 0, 0)})).toString();
        if (skin.isEmpty()) {
            t->setRowCount(0);
            t->setColumnCount(0);
            note->setText("Select vertices of a skinned mesh to see their weights.");
            *loading = false;
            return;
        }
        note->setText("Smooth Skins: " + skin);
        QJsonArray infs = ask("skinPercent", QJsonObject{{"args", QJsonArray{skin, verts[0]}}, {"query", true}, {"transform", QJsonValue()}}).toArray();
        QStringList inf_names;
        for (const QJsonValue& i : infs) inf_names << i.toString();
        // Total on the end, as the reference's.
        t->setColumnCount(inf_names.size() + 1);
        QStringList heads = inf_names;
        heads << "Total";
        t->setHorizontalHeaderLabels(heads);
        t->setRowCount(verts.size());
        QStringList rows;
        for (int r = 0; r < verts.size(); ++r) {
            QString vtx = verts[r].toString();
            rows << vtx.section('.', 1);
            QJsonArray vals = ask("skinPercent", QJsonObject{{"args", QJsonArray{skin, vtx}}, {"query", true}, {"value", true}}).toArray();
            double total = 0;
            for (int c = 0; c < inf_names.size(); ++c) {
                double w = vals.at(c).toDouble();
                total += w;
                auto* it = new QTableWidgetItem(QString::number(w, 'f', 3));
                it->setData(Qt::UserRole, vtx);
                it->setData(Qt::UserRole + 1, inf_names[c]);
                it->setData(Qt::UserRole + 2, skin);
                t->setItem(r, c, it);
            }
            auto* tot = new QTableWidgetItem(QString::number(total, 'f', 3));
            tot->setFlags(Qt::ItemIsEnabled);
            t->setItem(r, inf_names.size(), tot);
        }
        t->setVerticalHeaderLabels(rows);
        *loading = false;
    };
    QObject::connect(t, &QTableWidget::itemChanged, t, [loading](QTableWidgetItem* it) {
        if (*loading || it->data(Qt::UserRole).isNull()) return;
        exec("skinPercent", QJsonObject{{"args", QJsonArray{it->data(Qt::UserRole + 2).toString(), it->data(Qt::UserRole).toString()}},
                                        {"transformValue", QJsonArray{it->data(Qt::UserRole + 1).toString(), it->text().toDouble()}}});
    });
    refresh();
    follow_scene(win, refresh);
    return win;
}

// Two lists: click an output on the left, then an input on the right to connect or disconnect.
QWidget* connection_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Connection Editor", 620, 520);
    auto* v = new QVBoxLayout(win);
    auto* cols = new QHBoxLayout();
    struct Side { QLabel* name; QListWidget* list; QString node; };
    auto* left = new Side{new QLabel("Outputs", win), new QListWidget(win), QString()};
    auto* right = new Side{new QLabel("Inputs", win), new QListWidget(win), QString()};
    auto column = [win, cols](Side* s, const QString& reload) {
        auto* c = new QVBoxLayout();
        auto* b = new QPushButton(reload, win);
        c->addWidget(b);
        c->addWidget(s->name);
        c->addWidget(s->list, 1);
        cols->addLayout(c);
        return b;
    };
    auto load = [](Side* s) {
        QStringList sel = selection_names();
        if (sel.isEmpty()) return;
        s->node = sel.last().section('.', 0, 0);
        s->name->setText(s->node);
        s->list->clear();
        for (const QJsonValue& a : ask("listAttr", args(QJsonArray{s->node})).toArray()) s->list->addItem(a.toString());
    };
    QPushButton* lb = column(left, "Reload Left");
    QPushButton* rb = column(right, "Reload Right");
    QObject::connect(lb, &QPushButton::clicked, win, [load, left] { load(left); });
    QObject::connect(rb, &QPushButton::clicked, win, [load, right] { load(right); });
    v->addLayout(cols, 1);
    QObject::connect(right->list, &QListWidget::itemClicked, win, [left, right](QListWidgetItem* it) {
        if (!left->list->currentItem() || left->node.isEmpty()) return;
        QString src = left->node + "." + left->list->currentItem()->text();
        QString dst = right->node + "." + it->text();
        QJsonArray incoming = ask("listConnections", QJsonObject{{"args", QJsonArray{dst}}, {"source", true}, {"plugs", true}}).toArray();
        bool connected = false;
        for (const QJsonValue& p : incoming)
            if (p.toString().section('|', -1) == src.section('|', -1)) connected = true;
        if (connected) exec("disconnectAttr", args(QJsonArray{src, dst}));
        else exec("connectAttr", args(QJsonArray{src, dst}));
    });
    load(left);
    load(right);
    return win;
}

// The DAG as boxes joined by lines; a click selects.
class HierarchyGraph : public QWidget {
public:
    explicit HierarchyGraph(QWidget* parent) : QWidget(parent) {
        setMinimumSize(400, 300);
        rebuild();
    }
    void rebuild() {
        nodes_.clear();
        QJsonArray list = editors_query(QJsonObject{{"what", "hypergraph"}}).toArray();
        QHash<QString, QList<QJsonObject>> kids;
        QList<QJsonObject> roots;
        for (const QJsonValue& v : list) {
            QJsonObject o = v.toObject();
            if (o["parent"].isNull()) roots.append(o);
            else kids[o["parent"].toString()].append(o);
        }
        int row = 0;
        std::function<void(const QJsonObject&, int)> walk = [&](const QJsonObject& o, int depth) {
            Node n{o["name"].toString(), o["path"].toString(), o["parent"].toString(), QRect(20 + depth * 170, 16 + row * 34, 150, 24), o["selected"].toBool()};
            nodes_.append(n);
            ++row;
            for (const QJsonObject& c : kids[o["path"].toString()]) walk(c, depth + 1);
        };
        for (const QJsonObject& r : roots) walk(r, 0);
        int w = 200, h = 80;
        for (const Node& n : nodes_) {
            w = qMax(w, n.rect.right() + 20);
            h = qMax(h, n.rect.bottom() + 20);
        }
        setMinimumSize(w, h);
        update();
    }
protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.fillRect(rect(), QColor(58, 58, 58));
        p.setRenderHint(QPainter::Antialiasing);
        QHash<QString, QRect> at;
        for (const Node& n : nodes_) at[n.path] = n.rect;
        p.setPen(QPen(QColor(170, 170, 170), 1));
        for (const Node& n : nodes_)
            if (at.contains(n.parent)) {
                QRect a = at[n.parent];
                p.drawLine(QPoint(a.right(), a.center().y()), QPoint(n.rect.left(), n.rect.center().y()));
            }
        for (const Node& n : nodes_) {
            p.setPen(QColor(30, 30, 30));
            p.setBrush(n.selected ? QColor(95, 170, 120) : QColor(110, 130, 150));
            p.drawRoundedRect(n.rect, 3, 3);
            p.setPen(Qt::white);
            p.drawText(n.rect.adjusted(5, 0, -5, 0), Qt::AlignVCenter | Qt::AlignLeft, fontMetrics().elidedText(n.name, Qt::ElideRight, n.rect.width() - 10));
        }
    }
    void mousePressEvent(QMouseEvent* e) override {
        for (const Node& n : nodes_)
            if (n.rect.contains(e->position().toPoint())) {
                QJsonObject c = args(QJsonArray{n.path});
                if (e->modifiers() & Qt::ShiftModifier) c["add"] = true;
                exec("select", c);
                return;
            }
        exec("select", QJsonObject{{"clear", true}});
    }
private:
    struct Node { QString name, path, parent; QRect rect; bool selected; };
    QList<Node> nodes_;
};

QWidget* hypergraph_hierarchy(QWidget* parent) {
    auto* win = window_shell(parent, "Hypergraph Hierarchy", 760, 500);
    auto* v = new QVBoxLayout(win);
    v->setContentsMargins(0, 0, 0, 0);
    auto* scroll = new QScrollArea(win);
    auto* g = new HierarchyGraph(scroll);
    scroll->setWidget(g);
    scroll->setWidgetResizable(true);
    v->addWidget(scroll);
    follow_scene(win, [g] { g->rebuild(); });
    return win;
}

QWidget* hypergraph_connections(QWidget* parent) {
    auto* win = window_shell(parent, "Hypergraph Connections", 900, 560);
    auto* v = new QVBoxLayout(win);
    v->setContentsMargins(0, 0, 0, 0);
    auto* bar = new QToolBar(win);
    auto* ed = new NodeEditor(win);
    // The whole history, which is what this window is for; the Node Editor stays one step out until asked.
    bar->addAction("Input and Output Connections", [ed] { ed->graph_selection(64); });
    bar->addAction("One Step Out", [ed] { ed->graph_selection(1); });
    v->addWidget(bar);
    v->addWidget(ed, 1);
    ed->graph_selection(64);
    return win;
}

QWidget* namespace_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Namespace Editor", 360, 320);
    auto* v = new QVBoxLayout(win);
    v->addWidget(new QLabel(":", win));
    auto* list = new QListWidget(win);
    v->addWidget(list, 1);
    auto refresh = [list] {
        list->clear();
        QJsonObject names = editors_query(QJsonObject{{"what", "namespaces"}}).toObject();
        for (auto it = names.begin(); it != names.end(); ++it) list->addItem(QString("%1  (%2)").arg(it.key()).arg(it.value().toInt()));
    };
    bool can = has_command_named("namespace");
    auto* row = new QHBoxLayout();
    auto* add = new QPushButton("New", win);
    auto* del = new QPushButton("Delete", win);
    add->setEnabled(can);
    del->setEnabled(can);
    if (!can) add->setToolTip("Not available yet: the namespace command");
    QObject::connect(add, &QPushButton::clicked, win, [win] {
        QLineEdit* e = win->findChild<QLineEdit*>();
        if (e && !e->text().isEmpty()) exec("namespace", QJsonObject{{"add", e->text()}});
    });
    QObject::connect(del, &QPushButton::clicked, win, [list] {
        if (auto* it = list->currentItem()) exec("namespace", QJsonObject{{"removeNamespace", it->text().section(' ', 0, 0)}});
    });
    auto* name = new QLineEdit(win);
    name->setEnabled(can);
    row->addWidget(name, 1);
    row->addWidget(add);
    row->addWidget(del);
    v->addLayout(row);
    refresh();
    follow_scene(win, refresh);
    return win;
}

QWidget* channel_control(QWidget* parent) {
    auto* win = window_shell(parent, "Channel Control", 520, 460);
    auto* v = new QVBoxLayout(win);
    auto* tabs = new QTabWidget(win);
    auto* keyable_tab = new QWidget(win);
    auto* h = new QHBoxLayout(keyable_tab);
    auto* keyable = new QListWidget(win);
    auto* hidden = new QListWidget(win);
    keyable->setSelectionMode(QAbstractItemView::ExtendedSelection);
    hidden->setSelectionMode(QAbstractItemView::ExtendedSelection);
    auto* mid = new QVBoxLayout();
    auto* hide_b = new QPushButton("Move >>", win);
    auto* show_b = new QPushButton("<< Move", win);
    mid->addStretch(1);
    mid->addWidget(hide_b);
    mid->addWidget(show_b);
    mid->addStretch(1);
    h->addWidget(keyable, 1);
    h->addLayout(mid);
    h->addWidget(hidden, 1);
    tabs->addTab(keyable_tab, "Keyable");
    auto* locked_tab = new QWidget(win);
    auto* lh = new QHBoxLayout(locked_tab);
    auto* lock_list = new QListWidget(win);
    lock_list->setSelectionMode(QAbstractItemView::ExtendedSelection);
    lh->addWidget(lock_list, 1);
    auto* lm = new QVBoxLayout();
    auto* lock_b = new QPushButton("Lock", win);
    auto* unlock_b = new QPushButton("Unlock", win);
    lm->addWidget(lock_b);
    lm->addWidget(unlock_b);
    lm->addStretch(1);
    lh->addLayout(lm);
    tabs->addTab(locked_tab, "Locked");
    v->addWidget(tabs);
    auto node = [] { QStringList s = selection_names(); return s.isEmpty() ? QString() : s.last().section('.', 0, 0); };
    auto refresh = [keyable, hidden, lock_list, node] {
        keyable->clear();
        hidden->clear();
        lock_list->clear();
        QString n = node();
        if (n.isEmpty()) return;
        QSet<QString> kset;
        for (const QJsonValue& a : ask("listAttr", QJsonObject{{"args", QJsonArray{n}}, {"keyable", true}}).toArray()) {
            keyable->addItem(a.toString());
            kset.insert(a.toString());
        }
        for (const QJsonValue& a : ask("listAttr", args(QJsonArray{n})).toArray())
            if (!kset.contains(a.toString())) hidden->addItem(a.toString());
        for (const QJsonValue& a : ask("listAttr", QJsonObject{{"args", QJsonArray{n}}, {"keyable", true}}).toArray()) {
            bool locked = ask("getAttr", QJsonObject{{"args", QJsonArray{n + "." + a.toString()}}, {"lock", true}}).toBool();
            auto* it = new QListWidgetItem(a.toString() + (locked ? "  (locked)" : ""), lock_list);
            it->setData(Qt::UserRole, a.toString());
        }
    };
    auto keyable_set = [node](QListWidget* from, bool on) {
        QString n = node();
        for (auto* it : from->selectedItems()) exec("setAttr", QJsonObject{{"args", QJsonArray{n + "." + it->text()}}, {"keyable", on}});
    };
    QObject::connect(hide_b, &QPushButton::clicked, win, [keyable_set, keyable] { keyable_set(keyable, false); });
    QObject::connect(show_b, &QPushButton::clicked, win, [keyable_set, hidden] { keyable_set(hidden, true); });
    auto lock_set = [node, lock_list](bool on) {
        QString n = node();
        for (auto* it : lock_list->selectedItems()) exec("setAttr", QJsonObject{{"args", QJsonArray{n + "." + it->data(Qt::UserRole).toString()}}, {"lock", on}});
    };
    QObject::connect(lock_b, &QPushButton::clicked, win, [lock_set] { lock_set(true); });
    QObject::connect(unlock_b, &QPushButton::clicked, win, [lock_set] { lock_set(false); });
    refresh();
    follow_scene(win, refresh);
    return win;
}

QWidget* shape_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Shape Editor", 560, 480);
    auto* v = new QVBoxLayout(win);
    v->setContentsMargins(0, 0, 0, 0);
    v->addWidget(new ShapeEditor(win), 1);
    return win;
}


QWidget* human_ik(QWidget* parent) {
    auto* win = window_shell(parent, "Human IK", 640, 720);
    auto* v = new QVBoxLayout(win);
    v->setContentsMargins(0, 0, 0, 0);
    v->addWidget(human_ik_panel(win), 1);
    return win;
}

QWidget* expression_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Expression Editor", 560, 480);
    auto* v = new QVBoxLayout(win);
    auto* form = new QFormLayout();
    auto* name = new QLineEdit(win);
    form->addRow("Expression Name:", name);
    v->addLayout(form);
    auto* code = new QPlainTextEdit(win);
    QFont mono("monospace");
    mono.setStyleHint(QFont::Monospace);
    code->setFont(mono);
    code->setPlaceholderText("pCube1.translateY = sin(time);");
    v->addWidget(code, 1);
    auto* row = new QHBoxLayout();
    auto* create = new QPushButton("Create", win);
    auto* clear = new QPushButton("Clear", win);
    row->addWidget(create);
    row->addWidget(clear);
    v->addLayout(row);
    QObject::connect(create, &QPushButton::clicked, win, [name, code] {
        QJsonObject c{{"string", code->toPlainText()}};
        if (!name->text().isEmpty()) c["name"] = name->text();
        exec("expression", c);
    });
    QObject::connect(clear, &QPushButton::clicked, code, &QPlainTextEdit::clear);
    return win;
}

QWidget* select_constraint(QWidget* parent) {
    auto* win = window_shell(parent, "Use Constraints", 340, 300);
    auto* form = new QFormLayout(win);
    auto* mode = new QComboBox(win);
    mode->addItems({"Off", "Next Selection", "Current and Next", "All and Next"});
    auto* kind = new QComboBox(win);
    kind->addItems({"Vertex", "Edge", "Face"});
    auto* border = new QComboBox(win);
    border->addItems({"Off", "Border", "Inside"});
    auto* hard = new QComboBox(win);
    hard->addItems({"Off", "Hard", "Soft"});
    form->addRow("Constrain:", mode);
    form->addRow("Type:", kind);
    form->addRow("Border:", border);
    form->addRow("Smoothing:", hard);
    auto* apply = new QPushButton("Apply", win);
    auto* reset = new QPushButton("Reset", win);
    form->addRow(apply, reset);
    static const int kinds[] = {1, 0x8000, 8};
    QObject::connect(apply, &QPushButton::clicked, win, [=] {
        exec("polySelectConstraint", QJsonObject{{"mode", mode->currentIndex()}, {"type", kinds[kind->currentIndex()]}, {"where", border->currentIndex()}, {"smoothness", hard->currentIndex()}});
    });
    QObject::connect(reset, &QPushButton::clicked, win, [] { exec("polySelectConstraint", QJsonObject{{"disable", true}}); });
    return win;
}

QWidget* align_objects(QWidget* parent) {
    auto* win = window_shell(parent, "Align Objects", 300, 180);
    auto* form = new QFormLayout(win);
    auto* mode = new QComboBox(win);
    mode->addItems({"Min", "Mid", "Max"});
    auto* to = new QComboBox(win);
    to->addItems({"Last Selected", "Selection Average"});
    QCheckBox* ax[3];
    auto* row = new QHBoxLayout();
    const char* names[] = {"X", "Y", "Z"};
    for (int i = 0; i < 3; ++i) {
        ax[i] = new QCheckBox(names[i], win);
        row->addWidget(ax[i]);
    }
    ax[1]->setChecked(true);
    form->addRow("Align:", mode);
    form->addRow("To:", to);
    form->addRow("Axes:", row);
    auto* b = new QPushButton("Apply", win);
    form->addRow("", b);
    QCheckBox* a0 = ax[0];
    QCheckBox* a1 = ax[1];
    QCheckBox* a2 = ax[2];
    QObject::connect(b, &QPushButton::clicked, win, [=] {
        QStringList sel = selection_names();
        if (sel.size() < 2) {
            log("// Warning: select two or more objects to align //", 4);
            return;
        }
        // Bounds of each, by the box each mesh reports.
        auto bounds = [](const QString& n) -> QVector<double> {
            QJsonArray b = ask("xform", QJsonObject{{"args", QJsonArray{n}}, {"query", true}, {"boundingBox", true}, {"worldSpace", true}}).toArray();
            QVector<double> out;
            for (const QJsonValue& x : b) out << x.toDouble();
            if (out.size() != 6) {
                QJsonArray t = ask("xform", QJsonObject{{"args", QJsonArray{n}}, {"query", true}, {"worldSpace", true}, {"rotatePivot", true}}).toArray();
                out.clear();
                for (int k = 0; k < 6; ++k) out << t.at(k % 3).toDouble();
            }
            return out;
        };
        int m = mode->currentIndex();
        auto value = [m](const QVector<double>& b, int axis) { return m == 0 ? b[axis] : m == 2 ? b[axis + 3] : (b[axis] + b[axis + 3]) * 0.5; };
        bool on[3] = {a0->isChecked(), a1->isChecked(), a2->isChecked()};
        QVector<double> target(3, 0.0);
        if (to->currentIndex() == 0) {
            auto b0 = bounds(sel.last());
            for (int k = 0; k < 3; ++k) target[k] = value(b0, k);
        } else {
            for (const QString& n : sel) {
                auto b0 = bounds(n);
                for (int k = 0; k < 3; ++k) target[k] += value(b0, k) / sel.size();
            }
        }
        for (const QString& n : sel) {
            auto b0 = bounds(n);
            QJsonArray by;
            for (int k = 0; k < 3; ++k) by.append(on[k] ? target[k] - value(b0, k) : 0.0);
            exec("move", QJsonObject{{"args", QJsonArray{n}}, {"by", by}});
        }
    });
    return win;
}

QWidget* search_replace(QWidget* parent) {
    auto* win = window_shell(parent, "Search and Replace Names", 340, 170);
    auto* form = new QFormLayout(win);
    auto* a = new QLineEdit(win);
    auto* b = new QLineEdit(win);
    auto* scope = new QComboBox(win);
    scope->addItems({"Hierarchy", "Selected", "All"});
    form->addRow("Search for:", a);
    form->addRow("Replace with:", b);
    form->addRow("Operate on:", scope);
    auto* go = new QPushButton("Replace", win);
    form->addRow("", go);
    QObject::connect(go, &QPushButton::clicked, win, [=] {
        if (a->text().isEmpty()) return;
        QStringList targets;
        std::function<void(const QJsonArray&, bool)> walk = [&](const QJsonArray& nodes, bool under) {
            QStringList sel = selection_names();
            for (const QJsonValue& v : nodes) {
                QJsonObject o = v.toObject();
                bool picked = sel.contains(o["path"].toString()) || sel.contains(o["name"].toString());
                bool take = scope->currentIndex() == 2 || (scope->currentIndex() == 1 && picked) || (scope->currentIndex() == 0 && (picked || under));
                if (take && o["type"].toString() != "objectSet") targets << o["path"].toString();
                walk(o["children"].toArray(), under || picked);
            }
        };
        walk(query("outliner", "shapes").toArray(), false);
        std::sort(targets.begin(), targets.end(), [](const QString& x, const QString& y) { return x.count('|') > y.count('|'); });
        for (const QString& p : targets) {
            QString leaf = p.section('|', -1);
            QString to = leaf;
            to.replace(a->text(), b->text());
            if (to != leaf) exec("rename", args(QJsonArray{p, to}));
        }
    });
    return win;
}

QWidget* add_attribute(QWidget* parent) {
    auto* win = window_shell(parent, "Add Attribute", 380, 360);
    auto* form = new QFormLayout(win);
    auto* name = new QLineEdit(win);
    auto* nice = new QLineEdit(win);
    auto* make = new QComboBox(win);
    make->addItems({"Keyable", "Displayable", "Hidden"});
    auto* type = new QComboBox(win);
    type->addItems({"Float", "Integer", "Boolean", "String", "Enum", "Vector"});
    static const char* types[] = {"double", "long", "bool", "string", "enum", "double3"};
    auto* lo = new QLineEdit(win);
    auto* hi = new QLineEdit(win);
    auto* dflt = new QLineEdit(win);
    auto* en = new QLineEdit(win);
    en->setPlaceholderText("Green:Blue:");
    form->addRow("Long name:", name);
    form->addRow("Nice name:", nice);
    form->addRow("Make attribute:", make);
    form->addRow("Data type:", type);
    form->addRow("Minimum:", lo);
    form->addRow("Maximum:", hi);
    form->addRow("Default:", dflt);
    form->addRow("Enum names:", en);
    auto* row = new QHBoxLayout();
    auto* ok = new QPushButton("OK", win);
    auto* add = new QPushButton("Add", win);
    auto* cancel = new QPushButton("Cancel", win);
    row->addWidget(ok);
    row->addWidget(add);
    row->addWidget(cancel);
    form->addRow(row);
    auto apply = [=] {
        if (name->text().trimmed().isEmpty()) return false;
        int t = type->currentIndex();
        QJsonObject c{{"longName", name->text().trimmed()}};
        if (t == 3) c["dataType"] = "string";
        else c["attributeType"] = types[t];
        if (!nice->text().isEmpty()) c["niceName"] = nice->text();
        if (make->currentIndex() == 0) c["keyable"] = true;
        if (make->currentIndex() == 2) c["hidden"] = true;
        if (!lo->text().isEmpty() && t < 2) c["minValue"] = lo->text().toDouble();
        if (!hi->text().isEmpty() && t < 2) c["maxValue"] = hi->text().toDouble();
        if (!dflt->text().isEmpty() && t < 3) c["defaultValue"] = dflt->text().toDouble();
        if (t == 4) c["enumName"] = en->text();
        return !exec("addAttr", c).isUndefined();
    };
    QObject::connect(ok, &QPushButton::clicked, win, [apply, win] { if (apply()) win->close(); });
    QObject::connect(add, &QPushButton::clicked, win, [apply, name] { if (apply()) name->clear(); });
    QObject::connect(cancel, &QPushButton::clicked, win, &QWidget::close);
    return win;
}

// Lists a node's added attributes; used for Edit and Delete.
QWidget* attribute_list(QWidget* parent, bool editing) {
    auto* win = window_shell(parent, editing ? "Edit Attribute" : "Delete Attribute", 360, 340);
    auto* v = new QVBoxLayout(win);
    auto* head = new QLabel(win);
    v->addWidget(head);
    auto* list = new QListWidget(win);
    list->setSelectionMode(editing ? QAbstractItemView::SingleSelection : QAbstractItemView::ExtendedSelection);
    v->addWidget(list, 1);
    auto refresh = [head, list] {
        QStringList s = selection_names();
        list->clear();
        if (s.isEmpty()) {
            head->setText("Select a node.");
            return;
        }
        QString n = s.last().section('.', 0, 0);
        head->setText(n);
        for (const QJsonValue& a : ask("listAttr", QJsonObject{{"args", QJsonArray{n}}, {"userDefined", true}}).toArray()) list->addItem(a.toString());
    };
    if (editing) {
        auto* form = new QFormLayout();
        auto* key = new QCheckBox("Keyable", win);
        auto* lock = new QCheckBox("Locked", win);
        form->addRow(key);
        form->addRow(lock);
        v->addLayout(form);
        auto* apply = new QPushButton("Apply", win);
        v->addWidget(apply);
        QObject::connect(apply, &QPushButton::clicked, win, [head, list, key, lock] {
            if (!list->currentItem()) return;
            QString plug = head->text() + "." + list->currentItem()->text();
            exec("setAttr", QJsonObject{{"args", QJsonArray{plug}}, {"keyable", key->isChecked()}});
            exec("setAttr", QJsonObject{{"args", QJsonArray{plug}}, {"lock", lock->isChecked()}});
        });
    } else {
        auto* del = new QPushButton("Delete", win);
        v->addWidget(del);
        QObject::connect(del, &QPushButton::clicked, win, [head, list] {
            for (auto* it : list->selectedItems()) exec("deleteAttr", args(QJsonArray{head->text() + "." + it->text()}));
        });
    }
    refresh();
    follow_scene(win, refresh);
    return win;
}

// Grid Options, as the reference has them: size, grid lines and subdivisions, the three colours, which lines and
// numbers show. Kept in the settings and given to the grid command, which every view follows.
struct GridPrefs {
    double size, every;
    int divisions;
    bool axes, bold, lines, subdivisions;
    QString persp, ortho; // "axis", "edge" or "hidden"
    QColor axis_color, line_color, sub_color;
};

static GridPrefs read_grid_prefs() {
    QSettings s;
    GridPrefs g;
    g.size = s.value("prefs/gridSize", 12.0).toDouble();
    // Older settings kept the subdivision spacing alone, with a grid line every 5 of them.
    g.every = s.value("prefs/gridLinesEvery", s.value("prefs/gridSpacing", 1.0).toDouble() * 5.0).toDouble();
    g.divisions = s.value("prefs/gridDivisions", 5).toInt();
    g.axes = s.value("prefs/gridAxes", true).toBool();
    g.bold = s.value("prefs/gridAxesBold", true).toBool();
    g.lines = s.value("prefs/gridLines", true).toBool();
    g.subdivisions = s.value("prefs/gridSubdivisions", true).toBool();
    g.persp = s.value("prefs/gridPerspNumbers", "hidden").toString();
    g.ortho = s.value("prefs/gridOrthoNumbers", "hidden").toString();
    g.axis_color = s.value("prefs/gridAxisColor", QColor(0, 0, 0)).value<QColor>();
    g.line_color = s.value("prefs/gridLineColor", QColor(70, 70, 70)).value<QColor>();
    g.sub_color = s.value("prefs/gridSubdivisionColor", QColor(105, 105, 105)).value<QColor>();
    return g;
}

static void write_grid_prefs(const GridPrefs& g) {
    QSettings s;
    s.setValue("prefs/gridSize", g.size);
    s.setValue("prefs/gridLinesEvery", g.every);
    s.setValue("prefs/gridDivisions", g.divisions);
    s.setValue("prefs/gridAxes", g.axes);
    s.setValue("prefs/gridAxesBold", g.bold);
    s.setValue("prefs/gridLines", g.lines);
    s.setValue("prefs/gridSubdivisions", g.subdivisions);
    s.setValue("prefs/gridPerspNumbers", g.persp);
    s.setValue("prefs/gridOrthoNumbers", g.ortho);
    s.setValue("prefs/gridAxisColor", g.axis_color);
    s.setValue("prefs/gridLineColor", g.line_color);
    s.setValue("prefs/gridSubdivisionColor", g.sub_color);
}

static void send_grid_prefs(const GridPrefs& g) {
    auto rgb = [](const char* name, const QColor& c) {
        exec_quiet("displayRGBColor", QJsonObject{{"args", QJsonArray{name, c.redF(), c.greenF(), c.blueF()}}});
    };
    rgb("gridAxis", g.axis_color);
    rgb("gridHighlight", g.line_color);
    rgb("gridColor", g.sub_color);
    QJsonObject call{{"size", g.size}, {"spacing", g.every}, {"divisions", g.divisions}, {"displayAxes", g.axes},
                     {"displayAxesBold", g.bold}, {"displayGridLines", g.lines}, {"displayDivisionLines", g.subdivisions},
                     {"displayPerspectiveLabels", g.persp != "hidden"}, {"displayOrthographicLabels", g.ortho != "hidden"}};
    if (g.persp != "hidden") call["perspectiveLabelPosition"] = g.persp;
    if (g.ortho != "hidden") call["orthographicLabelPosition"] = g.ortho;
    exec_quiet("grid", call);
}

QWidget* grid_options(QWidget* parent) {
    auto* win = window_shell(parent, "Grid Options", 380, 470);
    auto* outer = new QVBoxLayout(win);
    // Save and Reset Settings in the Edit menu, as in every option window (#139).
    auto* bar = new QMenuBar(win);
    QMenu* edit_menu = bar->addMenu("Edit");
    outer->setMenuBar(bar);
    auto* form = new QFormLayout();
    outer->addLayout(form);
    const GridPrefs g = read_grid_prefs();
    auto* size = new QDoubleSpinBox(win);
    size->setRange(1, 100000);
    size->setDecimals(3);
    size->setValue(g.size);
    auto* every = new QDoubleSpinBox(win);
    every->setRange(0.001, 10000);
    every->setDecimals(3);
    every->setValue(g.every);
    auto* divisions = new QSpinBox(win);
    divisions->setRange(1, 1000);
    divisions->setValue(g.divisions);
    auto section = [win, form](const QString& t) {
        auto* l = new QLabel("<b>" + t + "</b>", win);
        form->addRow(l);
    };
    section("Size");
    form->addRow("Length and width:", size);
    form->addRow("Grid lines every:", every);
    form->addRow("Subdivisions:", divisions);
    section("Color");
    auto color_button = [win](const QColor& c) {
        auto* b = new QPushButton(win);
        b->setFixedSize(60, 20);
        b->setProperty("color", c);
        b->setStyleSheet(QString("background:%1; border:1px solid #222;").arg(c.name()));
        QObject::connect(b, &QPushButton::clicked, b, [b] {
            const QColor c = QColorDialog::getColor(b->property("color").value<QColor>(), b);
            if (!c.isValid()) return;
            b->setProperty("color", c);
            b->setStyleSheet(QString("background:%1; border:1px solid #222;").arg(c.name()));
        });
        return b;
    };
    auto* axis_c = color_button(g.axis_color);
    auto* line_c = color_button(g.line_color);
    auto* sub_c = color_button(g.sub_color);
    form->addRow("Axes:", axis_c);
    form->addRow("Grid lines && numbers:", line_c);
    form->addRow("Subdivision lines:", sub_c);
    section("Display");
    auto box = [win](const QString& t, bool on) {
        auto* c = new QCheckBox(t, win);
        c->setChecked(on);
        return c;
    };
    auto* axes = box("Axes", g.axes);
    auto* bold = box("Thicker line for axes", g.bold);
    auto* lines = box("Grid lines", g.lines);
    auto* subs = box("Subdivision lines", g.subdivisions);
    form->addRow(axes);
    form->addRow(bold);
    form->addRow(lines);
    form->addRow(subs);
    auto numbers = [win](const QString& v) {
        auto* c = new QComboBox(win);
        c->addItem("On axes", "axis");
        c->addItem("Along edge", "edge");
        c->addItem("Hidden", "hidden");
        c->setCurrentIndex(std::max(0, c->findData(v)));
        return c;
    };
    auto* persp = numbers(g.persp);
    auto* ortho = numbers(g.ortho);
    form->addRow("Perspective grid numbers:", persp);
    form->addRow("Orthographic grid numbers:", ortho);
    auto* row = new QHBoxLayout();
    auto* apply_close = new QPushButton("Apply and Close", win);
    auto* apply = new QPushButton("Apply", win);
    auto* close = new QPushButton("Close", win);
    row->addWidget(apply_close);
    row->addWidget(apply);
    row->addWidget(close);
    outer->addStretch(1);
    outer->addLayout(row);
    auto take = [=] {
        GridPrefs n;
        n.size = size->value();
        n.every = every->value();
        n.divisions = divisions->value();
        n.axes = axes->isChecked();
        n.bold = bold->isChecked();
        n.lines = lines->isChecked();
        n.subdivisions = subs->isChecked();
        n.persp = persp->currentData().toString();
        n.ortho = ortho->currentData().toString();
        n.axis_color = axis_c->property("color").value<QColor>();
        n.line_color = line_c->property("color").value<QColor>();
        n.sub_color = sub_c->property("color").value<QColor>();
        write_grid_prefs(n);
        send_grid_prefs(n);
    };
    QObject::connect(apply, &QPushButton::clicked, win, take);
    QObject::connect(apply_close, &QPushButton::clicked, win, [take, win] {
        take();
        win->close();
    });
    QObject::connect(close, &QPushButton::clicked, win, &QWidget::close);
    edit_menu->addAction("Save Settings", win, take);
    QObject::connect(edit_menu->addAction("Reset Settings"), &QAction::triggered, win, [=] {
        QSettings s;
        for (const char* k : {"gridSize", "gridLinesEvery", "gridSpacing", "gridDivisions", "gridAxes", "gridAxesBold", "gridLines", "gridSubdivisions",
                              "gridPerspNumbers", "gridOrthoNumbers", "gridAxisColor", "gridLineColor", "gridSubdivisionColor"})
            s.remove(QString("prefs/") + k);
        const GridPrefs d = read_grid_prefs();
        size->setValue(d.size);
        every->setValue(d.every);
        divisions->setValue(d.divisions);
        axes->setChecked(d.axes);
        bold->setChecked(d.bold);
        lines->setChecked(d.lines);
        subs->setChecked(d.subdivisions);
        persp->setCurrentIndex(persp->findData(d.persp));
        ortho->setCurrentIndex(ortho->findData(d.ortho));
        for (auto [b, c] : {std::pair{axis_c, d.axis_color}, std::pair{line_c, d.line_color}, std::pair{sub_c, d.sub_color}}) {
            b->setProperty("color", c);
            b->setStyleSheet(QString("background:%1; border:1px solid #222;").arg(c.name()));
        }
        send_grid_prefs(d);
    });
    return win;
}

} // namespace

void apply_grid_prefs() {
    send_grid_prefs(read_grid_prefs());
}

// The ViewCube preferences, as viewManip takes them.
void apply_view_cube_prefs() {
    QSettings s;
    const QString pos = s.value("prefs/viewCubePosition", "Top Right").toString();
    const QString corner = pos == "Top Left" ? "topLeft" : pos == "Bottom Right" ? "bottomRight" : pos == "Bottom Left" ? "bottomLeft" : "topRight";
    QJsonObject call{{"visible", s.value("prefs/viewCubeShow", true).toBool()},
                     {"size", s.value("prefs/viewCubeSize", "Normal").toString().toLower()},
                     {"minOpacity", s.value("prefs/viewCubeOpacity", "50%").toString().chopped(1).toDouble() / 100.0},
                     {"dragSnap", s.value("prefs/viewCubeSnap", false).toBool()},
                     {"preserveSceneUp", s.value("prefs/viewCubeSceneUp", true).toBool()},
                     {"zoomToFitScene", s.value("prefs/viewCubeFit", true).toBool()}};
    call[corner] = true;
    exec_quiet("viewManip", call);
}

bool has_command_named(const QString& name) {
    for (const QJsonValue& c : query("commands").toArray())
        if (c.toObject()["name"].toString() == name) return true;
    return false;
}

// The UV Set Editor: the UV sets of the selected mesh, in their order. Picking one makes it current, which is the set
// the UV Editor, the UV tools and the views show. New, Copy, Rename and Delete act on the one picked.
static QWidget* uv_set_editor(QWidget* parent) {
    auto* w = window_shell(parent, "UV Set Editor", 380, 300);
    w->setObjectName("uvSetEditor");
    auto* v = new QVBoxLayout(w);
    auto* target = new QLabel(w);
    v->addWidget(target);
    auto* h = new QHBoxLayout();
    auto* list = new QListWidget(w);
    list->setObjectName("uvSetList");
    list->setEditTriggers(QAbstractItemView::EditKeyPressed | QAbstractItemView::DoubleClicked);
    h->addWidget(list, 1);
    auto* buttons = new QVBoxLayout();
    h->addLayout(buttons);
    v->addLayout(h, 1);
    auto* hint = new QLabel("Pick a set to make it current. Double-click to rename.", w);
    hint->setWordWrap(true);
    hint->setStyleSheet("color: palette(mid);");
    v->addWidget(hint);

    // The mesh asked about: the last thing selected, by its object.
    auto mesh = [] {
        const QStringList s = selection_names();
        return s.isEmpty() ? QString() : s.last().section('.', 0, 0);
    };
    auto sets = [](const QString& m) {
        QStringList out;
        if (m.isEmpty()) return out;
        for (const QJsonValue& x : ask("polyUVSet", QJsonObject{{"args", QJsonArray{m}}, {"query", true}, {"queried", QJsonArray{"allUVSets"}}}).toArray()) out << x.toString();
        return out;
    };
    auto current = [](const QString& m) {
        if (m.isEmpty()) return QString();
        const QJsonValue c = ask("polyUVSet", QJsonObject{{"args", QJsonArray{m}}, {"query", true}, {"queried", QJsonArray{"currentUVSet"}}});
        return c.isArray() ? c.toArray().at(0).toString() : c.toString();
    };
    auto reload = [list, target, mesh, sets, current, buttons] {
        const QString m = mesh();
        const QStringList names = sets(m);
        const QString cur = current(m);
        QSignalBlocker block(list);
        list->clear();
        for (const QString& n : names) {
            auto* item = new QListWidgetItem(n, list);
            item->setFlags(item->flags() | Qt::ItemIsEditable);
            item->setData(Qt::UserRole, n);
            if (n == cur) {
                QFont f = item->font();
                f.setBold(true);
                item->setFont(f);
                list->setCurrentItem(item);
            }
        }
        target->setText(names.isEmpty() ? "Select a polygon mesh." : m);
        for (int i = 0; i < buttons->count(); ++i)
            if (QWidget* b = buttons->itemAt(i)->widget()) b->setEnabled(!names.isEmpty());
    };
    auto picked = [list] { return list->currentItem() ? list->currentItem()->data(Qt::UserRole).toString() : QString(); };
    auto run = [mesh, reload](QJsonObject call) {
        const QString m = mesh();
        if (m.isEmpty()) return;
        call["args"] = QJsonArray{m};
        exec("polyUVSet", call);
        Host::get()->poke();
        reload();
    };
    auto button = [&](const char* label, const char* tip, std::function<void()> f) {
        auto* b = new QPushButton(label, w);
        b->setToolTip(tip);
        QObject::connect(b, &QPushButton::clicked, w, f);
        buttons->addWidget(b);
    };
    button("New", "An empty UV set", [run] { run(QJsonObject{{"create", true}}); });
    button("Copy", "A copy of the picked set", [run, picked] {
        if (!picked().isEmpty()) run(QJsonObject{{"copy", true}, {"uvSet", picked()}});
    });
    button("Rename", "Rename the picked set", [list] {
        if (list->currentItem()) list->editItem(list->currentItem());
    });
    button("Delete", "Delete the picked set", [run, picked] {
        if (!picked().isEmpty()) run(QJsonObject{{"delete", true}, {"uvSet", picked()}});
    });
    buttons->addStretch(1);
    // Picking makes it current; an edited name renames it.
    QObject::connect(list, &QListWidget::itemClicked, w, [run, current, mesh](QListWidgetItem* item) {
        const QString name = item->data(Qt::UserRole).toString();
        if (!name.isEmpty() && name != current(mesh())) run(QJsonObject{{"currentUVSet", true}, {"uvSet", name}});
    });
    QObject::connect(list, &QListWidget::itemChanged, w, [run, reload](QListWidgetItem* item) {
        const QString was = item->data(Qt::UserRole).toString(), now = item->text().trimmed();
        if (now.isEmpty() || now == was) {
            reload();
            return;
        }
        run(QJsonObject{{"rename", true}, {"uvSet", was}, {"newUVSet", now}});
    });
    // Follows the selection and whatever else changes the sets, but not while a name is being typed.
    auto shown = std::make_shared<QString>();
    follow_scene(w, [list, reload, mesh, sets, current, shown] {
        if (list->findChild<QLineEdit*>()) return;
        const QString m = mesh();
        const QString now = m + "|" + sets(m).join("|") + "|" + current(m);
        if (now == *shown) return;
        *shown = now;
        reload();
    });
    reload();
    return w;
}

// ---------------------------------------------------------------------------------------------------------------------
// Interactive Groom Editor: descriptions, their modifier stacks from the top down, and sculpt layers with weight,
// visibility and the active one. A click selects the node for the Attribute Editor.

QWidget* groom_editor(QWidget* parent) {
    auto* win = window_shell(parent, "Interactive Groom Editor", 540, 520);
    auto* v = new QVBoxLayout(win);
    auto* bar = new QToolBar(win);
    v->addWidget(bar);
    auto* tree = new QTreeWidget(win);
    tree->setColumnCount(3);
    tree->setHeaderLabels({"Name", "Weight", "On"});
    tree->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    tree->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    tree->header()->setSectionResizeMode(2, QHeaderView::ResizeToContents);
    v->addWidget(tree, 1);
    auto* updating = new bool(false);
    // Roles: the node path, what the row is ("description", "modifier", "layer"), the layer's index, its description.
    const int NODE = Qt::UserRole, KIND = Qt::UserRole + 1, INDEX = Qt::UserRole + 2, DESC = Qt::UserRole + 3;
    auto refresh = [tree, updating, NODE, KIND, INDEX, DESC] {
        *updating = true;
        QSet<QString> open;
        for (int i = 0; i < tree->topLevelItemCount(); ++i)
            if (!tree->topLevelItem(i)->isExpanded()) open.insert("closed:" + tree->topLevelItem(i)->data(0, NODE).toString());
        QString current = tree->currentItem() ? tree->currentItem()->data(0, NODE).toString() + "/" + tree->currentItem()->data(0, INDEX).toString() : QString();
        tree->clear();
        for (const QJsonValue& dv : ask("xjenGroomInfo", QJsonObject()).toArray()) {
            QJsonObject d = dv.toObject();
            QString tr = d["transform"].toString();
            auto* top = new QTreeWidgetItem(tree, {QString("%1  (%2 splines)").arg(tr).arg(d["splines"].toInt())});
            QFont f = top->font(0);
            f.setBold(true);
            top->setFont(0, f);
            top->setData(0, NODE, tr);
            top->setData(0, KIND, "description");
            top->setData(0, DESC, tr);
            QJsonObject sculpt = d["sculpt"].toArray().isEmpty() ? QJsonObject() : d["sculpt"].toArray()[0].toObject();
            for (const QJsonValue& mv : d["stack"].toArray()) {
                QJsonObject m = mv.toObject();
                QString node = m["node"].toString();
                QString ty = m["type"].toString();
                QString label = ty.startsWith("xgmModifier") ? ty.mid(11) : ty == "xgmSplineBase" ? "Base" : ty.mid(3);
                auto* it = new QTreeWidgetItem(top, {label + "   " + node});
                it->setData(0, NODE, node);
                it->setData(0, KIND, "modifier");
                it->setData(0, DESC, tr);
                if (ty != "xgmSplineBase") {
                    it->setFlags(it->flags() | Qt::ItemIsUserCheckable);
                    it->setCheckState(2, m["mute"].toBool() ? Qt::Unchecked : Qt::Checked);
                }
                if (ty == "xgmModifierSculpt" && sculpt["node"].toString() == node) {
                    int active = sculpt["active"].toInt();
                    QJsonArray layers = sculpt["layers"].toArray();
                    for (int k = layers.size() - 1; k >= 0; --k) {
                        QJsonObject l = layers[k].toObject();
                        auto* li = new QTreeWidgetItem(it, {(k == active ? QString::fromUtf8("● ") : QString("   ")) + l["name"].toString(), QString::number(l["weight"].toDouble(), 'f', 3)});
                        li->setFlags(li->flags() | Qt::ItemIsUserCheckable | Qt::ItemIsEditable);
                        li->setCheckState(2, l["visible"].toBool() ? Qt::Checked : Qt::Unchecked);
                        li->setData(0, NODE, node);
                        li->setData(0, KIND, "layer");
                        li->setData(0, INDEX, k);
                        li->setData(0, DESC, tr);
                        if (current == node + "/" + QString::number(k)) tree->setCurrentItem(li);
                    }
                    it->setExpanded(true);
                }
                if (current == node + "/") tree->setCurrentItem(it);
            }
            top->setExpanded(!open.contains("closed:" + tr));
        }
        *updating = false;
    };
    // The command runs once the tree's own handling is over: the scene change it makes rebuilds the tree, which
    // deletes the item the signal is still about (#202).
    QObject::connect(tree, &QTreeWidget::itemChanged, win, [win, updating, NODE, KIND, INDEX](QTreeWidgetItem* it, int col) {
        if (*updating) return;
        const QString node = it->data(0, NODE).toString();
        const QString kind = it->data(0, KIND).toString();
        const int k = it->data(0, INDEX).toInt();
        const bool on = it->checkState(2) == Qt::Checked;
        const double weight = it->text(1).toDouble();
        const QString name = it->text(0).trimmed().remove(QString::fromUtf8("●")).trimmed();
        QTimer::singleShot(0, win, [=] {
            if (kind == "modifier" && col == 2) {
                exec("setAttr", QJsonObject{{"args", QJsonArray{node + ".mute", !on}}});
            } else if (kind == "layer") {
                if (col == 2) exec("setAttr", QJsonObject{{"args", QJsonArray{QString("%1.layer[%2].layerVisibility").arg(node).arg(k), on}}});
                if (col == 1) exec("setAttr", QJsonObject{{"args", QJsonArray{QString("%1.layer[%2].layerWeight").arg(node).arg(k), weight}}});
                if (col == 0) exec("setAttr", QJsonObject{{"args", QJsonArray{QString("%1.layer[%2].layerName").arg(node).arg(k), name}}, {"type", "string"}});
            }
        });
    });
    QObject::connect(tree, &QTreeWidget::itemClicked, win, [win, updating, NODE](QTreeWidgetItem* it, int) {
        if (*updating) return;
        const QString node = it->data(0, NODE).toString();
        QTimer::singleShot(0, win, [node] { exec("select", QJsonObject{{"args", QJsonArray{node}}, {"replace", true}}); });
    });
    QObject::connect(tree, &QTreeWidget::itemDoubleClicked, win, [win, KIND, INDEX, DESC](QTreeWidgetItem* it, int col) {
        if (it->data(0, KIND).toString() != "layer" || col != 0) return;
        const QString desc = it->data(0, DESC).toString();
        const int k = it->data(0, INDEX).toInt();
        QTimer::singleShot(0, win, [desc, k] { exec("xjenSculptLayer", QJsonObject{{"args", QJsonArray{desc}}, {"active", k}}); });
    });
    auto desc_of = [tree, DESC]() -> QString {
        if (tree->currentItem()) return tree->currentItem()->data(0, DESC).toString();
        if (tree->topLevelItemCount()) return tree->topLevelItem(0)->data(0, DESC).toString();
        return QString();
    };
    bar->addAction("Create", [] { exec("xjenCreateGroom", QJsonObject()); });
    auto* add = new QMenu(win);
    for (auto [label, id] : std::initializer_list<std::pair<const char*, const char*>>{{"Clump", "clump"}, {"Noise", "noise"}, {"Cut", "cut"}, {"Collide", "collide"}, {"Displace", "displace"}, {"Guide", "guide"}, {"Linear Wire", "linearWire"}, {"Curve to Spline", "curveToSpline"}, {"Scale", "scale"}, {"Sculpt", "sculpt"}}) {
        QString t = id;
        add->addAction(label, [t, desc_of] {
            QString d = desc_of();
            if (!d.isEmpty()) exec("xjenAddModifier", QJsonObject{{"args", QJsonArray{d}}, {"type", t}});
        });
    }
    QAction* add_a = bar->addAction("Add Modifier");
    add_a->setMenu(add);
    QObject::connect(add_a, &QAction::triggered, win, [add] { add->popup(QCursor::pos()); });
    bar->addAction("Remove", [tree, NODE, KIND, INDEX, DESC] {
        auto* it = tree->currentItem();
        if (!it) return;
        if (it->data(0, KIND).toString() == "modifier") exec("xjenRemoveModifier", QJsonObject{{"args", QJsonArray{it->data(0, NODE).toString()}}});
        if (it->data(0, KIND).toString() == "layer") exec("xjenSculptLayer", QJsonObject{{"args", QJsonArray{it->data(0, DESC).toString()}}, {"remove", it->data(0, INDEX).toInt()}});
        if (it->data(0, KIND).toString() == "description") exec("delete", QJsonObject{{"args", QJsonArray{it->data(0, NODE).toString()}}});
    });
    bar->addSeparator();
    bar->addAction("Add Layer", [desc_of] {
        QString d = desc_of();
        if (!d.isEmpty()) exec("xjenSculptLayer", QJsonObject{{"args", QJsonArray{d}}, {"add", "Layer"}});
    });
    bar->addAction("Merge Visible", [desc_of] {
        QString d = desc_of();
        if (!d.isEmpty()) exec("xjenSculptLayer", QJsonObject{{"args", QJsonArray{d}}, {"mergeVisible", true}});
    });
    auto* more = new QMenu(win);
    more->addAction("Convert to Polygons", [desc_of] {
        QString d = desc_of();
        if (!d.isEmpty()) exec("xjenConvertToPolygons", QJsonObject{{"args", QJsonArray{d}}});
    });
    more->addAction("Export Preset...", [win, desc_of] {
        QString d = desc_of();
        if (d.isEmpty()) return;
        QString f = QFileDialog::getSaveFileName(win, "Export Preset", d + ".json", "XJEN preset (*.json)");
        if (!f.isEmpty()) exec("xjenPreset", QJsonObject{{"args", QJsonArray{d}}, {"export", f}});
    });
    more->addAction("Import Preset...", [win] {
        QString f = QFileDialog::getOpenFileName(win, "Import Preset onto the selected mesh", QString(), "XJEN preset (*.json)");
        if (!f.isEmpty()) exec("xjenPreset", QJsonObject{{"import", f}});
    });
    more->addAction("Select Splines on Selected Faces", [] { exec("xgmSplineSelect", QJsonObject{{"replaceBySelectedFaces", true}}); });
    more->addAction("Freeze Selected Splines", [] { exec("xgmSplineSelect", QJsonObject{{"convertToFreeze", true}}); });
    QAction* more_a = bar->addAction("Description");
    more_a->setMenu(more);
    QObject::connect(more_a, &QAction::triggered, win, [more] { more->popup(QCursor::pos()); });
    refresh();
    follow_scene(win, refresh);
    return win;
}

QWidget* make_editor_window(const QString& id, QWidget* parent) {
    if (id == "interactiveGroomEditor") return groom_editor(parent);
    if (id == "spreadSheet") return spread_sheet(parent);
    if (id == "componentEditor") return component_editor(parent);
    if (id == "connectionEditor") return connection_editor(parent);
    if (id == "hypergraphHierarchy") return hypergraph_hierarchy(parent);
    if (id == "hypergraphConnections") return hypergraph_connections(parent);
    if (id == "namespaceEditor") return namespace_editor(parent);
    if (id == "channelControl") return channel_control(parent);
    if (id == "shapeEditor") return shape_editor(parent);
    if (id == "humanIK") return human_ik(parent);
    if (id == "expressionEditor") return expression_editor(parent);
    if (id == "polySelectConstraint") return select_constraint(parent);
    if (id == "alignObjects") return align_objects(parent);
    if (id == "searchReplace") return search_replace(parent);
    if (id == "addAttribute") return add_attribute(parent);
    if (id == "editAttribute") return attribute_list(parent, true);
    if (id == "deleteAttribute") return attribute_list(parent, false);
    if (id == "gridOptions") return grid_options(parent);
    if (id == "uvSetEditor") return uv_set_editor(parent);
    return nullptr;
}

// ---------------------------------------------------------------------------------------------------------------------

InViewEditor::InViewEditor(QWidget* parent) : QFrame(parent) {
    // Native, or the view's own native window would paint over it.
    setAttribute(Qt::WA_NativeWindow);
    setObjectName("inViewEditor");
    setFrameShape(QFrame::StyledPanel);
    body_ = new QVBoxLayout(this);
    body_->setContentsMargins(6, 4, 6, 6);
    hide();
}

namespace {
// A row's label: drag it sideways to change the number. Previews are silent, the release runs one setAttr.
class DragLabel : public QLabel {
public:
    DragLabel(const QString& text, QString plug, double lo, double hi, bool is_int, QDoubleSpinBox* spin, QWidget* parent)
        : QLabel(text, parent), plug_(std::move(plug)), lo_(lo), hi_(hi), is_int_(is_int), spin_(spin) {
        setCursor(Qt::SizeHorCursor);
    }
protected:
    void mousePressEvent(QMouseEvent* e) override {
        from_ = e->globalPosition().x();
        start_ = spin_->value();
        dragged_ = false;
    }
    void mouseMoveEvent(QMouseEvent* e) override {
        if (!(e->buttons() & Qt::LeftButton)) return;
        double dx = e->globalPosition().x() - from_;
        double v = std::clamp(start_ + dx * (hi_ - lo_) / (is_int_ ? 200.0 : 1000.0), lo_, hi_);
        if (is_int_) v = std::round(v);
        QSignalBlocker block(spin_);
        spin_->setValue(v);
        if (dragged_) aztek_retract();
        exec_quiet("setAttr", QJsonObject{{"args", QJsonArray{plug_, is_int_ ? QJsonValue(qint64(v)) : QJsonValue(v)}}});
        dragged_ = true;
    }
    void mouseReleaseEvent(QMouseEvent*) override {
        if (!dragged_) return;
        aztek_retract();
        double v = spin_->value();
        exec("setAttr", QJsonObject{{"args", QJsonArray{plug_, is_int_ ? QJsonValue(qint64(v)) : QJsonValue(v)}}});
        dragged_ = false;
    }
private:
    QString plug_;
    double lo_, hi_, from_ = 0, start_ = 0;
    bool is_int_, dragged_ = false;
    QDoubleSpinBox* spin_;
};
} // namespace

// The rows the node's table names. False when it names none.
void InViewEditor::update_values() {
    if (!isVisible()) return;
    QJsonObject info = query("nodeManip").toObject();
    if (info["node"].toString() != shown_node_) {
        if (!shown_node_.isEmpty() && !info["fields"].toArray().isEmpty()) show_fields();
        return;
    }
    for (const QJsonValue& fv : info["fields"].toArray()) {
        QJsonObject f = fv.toObject();
        QString plug = f["plug"].toString();
        if (auto* s = spins_.value(plug)) {
            if (s->hasFocus() || std::abs(s->value() - f["value"].toDouble()) < 1e-9) continue;
            QSignalBlocker block(s);
            s->setValue(f["value"].toDouble());
        } else if (auto* b = boxes_.value(plug)) {
            QSignalBlocker block(b);
            b->setChecked(f["value"].toDouble() != 0);
        }
    }
}

bool InViewEditor::show_fields() {
    QJsonObject info = query("nodeManip").toObject();
    QJsonArray fields = info["fields"].toArray();
    if (fields.isEmpty()) return false;
    shown_node_ = info["node"].toString();
    spins_.clear();
    boxes_.clear();
    delete rows_;
    rows_ = new QWidget(this);
    auto* form = new QFormLayout(rows_);
    form->setContentsMargins(0, 0, 0, 0);
    form->setVerticalSpacing(2);
    auto* title = new QLabel("<b>" + info["node"].toString() + "</b>", rows_);
    auto* close = new QPushButton("x", rows_);
    close->setFixedSize(18, 18);
    connect(close, &QPushButton::clicked, this, &QWidget::hide);
    auto* head = new QHBoxLayout();
    head->addWidget(title, 1);
    head->addWidget(close);
    form->addRow(head);
    for (const QJsonValue& fv : fields) {
        QJsonObject f = fv.toObject();
        QString plug = f["plug"].toString(), kind = f["kind"].toString();
        if (kind == "b") {
            auto* box = new QCheckBox(f["label"].toString(), rows_);
            box->setChecked(f["value"].toDouble() != 0);
            connect(box, &QCheckBox::toggled, box, [plug](bool on) { exec("setAttr", QJsonObject{{"args", QJsonArray{plug, on ? 1 : 0}}}); });
            boxes_[plug] = box;
            form->addRow(box);
            continue;
        }
        bool is_int = kind == "i";
        double lo = f["min"].toDouble(), hi = f["max"].toDouble();
        auto* e = new QDoubleSpinBox(rows_);
        // The range is for dragging; typed numbers can go past it.
        e->setRange(-1e6, 1e6);
        e->setDecimals(is_int ? 0 : 3);
        e->setSingleStep(is_int ? 1 : 0.1);
        e->setValue(f["value"].toDouble());
        e->setKeyboardTracking(false);
        connect(e, &QDoubleSpinBox::valueChanged, e, [plug, is_int](double v) {
            exec("setAttr", QJsonObject{{"args", QJsonArray{plug, is_int ? QJsonValue(qint64(v)) : QJsonValue(v)}}});
        });
        spins_[plug] = e;
        form->addRow(new DragLabel(f["label"].toString(), plug, lo, hi, is_int, e, rows_), e);
    }
    body_->addWidget(rows_);
    // A new child counts as hidden until shown, and a layout leaves hidden widgets out of its size.
    rows_->show();
    adjustSize();
    move(12, 40);
    show();
    raise();
    return true;
}

void InViewEditor::show_last() {
    if (show_fields()) return;
    shown_node_.clear();
    QJsonArray inputs = query("channelBox").toObject()["inputs"].toArray();
    if (inputs.isEmpty()) {
        hide();
        return;
    }
    // History lists newest first.
    QJsonObject node = inputs.first().toObject();
    delete rows_;
    rows_ = new QWidget(this);
    auto* form = new QFormLayout(rows_);
    form->setContentsMargins(0, 0, 0, 0);
    form->setVerticalSpacing(2);
    auto* title = new QLabel("<b>" + node["name"].toString() + "</b>", rows_);
    auto* close = new QPushButton("x", rows_);
    close->setFixedSize(18, 18);
    connect(close, &QPushButton::clicked, this, &QWidget::hide);
    auto* head = new QHBoxLayout();
    head->addWidget(title, 1);
    head->addWidget(close);
    form->addRow(head);
    int shown = 0;
    for (const QJsonValue& cv : node["channels"].toArray()) {
        QJsonObject c = cv.toObject();
        QString ty = c["type"].toString();
        if (ty != "float" && ty != "int") continue;
        if (++shown > 10) break;
        auto* e = new QDoubleSpinBox(rows_);
        e->setRange(-1e6, 1e6);
        e->setDecimals(ty == "int" ? 0 : 3);
        e->setSingleStep(ty == "int" ? 1 : 0.1);
        e->setValue(c["value"].toDouble());
        e->setKeyboardTracking(false);
        QString plug = c["plug"].toString();
        bool is_int = ty == "int";
        connect(e, &QDoubleSpinBox::valueChanged, e, [plug, is_int](double v) {
            exec("setAttr", QJsonObject{{"args", QJsonArray{plug, is_int ? QJsonValue(qint64(v)) : QJsonValue(v)}}});
        });
        form->addRow(c["nice"].toString(), e);
    }
    body_->addWidget(rows_);
    // A new child counts as hidden until shown, and a layout leaves hidden widgets out of its size.
    rows_->show();
    adjustSize();
    QWidget* p = parentWidget();
    move(p->width() - width() - 12, 40);
    show();
    raise();
}

} // namespace az

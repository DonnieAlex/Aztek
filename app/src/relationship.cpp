// Windows > Relationship Editors: one window for every kind, as the reference's: the containers on the left (display
// layers, sets, deformer sets, shading groups, character sets, lights or objects for light linking), the objects on
// the right with the left one's members highlighted. Clicking an object on the right puts it in or takes it out.

#include <QComboBox>
#include <QDialog>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonObject>
#include <QLabel>
#include <QListWidget>
#include <QPointer>
#include <QSet>
#include <QSignalBlocker>
#include <QVBoxLayout>

#include "host.h"
#include "widgets.h"

namespace az {

namespace {

struct Kind {
    const char* id;
    const char* label;
    const char* left;
    const char* right;
};

const Kind kKinds[] = {
    {"displayLayers", "Display Layers", "Layers", "Objects"},
    {"sets", "Sets", "Sets", "Objects"},
    {"deformerSets", "Deformer Sets", "Deformer Sets", "Objects"},
    {"shadingGroups", "Shading Groups", "Shading Groups", "Objects"},
    {"characters", "Character Sets", "Character Sets", "Objects"},
    {"lightCentric", "Light Linking (Light Centric)", "Light Sources", "Illuminated Objects"},
    {"objectCentric", "Light Linking (Object Centric)", "Illuminated Objects", "Light Sources"},
};

QStringList strings(const QJsonValue& v) {
    QStringList out;
    if (v.isString()) out << v.toString();
    for (const QJsonValue& x : v.toArray()) out << x.toString();
    return out;
}

QStringList ls(const QJsonObject& flags, const QStringList& args = {}) {
    QJsonObject call = flags;
    if (!args.isEmpty()) call["args"] = QJsonArray::fromStringList(args);
    return strings(ask("ls", call));
}

// Long names of the transforms above these shapes.
QStringList parents_of(const QStringList& shapes) {
    QStringList out;
    for (const QString& s : shapes) {
        for (const QString& p : strings(ask("listRelatives", QJsonObject{{"args", QJsonArray{s}}, {"parent", true}, {"fullPath", true}})))
            if (!out.contains(p)) out << p;
    }
    return out;
}

QString leaf(const QString& path) {
    return path.section('|', -1);
}

bool is_deformer_set(const QString& set) {
    return !strings(ask("listConnections", QJsonObject{{"args", QJsonArray{set + ".usedBy"}}})).isEmpty();
}

class RelationshipEditor : public QDialog {
public:
    explicit RelationshipEditor(QWidget* parent) : QDialog(parent, editor_window_flags()) {
        setWindowTitle("Relationship Editor");
        resize(620, 440);
        auto* root = new QVBoxLayout(this);
        kind_ = new QComboBox(this);
        for (const Kind& k : kKinds) kind_->addItem(k.label, k.id);
        root->addWidget(kind_);
        auto* panes = new QHBoxLayout();
        auto* lcol = new QVBoxLayout();
        auto* rcol = new QVBoxLayout();
        left_title_ = new QLabel(this);
        right_title_ = new QLabel(this);
        left_ = new QListWidget(this);
        right_ = new QListWidget(this);
        left_->setSelectionMode(QAbstractItemView::SingleSelection);
        right_->setSelectionMode(QAbstractItemView::MultiSelection);
        lcol->addWidget(left_title_);
        lcol->addWidget(left_, 1);
        rcol->addWidget(right_title_);
        rcol->addWidget(right_, 1);
        panes->addLayout(lcol, 1);
        panes->addLayout(rcol, 1);
        root->addLayout(panes, 1);
        connect(kind_, &QComboBox::currentIndexChanged, this, [this] { refresh(); });
        connect(left_, &QListWidget::currentRowChanged, this, [this] { show_members(); });
        connect(right_, &QListWidget::itemClicked, this, [this](QListWidgetItem* it) { toggle(it); });
        connect(Host::get(), &Host::sceneChanged, this, [this] {
            if (!busy_) refresh();
        });
    }

    void set_kind(const QString& id) {
        int i = kind_->findData(id);
        if (i >= 0 && i != kind_->currentIndex()) kind_->setCurrentIndex(i);
        else refresh();
    }

private:
    QString kind() const { return kind_->currentData().toString(); }

    QStringList lights() { return parents_of(ls({{"lights", true}})); }

    // Every transform but the cameras' and, for light linking, the lights'.
    QStringList objects() {
        QStringList skip = parents_of(ls({{"cameras", true}}));
        if (kind() == "lightCentric" || kind() == "objectCentric") skip += lights();
        QStringList out;
        for (const QString& t : ls({{"long", true}, {"type", "transform"}}))
            if (!skip.contains(t)) out << t;
        return out;
    }

    QStringList containers() {
        const QString k = kind();
        if (k == "displayLayers") {
            QStringList out = ls({{"type", "displayLayer"}});
            out.removeAll("defaultLayer");
            return out;
        }
        if (k == "shadingGroups") return ls({{"type", "shadingEngine"}});
        if (k == "characters") return ls({{"type", "character"}});
        if (k == "lightCentric") return lights();
        if (k == "objectCentric") return objects();
        QStringList engines = ls({{"type", "shadingEngine"}});
        QStringList out;
        for (const QString& s : ls({{"type", "objectSet"}})) {
            if (engines.contains(s) || s == "defaultLightSet" || s == "defaultObjectSet") continue;
            if (is_deformer_set(s) == (k == "deformerSets")) out << s;
        }
        return out;
    }

    QStringList raw_members(const QString& c) {
        const QString k = kind();
        if (k == "displayLayers") return strings(ask("editDisplayLayerMembers", QJsonObject{{"args", QJsonArray{c}}, {"query", true}, {"fullNames", true}}));
        if (k == "characters") return strings(ask("character", QJsonObject{{"args", QJsonArray{c}}, {"query", true}, {"nodesOnly", true}}));
        if (k == "lightCentric") return strings(ask("lightlink", QJsonObject{{"query", true}, {"light", c}}));
        if (k == "objectCentric") return strings(ask("lightlink", QJsonObject{{"query", true}, {"object", c}}));
        return strings(ask("sets", QJsonObject{{"args", QJsonArray{c}}, {"query", true}}));
    }

    // The members as long transform names: a shape or a component stands for its transform.
    QSet<QString> members(const QString& c) {
        QSet<QString> out;
        for (QString m : raw_members(c)) {
            m = m.section('.', 0, 0);
            for (const QString& l : ls({{"long", true}}, {m})) {
                if (right_names_.contains(l)) out.insert(l);
                else
                    for (const QString& p : parents_of({l})) out.insert(p);
            }
        }
        return out;
    }

    void refresh() {
        const Kind* k = &kKinds[qMax(0, kind_->currentIndex())];
        left_title_->setText(k->left);
        right_title_->setText(k->right);
        QString was = left_->currentItem() ? left_->currentItem()->data(Qt::UserRole).toString() : QString();
        QSignalBlocker b(left_);
        left_->clear();
        for (const QString& c : containers()) {
            auto* it = new QListWidgetItem(leaf(c), left_);
            it->setData(Qt::UserRole, c);
            if (c == was) left_->setCurrentItem(it);
        }
        if (!left_->currentItem() && left_->count()) left_->setCurrentRow(0);
        right_->clear();
        right_names_ = kind() == "objectCentric" ? lights() : objects();
        for (const QString& o : right_names_) {
            auto* it = new QListWidgetItem(QString(2 * qMax(0, o.count('|') - 1), ' ') + leaf(o), right_);
            it->setData(Qt::UserRole, o);
        }
        show_members();
    }

    void show_members() {
        QSignalBlocker b(right_);
        right_->clearSelection();
        if (!left_->currentItem()) return;
        QSet<QString> in = members(left_->currentItem()->data(Qt::UserRole).toString());
        for (int i = 0; i < right_->count(); ++i)
            right_->item(i)->setSelected(in.contains(right_->item(i)->data(Qt::UserRole).toString()));
    }

    void toggle(QListWidgetItem* it) {
        if (!left_->currentItem()) return;
        const QString c = left_->currentItem()->data(Qt::UserRole).toString();
        const QString o = it->data(Qt::UserRole).toString();
        const bool add = it->isSelected();
        const QString k = kind();
        busy_ = true;
        if (k == "displayLayers") {
            az::exec("editDisplayLayerMembers", QJsonObject{{"args", QJsonArray{add ? c : QString("defaultLayer"), o}}});
        } else if (k == "characters") {
            az::exec("character", QJsonObject{{"args", QJsonArray{o}}, {add ? "addElement" : "remove", c}});
        } else if (k == "lightCentric" || k == "objectCentric") {
            QString light = k == "lightCentric" ? c : o, object = k == "lightCentric" ? o : c;
            az::exec("lightlink", QJsonObject{{add ? "make" : "b", true}, {"light", light}, {"object", object}});
        } else if (k == "shadingGroups") {
            if (add) az::exec("sets", QJsonObject{{"args", QJsonArray{o}}, {"edit", true}, {"forceElement", c}});
            else az::exec("sets", QJsonObject{{"args", QJsonArray{o}}, {"remove", c}});
        } else {
            az::exec("sets", QJsonObject{{"args", QJsonArray{o}}, {add ? "addElement" : "remove", c}});
        }
        busy_ = false;
        show_members();
    }

    QComboBox* kind_;
    QLabel* left_title_;
    QLabel* right_title_;
    QListWidget* left_;
    QListWidget* right_;
    QStringList right_names_;
    bool busy_ = false;
};

} // namespace

void show_relationship_editor(QWidget* parent, const QString& kind) {
    static QPointer<RelationshipEditor> open;
    if (!open) {
        open = new RelationshipEditor(parent);
        open->setAttribute(Qt::WA_DeleteOnClose);
    }
    open->set_kind(kind);
    open->show();
    open->raise();
    open->activateWindow();
}

} // namespace az

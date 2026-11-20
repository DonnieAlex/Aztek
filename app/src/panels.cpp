#include "panels.h"
#include "colorchooser.h"
#include <QStyle>
#include <QMouseEvent>
#include "host.h"
#include "icons.h"
#include "viewport.h"

#include <QApplication>
#include <QCheckBox>
#include <QComboBox>
#include <QDropEvent>
#include <QColorDialog>
#include <QFormLayout>
#include <QHash>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QKeyEvent>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMenuBar>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QScrollArea>
#include <QScrollBar>
#include <QSplitter>
#include <QTabWidget>
#include <QDockWidget>
#include <QActionGroup>
#include <QSettings>
#include <functional>
#include <tuple>
#include <QInputDialog>
#include <QGuiApplication>
#include <QPointer>
#include <QTimer>
#include <QToolBar>
#include <QToolButton>
#include <QTreeWidget>
#include <QVBoxLayout>

namespace az {

namespace {

QJsonObject args(const QJsonArray& a) {
    QJsonObject o;
    o["args"] = a;
    return o;
}

// The Channel Box's settings (Ctrl+right-click > Settings and Channel Names), kept between sessions.
QVariant cb_setting(const QString& key, const QVariant& fallback) {
    return QSettings().value("channelBox/" + key, fallback);
}

QString fmt(double v) {
    // Three decimals as the reference shows by default; Settings > Change Precision sets another.
    const int places = cb_setting("precision", 3).toInt();
    QString s = QString::number(v, 'f', places);
    if (s.startsWith('-') && s.toDouble() == 0.0) s = s.mid(1);
    return s;
}

QString value_text(const QJsonObject& ch) {
    QJsonValue v = ch["value"];
    QString t = ch["type"].toString();
    if (t == "bool") return v.toBool() ? "on" : "off";
    if (t == "enum") {
        QJsonArray names = ch["enum"].toArray();
        int i = v.toInt();
        if (i >= 0 && i < names.size()) return names[i].toString();
        return QString::number(i);
    }
    if (t == "int") return QString::number(v.toInteger());
    if (t == "float") return fmt(v.toDouble());
    if (t == "string") return v.toString();
    return QString();
}

// Text typed into a field to the setAttr value for its type.
QJsonValue typed(const QString& t, const QString& text, const QJsonObject& ch) {
    QString s = text.trimmed();
    if (t == "bool") return QJsonValue(s == "on" || s == "1" || s == "true" || s == "yes");
    if (t == "int") return QJsonValue(qint64(s.toDouble()));
    if (t == "enum") {
        QJsonArray names = ch["enum"].toArray();
        for (int i = 0; i < names.size(); ++i)
            if (names[i].toString().compare(s, Qt::CaseInsensitive) == 0) return QJsonValue(i);
        return QJsonValue(qint64(s.toDouble()));
    }
    if (t == "float") return QJsonValue(s.toDouble());
    return QJsonValue(s);
}

void set_attr(const QString& plug, const QJsonValue& v, const QString& type) {
    QJsonObject call = args(QJsonArray{plug, v});
    if (type == "string") call["type"] = "string";
    exec("setAttr", call);
}

} // namespace

// ---------------------------------------------------------------------------------------------------------------------

Panel::Panel(QWidget* parent) : QWidget(parent) {
    connect(Host::get(), &Host::sceneChanged, this, &Panel::schedule);
}

void Panel::schedule() {
    if (pending_) return;
    pending_ = true;
    QTimer::singleShot(0, this, [this] {
        pending_ = false;
        // Not seen (closed, or a tab behind another): brought up to date when it shows, as showEvent schedules.
        if (!isVisible()) return;
        // Just shown, its window not laid out yet: nothing of it is on screen for a moment. It looks again shortly, or it
        // stayed out of date until the next change in the scene (#185). A tab behind another stops after a while and
        // waits for its dock to say it shows.
        if (visibleRegion().isEmpty()) {
            if (retries_ < 20) {
                ++retries_;
                QTimer::singleShot(50, this, [this] { schedule(); });
            }
            return;
        }
        retries_ = 0;
        if (!live_during_drag() && QApplication::mouseButtons() != Qt::NoButton && !underMouse()) {
            if (!waiting_for_release_) {
                waiting_for_release_ = true;
                auto* t = new QTimer(this);
                t->setInterval(80);
                connect(t, &QTimer::timeout, this, [this, t] {
                    if (QApplication::mouseButtons() != Qt::NoButton) return;
                    t->deleteLater();
                    waiting_for_release_ = false;
                    schedule();
                });
                t->start();
            }
            return;
        }
        // Don't pull a field out from under the user's typing.
        QWidget* f = QApplication::focusWidget();
        if (f && isAncestorOf(f) && f->property("editing").toBool()) return;
        QElapsedTimer t;
        t.start();
        refresh();
        profile_line(QString("panel %1").arg(metaObject()->className()), t);
    });
}

void Panel::showEvent(QShowEvent*) {
    retries_ = 0;
    schedule();
    // A dock tabbed behind another counts as shown with nothing of it on screen, so it waits; bringing its tab to the
    // front shows nothing new, only the dock says so. Tool Settings behind the Outliner stayed empty (#105).
    for (QWidget* up = parentWidget(); up; up = up->parentWidget())
        if (auto* dock = qobject_cast<QDockWidget*>(up)) {
            if (!dock->property("panelWatch").toBool()) {
                dock->setProperty("panelWatch", true);
                connect(dock, &QDockWidget::visibilityChanged, this, [this](bool on) {
                    if (on) QTimer::singleShot(0, this, [this] { schedule(); });
                });
            }
            break;
        }
}

// ---------------------------------------------------------------------------------------------------------------------

// Channel names picked in the channel box (the reference's highlighted channels): their attribute names, kept across refreshes.
static QStringList g_channel_sel;
static QList<QLabel*> g_channel_labels;
// The value fields, which light up with their names when picked (#161).
static QList<QLineEdit*> g_channel_fields;
// Where a Shift+click or a drag over the names starts its range.
static QString g_channel_anchor;
// attr to whole plug for the picked channels under INPUTS.
static QHash<QString, QString> g_input_plugs;

QStringList selected_channels() {
    return g_channel_sel;
}

QStringList selected_input_plugs() {
    QStringList out;
    for (const QString& a : g_channel_sel)
        if (g_input_plugs.contains(a)) out << g_input_plugs[a];
    return out;
}

// Picked translate channels put the Move manipulator in the parent's axes, as the reference does: those channels are
// values in the parent's space (#136).
void tell_channel_space() {
    bool translate = false;
    for (const QString& a : g_channel_sel) translate = translate || a.startsWith("translate") || a == "tx" || a == "ty" || a == "tz";
    // Only the views change: the manipulator's axes.
    if (query("uiChannelSpace", translate ? "parent" : "").toBool())
        for (QWidget* w : QApplication::allWidgets())
            if (auto* v = qobject_cast<ViewportWidget*>(w)) v->update();
}

void clear_channel_selection() {
    g_channel_sel.clear();
    g_input_plugs.clear();
    g_channel_anchor.clear();
    tell_channel_space();
}

namespace {

void restyle_channel_labels() {
    tell_channel_space();
    for (QLabel* l : g_channel_labels) {
        l->setProperty("picked", g_channel_sel.contains(l->property("attr").toString()));
        l->style()->unpolish(l);
        l->style()->polish(l);
    }
    for (QLineEdit* f : g_channel_fields) {
        const bool on = g_channel_sel.contains(f->property("attr").toString());
        if (f->property("picked").toBool() == on) continue;
        f->setProperty("picked", on);
        f->style()->unpolish(f);
        f->style()->polish(f);
    }
}

// The channel names from the anchor's row to this one, in the order the box shows them.
QStringList channel_range(const QString& to) {
    int a = -1, b = -1;
    for (int i = 0; i < g_channel_labels.size(); ++i) {
        QString n = g_channel_labels[i]->property("attr").toString();
        if (a < 0 && n == g_channel_anchor) a = i;
        if (b < 0 && n == to) b = i;
    }
    if (a < 0 || b < 0) return {to};
    QStringList out;
    for (int i = qMin(a, b); i <= qMax(a, b); ++i) {
        QString n = g_channel_labels[i]->property("attr").toString();
        if (!out.contains(n)) out << n;
    }
    return out;
}

// The reference's picking, on a name or a value: click picks one, Ctrl+click toggles, Shift+click picks the run from the
// last click. A click on a value that is already picked keeps the others, so the value typed there goes to all of them.
void pick_channel(const QString& a, Qt::KeyboardModifiers mods, bool on_value) {
    if (mods & Qt::ControlModifier) {
        if (g_channel_sel.contains(a)) g_channel_sel.removeAll(a);
        else g_channel_sel << a;
        g_channel_anchor = a;
    } else if ((mods & Qt::ShiftModifier) && !g_channel_anchor.isEmpty()) {
        g_channel_sel = channel_range(a);
    } else if (!(on_value && g_channel_sel.contains(a))) {
        g_channel_sel = {a};
        g_input_plugs.clear();
        g_channel_anchor = a;
    }
}

// A value field: a click picks its channel the same way, before the field takes the focus. A drag over the values picks
// the run, as a drag over the names does, and what's typed next goes to all of them (#161).
class ChannelValueClicks : public QObject {
public:
    using QObject::QObject;
protected:
    bool eventFilter(QObject* o, QEvent* e) override {
        // A click into a value, or a double-click on it, takes the whole value, so what's typed replaces it, as in the
        // reference; a double-click would otherwise take only the word under it (#200).
        if (e->type() == QEvent::MouseButtonDblClick && static_cast<QMouseEvent*>(e)->button() == Qt::LeftButton) {
            if (auto* f = qobject_cast<QLineEdit*>(o)) {
                f->setFocus(Qt::MouseFocusReason);
                f->selectAll();
                return true;
            }
        }
        // A click that brings the focus into a value takes the whole value once the click is over: the field puts
        // its cursor where the press was.
        if (e->type() == QEvent::FocusIn && static_cast<QFocusEvent*>(e)->reason() == Qt::MouseFocusReason) select_on_release_ = true;
        if (e->type() == QEvent::MouseButtonPress && static_cast<QMouseEvent*>(e)->button() == Qt::LeftButton) {
            pick_channel(o->property("attr").toString(), static_cast<QMouseEvent*>(e)->modifiers(), true);
            restyle_channel_labels();
            dragged_ = false;
            if (!(static_cast<QMouseEvent*>(e)->modifiers() & (Qt::ControlModifier | Qt::ShiftModifier))) g_channel_anchor = o->property("attr").toString();
        } else if (e->type() == QEvent::MouseMove && (static_cast<QMouseEvent*>(e)->buttons() & Qt::LeftButton) && !g_channel_anchor.isEmpty()) {
            QWidget* over = QApplication::widgetAt(static_cast<QMouseEvent*>(e)->globalPosition().toPoint());
            auto* field = qobject_cast<QLineEdit*>(over);
            auto* name = qobject_cast<QLabel*>(over);
            QString attr;
            if (field && g_channel_fields.contains(field)) attr = field->property("attr").toString();
            else if (name && g_channel_labels.contains(name)) attr = name->property("attr").toString();
            if (attr.isEmpty() || (!dragged_ && attr == o->property("attr").toString())) return false;
            dragged_ = true;
            QStringList run = channel_range(attr);
            if (run != g_channel_sel) {
                g_channel_sel = run;
                restyle_channel_labels();
            }
            // The run, not the text under the drag.
            return true;
        } else if (e->type() == QEvent::MouseButtonRelease && (dragged_ || select_on_release_)) {
            dragged_ = false;
            select_on_release_ = false;
            if (auto* f = qobject_cast<QLineEdit*>(o)) {
                QPointer<QLineEdit> keep(f);
                QTimer::singleShot(0, f, [keep] {
                    if (!keep) return;
                    keep->setFocus(Qt::MouseFocusReason);
                    keep->selectAll();
                });
            }
        }
        return false;
    }

private:
    bool dragged_ = false;
    bool select_on_release_ = false;
};

// A channel name: click picks it, Ctrl+click toggles, Shift+click or a drag down the names picks a run.
class ChannelLabel : public QLabel {
public:
    ChannelLabel(const QString& text, QWidget* parent) : QLabel(text, parent) {}
protected:
    void mouseMoveEvent(QMouseEvent* e) override {
        if (!(e->buttons() & Qt::LeftButton) || g_channel_anchor.isEmpty()) return QLabel::mouseMoveEvent(e);
        auto* over = qobject_cast<QLabel*>(QApplication::widgetAt(e->globalPosition().toPoint()));
        if (!over || !g_channel_labels.contains(over)) return;
        QStringList run = channel_range(over->property("attr").toString());
        if (run == g_channel_sel) return;
        g_channel_sel = run;
        restyle_channel_labels();
    }
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() != Qt::LeftButton) return QLabel::mousePressEvent(e);
        QString a = property("attr").toString();
        pick_channel(a, e->modifiers(), false);
        if (property("inputPlug").isValid()) {
            g_input_plugs[a] = property("inputPlug").toString();
            // The channel's handle lights up, and a middle drag takes it.
            QString plug = property("inputPlug").toString();
            // Settings > No or Invisible Manipulators keep the handle out of the view; the middle drag still works.
            if (cb_setting("manipulators", 2).toInt() == 2) {
                query("uiShowManip", plug.section('.', 0, -2));
                query("uiShowManipAttr", plug.section('.', -1));
            }
        }
        else g_input_plugs.remove(a);
        restyle_channel_labels();
    }
};

// A value typed into a field. When its channel is picked, it goes to every picked channel of that node, and for the
// objects' own channels to every selected object, as one undo step. +=, -=, *= and /= work on each plug's own value.
void commit_channels(QLabel* edited, const QString& plug, const QString& text) {
    QString attr = edited->property("attr").toString();
    QString node = edited->property("node").toString();
    QList<QJsonObject> chans;
    if (g_channel_sel.contains(attr)) {
        for (QLabel* l : g_channel_labels)
            if (l->property("node").toString() == node && g_channel_sel.contains(l->property("attr").toString()))
                chans << QJsonObject::fromVariantMap(l->property("channel").toMap());
    } else {
        chans << QJsonObject::fromVariantMap(edited->property("channel").toMap());
    }
    QStringList nodes{node};
    if (edited->property("main").toBool()) {
        nodes.clear();
        for (const QJsonValue& v : state()["selection"].toArray()) {
            QString n = v.toString().section('.', 0, 0);
            if (!nodes.contains(n)) nodes << n;
        }
        if (nodes.isEmpty()) nodes << node;
    }
    QString t = text.trimmed();
    QString op;
    for (QString o : {"+=", "-=", "*=", "/="})
        if (t.startsWith(o)) op = o;
    bool many = chans.size() * nodes.size() > 1;
    if (many) exec_quiet("undoInfo", QJsonObject{{"openChunk", true}});
    for (const QJsonObject& c : chans) {
        QString type = c["type"].toString();
        if (c["state"].toString() == "locked" && c["plug"].toString() != plug) continue;
        for (const QString& n : nodes) {
            QString p = n + "." + c["attr"].toString();
            QString value = t;
            if (!op.isEmpty() && (type == "float" || type == "int")) {
                double current = p == c["plug"].toString() ? c["value"].toDouble() : ask("getAttr", args(QJsonArray{p})).toDouble();
                double x = t.mid(2).toDouble();
                double r = op == "+=" ? current + x : op == "-=" ? current - x : op == "*=" ? current * x : (x != 0 ? current / x : current);
                value = QString::number(r, 'g', 12);
            }
            set_attr(p, typed(type, value, c), type);
        }
    }
    if (many) exec_quiet("undoInfo", QJsonObject{{"closeChunk", true}});
}

// A channel row: right-aligned name, value field. Enter commits a setAttr.
QWidget* channel_row(const QJsonObject& ch, QWidget* parent, int label_width) {
    auto* row = new QWidget(parent);
    auto* h = new QHBoxLayout(row);
    h->setContentsMargins(0, 0, 0, 0);
    h->setSpacing(4);
    const int names = cb_setting("names", 0).toInt();
    const QString shown = names == 1 ? ch["attr"].toString() : names == 2 ? ch["short"].toString(ch["attr"].toString()) : ch["nice"].toString();
    auto* label = new ChannelLabel(shown, row);
    label->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    label->setFixedWidth(label_width);
    label->setToolTip(ch["plug"].toString());
    label->setObjectName("channelName");
    label->setProperty("attr", ch["attr"].toString());
    if (parent->property("inInputs").toBool()) label->setProperty("inputPlug", ch["plug"].toString());
    label->setProperty("picked", g_channel_sel.contains(ch["attr"].toString()));
    // The node the row belongs to, and whether it is the selected objects' own channel (the value goes to every
    // selected object) or a shape's or an input's (the value goes to that node).
    QString node = ch["plug"].toString().section('.', 0, -2);
    label->setProperty("node", node);
    label->setProperty("main", parent->property("mainNode").toString() == node);
    label->setProperty("channel", ch.toVariantMap());
    g_channel_labels << label;
    auto* edit = new QLineEdit(value_text(ch), row);
    edit->setProperty("attr", ch["attr"].toString());
    edit->setProperty("picked", g_channel_sel.contains(ch["attr"].toString()));
    g_channel_fields << edit;
    static auto* clicks = new ChannelValueClicks(qApp);
    edit->installEventFilter(clicks);
    // The reference's channel colours: keyed, key on this frame, changed since keyed, driven, locked.
    QString st = ch["state"].toString();
    edit->setObjectName(st == "keyed" ? "channelKeyed" : st == "keyOnFrame" ? "channelKeyOnFrame" : st == "changed" ? "channelChanged"
                        : st == "layered" ? "channelLayered" : st == "muted" ? "channelMuted" : st == "locked" ? "channelLocked" : (st == "connected" || ch["connected"].toBool()) ? "channelConnected" : "channelValue");
    edit->setFixedHeight(18);
    // Right-click is the channel box's menu, not the text field's.
    edit->setContextMenuPolicy(Qt::NoContextMenu);
    QString plug = ch["plug"].toString();
    QObject::connect(edit, &QLineEdit::textEdited, edit, [edit] { edit->setProperty("editing", true); });
    QObject::connect(edit, &QLineEdit::editingFinished, edit, [edit, label, plug] {
        if (!edit->property("editing").toBool()) return;
        edit->setProperty("editing", false);
        commit_channels(label, plug, edit->text());
    });
    h->addWidget(label);
    h->addWidget(edit, 1);
    return row;
}

QWidget* section(const QString& text, QWidget* parent) {
    auto* l = new QLabel(text, parent);
    l->setObjectName("channelSection");
    return l;
}

} // namespace

namespace {

// A right-click menu's name at its top, as the reference shows it.
void menu_title(QMenu* m, const QString& text) {
    QAction* a = m->addAction(text);
    QFont f = a->font();
    f.setBold(true);
    a->setFont(f);
    a->setEnabled(false);
    m->addSeparator();
}

// The reference's Channels menu, also the channel box's right-click menu (#137): keys, mutes, the key clipboard, values,
// freezing, connections, locks and animation layers, on the picked channels of the selected objects.
void fill_channels_menu(QMenu* m, bool titled) {
    QJsonArray picked;
    for (const QString& a : g_channel_sel) picked.append(a);
    bool any = !picked.isEmpty();
    auto objects = [] {
        QStringList out;
        for (const QJsonValue& o : state()["selection"].toArray()) {
            const QString n = o.toString().section('.', 0, 0);
            if (!out.contains(n)) out << n;
        }
        return out;
    };
    auto plugs = [objects] {
        QJsonArray out;
        for (const QString& n : objects())
            for (const QString& a : g_channel_sel) out.append(n + "." + a);
        return out;
    };
    auto each = [plugs](std::function<void(const QJsonValue&)> f) {
        const QJsonArray all = plugs();
        if (all.size() > 1) exec_quiet("undoInfo", QJsonObject{{"openChunk", true}});
        for (const QJsonValue& pl : all) f(pl);
        if (all.size() > 1) exec_quiet("undoInfo", QJsonObject{{"closeChunk", true}});
    };
    if (titled) menu_title(m, "Channels");
    m->addAction("Key Selected", [picked] { exec("setKeyframe", QJsonObject{{"attribute", picked}}); })->setEnabled(any);
    m->addAction("Key All Keyable", [] { exec("setKeyframe"); });
    m->addAction("Breakdown Selected", [picked] { exec("setKeyframe", QJsonObject{{"attribute", picked}, {"breakdown", true}}); })->setEnabled(any);
    m->addAction("Breakdown All", [] { exec("setKeyframe", QJsonObject{{"breakdown", true}}); });
    m->addAction("Mute Selected", [each] { each([](const QJsonValue& pl) { exec("mute", QJsonObject{{"args", QJsonArray{pl}}}); }); })->setEnabled(any);
    m->addAction("Mute All", [] { exec("mute"); });
    m->addAction("Unmute Selected", [each] {
        each([](const QJsonValue& pl) { exec("mute", QJsonObject{{"args", QJsonArray{pl}}, {"disable", true}, {"force", true}}); });
    })->setEnabled(any);
    m->addAction("Unmute All", [] { exec("mute", QJsonObject{{"disable", true}, {"force", true}}); });
    m->addSeparator();
    for (auto [label, key, fallback] : {std::tuple{"Sync Selection in Graph Editor", "syncGraphEditor", true}, std::tuple{"Sync Timeline Display", "syncTimeline", false}}) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(cb_setting(key, fallback).toBool());
        QObject::connect(a, &QAction::toggled, [key = QString(key)](bool on) { QSettings().setValue("channelBox/" + key, on); });
    }
    m->addSeparator();
    // The key clipboard, on the picked channels.
    m->addAction("Cut Selected", [picked] { exec("cutKey", QJsonObject{{"attribute", picked}}); })->setEnabled(any);
    m->addAction("Copy Selected", [picked] { exec("copyKey", QJsonObject{{"attribute", picked}}); })->setEnabled(any);
    m->addAction("Paste Selected", [picked] { exec("pasteKey", QJsonObject{{"attribute", picked}}); })->setEnabled(any);
    m->addAction("Delete Selected", [picked] { exec("cutKey", QJsonObject{{"attribute", picked}, {"clear", true}}); })->setEnabled(any);
    m->addSeparator();
    // Values: copied from the lead object's picked channels, pasted onto the same channels of the selected objects.
    static QList<QPair<QString, QJsonValue>> copied;
    m->addAction("Copy Selected Attribute Values", [objects] {
        copied.clear();
        const QStringList objs = objects();
        if (objs.isEmpty()) return;
        for (const QString& a : g_channel_sel) copied.append({a, ask("getAttr", QJsonObject{{"args", QJsonArray{objs.last() + "." + a}}})});
    })->setEnabled(any);
    m->addAction("Paste Selected Attribute Values", [objects] {
        exec_quiet("undoInfo", QJsonObject{{"openChunk", true}});
        for (const QString& n : objects())
            for (const auto& [a, v] : copied)
                if (!v.isUndefined() && !v.isNull()) exec("setAttr", QJsonObject{{"args", QJsonArray{n + "." + a, v}}});
        exec_quiet("undoInfo", QJsonObject{{"closeChunk", true}});
    })->setEnabled(!copied.isEmpty());
    m->addSeparator();
    // The lead object's values of the picked channels, given to the other selected objects.
    m->addAction("Duplicate Values", [objects] {
        const QStringList objs = objects();
        if (objs.size() < 2) return;
        exec_quiet("undoInfo", QJsonObject{{"openChunk", true}});
        for (const QString& a : g_channel_sel) {
            const QJsonValue v = ask("getAttr", QJsonObject{{"args", QJsonArray{objs.last() + "." + a}}});
            for (int i = 0; i + 1 < objs.size(); ++i) exec("setAttr", QJsonObject{{"args", QJsonArray{objs[i] + "." + a, v}}});
        }
        exec_quiet("undoInfo", QJsonObject{{"closeChunk", true}});
    })->setEnabled(any);
    QMenu* freeze = m->addMenu("Freeze");
    for (auto [label, flags] : {std::pair{"Translate", QJsonObject{{"translate", true}}}, std::pair{"Rotate", QJsonObject{{"rotate", true}}},
                                std::pair{"Scale", QJsonObject{{"scale", true}}}, std::pair{"All", QJsonObject{{"translate", true}, {"rotate", true}, {"scale", true}}}}) {
        QJsonObject call = flags;
        call["apply"] = true;
        freeze->addAction(label, [call] { exec("makeIdentity", call); });
    }
    m->addSeparator();
    m->addAction("Break Connections", [each] { each([](const QJsonValue& pl) { exec("disconnectAttr", QJsonObject{{"args", QJsonArray{pl}}}); }); })->setEnabled(any);
    // The nodes driving the picked channels.
    m->addAction("Select Connection", [plugs] {
        QJsonArray sources;
        for (const QJsonValue& pl : plugs()) {
            const QJsonArray s = ask("listConnections", QJsonObject{{"args", QJsonArray{pl}}, {"source", true}, {"destination", false}, {"skipConversionNodes", true}}).toArray();
            for (const QJsonValue& n : s)
                if (!sources.contains(n)) sources.append(n);
        }
        if (!sources.isEmpty()) exec("select", QJsonObject{{"args", sources}, {"replace", true}});
    })->setEnabled(any);
    m->addSeparator();
    m->addAction("Lock Selected", [each] { each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"lock", true}}); }); })->setEnabled(any);
    m->addAction("Unlock Selected", [each] { each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"lock", false}}); }); })->setEnabled(any);
    m->addAction("Hide Selected", [each] { each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"keyable", false}, {"channelBox", false}}); }); })->setEnabled(any);
    m->addAction("Lock and Hide Selected", [each] {
        each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"lock", true}, {"keyable", false}, {"channelBox", false}}); });
    })->setEnabled(any);
    // Nonkeyable but still in the box, shown grey, as the reference's.
    m->addAction("Make Selected Nonkeyable", [each] { each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"keyable", false}, {"channelBox", true}}); }); })->setEnabled(any);
    m->addAction("Make Selected Keyable", [each] { each([](const QJsonValue& pl) { exec("setAttr", QJsonObject{{"args", QJsonArray{pl}}, {"keyable", true}}); }); })->setEnabled(any);
    m->addSeparator();
    // The animation layers picked in the Anim layer editor.
    auto selected_layers = [] {
        QStringList out;
        for (const QJsonValue& l : ask("ls", QJsonObject{{"type", "animLayer"}}).toArray())
            if (ask("animLayer", QJsonObject{{"args", QJsonArray{l}}, {"query", true}, {"selected", true}}).toBool()) out << l.toString();
        return out;
    };
    const bool layers = !selected_layers().isEmpty();
    m->addAction("Add to Selected Layers", [plugs, selected_layers] {
        for (const QString& l : selected_layers()) exec("animLayer", QJsonObject{{"args", QJsonArray{l}}, {"edit", true}, {"attribute", plugs()}});
    })->setEnabled(any && layers);
    m->addAction("Remove From Selected Layers", [plugs, selected_layers] {
        for (const QString& l : selected_layers()) exec("animLayer", QJsonObject{{"args", QJsonArray{l}}, {"edit", true}, {"removeAttribute", plugs()}});
    })->setEnabled(any && layers);
}

// Ctrl+right-click: the reference's Edit menu for the node the click was on (the object, a shape or an input): editors,
// attributes, the node itself and the Channel Box's settings (#137).
void fill_edit_menu(QMenu* m, const QString& node, std::function<void()> refresh) {
    const bool any = !g_channel_sel.isEmpty();
    menu_title(m, "Edit");
    m->addAction("Expressions...", [] { mel_action("ExpressionEditor"); })->setEnabled(any);
    m->addAction("Set Driven Key...", [] { mel_action("SetDrivenKeyOptions"); })->setEnabled(any);
    m->addAction("Graph Editor", [] { mel_action("GraphEditor"); });
    m->addAction("Channel Control", [] { mel_action("ChannelControlEditor"); });
    m->addAction("Connection Editor", [] { mel_action("ConnectionEditor"); });
    m->addAction("Attribute Editor", [] { mel_action("AttributeEditor"); });
    // The material on the object, in the Attribute Editor.
    m->addAction("Material Attributes", [node] {
        const QJsonArray shapes = ask("listRelatives", QJsonObject{{"args", QJsonArray{node}}, {"shapes", true}, {"fullPath", true}}).toArray();
        const QJsonArray from = shapes.isEmpty() ? QJsonArray{node} : shapes;
        for (const QJsonValue& s : from) {
            const QJsonArray sgs = ask("listConnections", QJsonObject{{"args", QJsonArray{s}}, {"type", "shadingEngine"}}).toArray();
            if (sgs.isEmpty()) continue;
            const QJsonArray mats = ask("listConnections", QJsonObject{{"args", QJsonArray{sgs[0].toString() + ".surfaceShader"}}, {"source", true}, {"destination", false}}).toArray();
            if (mats.isEmpty()) continue;
            exec("select", QJsonObject{{"args", QJsonArray{mats[0]}}, {"replace", true}});
            mel_action("AttributeEditor");
            return;
        }
    });
    m->addSeparator();
    m->addAction("Add Attribute", [] { mel_action("AddAttribute"); });
    // Rename the one picked attribute (one added with Add Attribute).
    m->addAction("Edit Attribute", [node, refresh] {
        const QString a = g_channel_sel.first();
        bool ok = false;
        const QString to = QInputDialog::getText(main_window_widget(), "Edit Attribute", "Name:", QLineEdit::Normal, a, &ok).trimmed();
        if (ok && !to.isEmpty() && to != a) {
            exec("renameAttr", QJsonObject{{"args", QJsonArray{node + "." + a, to}}});
            g_channel_sel = {to};
            refresh();
        }
    })->setEnabled(g_channel_sel.size() == 1);
    QAction* dup = m->addAction("Duplicate Attribute");
    dup->setEnabled(false);
    dup->setToolTip("Not in Aztek yet.");
    m->addAction("Delete Attributes", [node] {
        exec_quiet("undoInfo", QJsonObject{{"openChunk", true}});
        for (const QString& a : g_channel_sel) exec("deleteAttr", QJsonObject{{"args", QJsonArray{node + "." + a}}});
        exec_quiet("undoInfo", QJsonObject{{"closeChunk", true}});
        g_channel_sel.clear();
    })->setEnabled(any);
    // Assets (containers) are not in Aztek: these stay off.
    for (const char* label : {"Publish to Asset", "Unpublish from Asset", "Lock Unpublished Attributes", "Unlock Unpublished Attributes"}) {
        QAction* a = m->addAction(label);
        a->setEnabled(false);
        a->setToolTip("Assets are not in Aztek yet.");
    }
    m->addSeparator();
    m->addAction("Select Node", [node] { exec("select", QJsonObject{{"args", QJsonArray{node}}, {"replace", true}}); });
    m->addAction("Delete Node", [node] { exec("delete", QJsonObject{{"args", QJsonArray{node}}}); });
    m->addAction("Delete History", [node] { exec("delete", QJsonObject{{"args", QJsonArray{node}}, {"constructionHistory", true}}); });
    QMenu* settings = m->addMenu("Settings");
    auto radio = [settings, refresh](const QString& key, int fallback, const QStringList& labels) {
        auto* group = new QActionGroup(settings);
        for (int i = 0; i < labels.size(); ++i) {
            QAction* a = settings->addAction(labels[i]);
            a->setCheckable(true);
            a->setChecked(cb_setting(key, fallback).toInt() == i);
            group->addAction(a);
            QObject::connect(a, &QAction::triggered, [key, i, refresh] {
                QSettings().setValue("channelBox/" + key, i);
                refresh();
            });
        }
    };
    radio("speed", 2, {"Slowest", "Slow", "Medium", "Fast"});
    settings->addSeparator();
    for (auto [label, key, fallback] : {std::tuple{"Hyperbolic", "hyperbolic", false}, std::tuple{"Show Namespace", "namespace", true}}) {
        QAction* a = settings->addAction(label);
        a->setCheckable(true);
        a->setChecked(cb_setting(key, fallback).toBool());
        QObject::connect(a, &QAction::toggled, [key = QString(key), refresh](bool on) {
            QSettings().setValue("channelBox/" + key, on);
            refresh();
        });
    }
    settings->addSeparator();
    radio("manipulators", 2, {"No Manipulators", "Invisible Manipulators", "Standard Manipulators"});
    settings->addSeparator();
    settings->addAction("Change Precision...", [refresh] {
        bool ok = false;
        const int p = QInputDialog::getInt(main_window_widget(), "Change Precision", "Decimal places:", cb_setting("precision", 3).toInt(), 0, 15, 1, &ok);
        if (!ok) return;
        QSettings().setValue("channelBox/precision", p);
        refresh();
    });
    settings->addAction("Reset to Default", [refresh] {
        QSettings s;
        for (const char* k : {"speed", "hyperbolic", "namespace", "manipulators", "precision"}) s.remove(QString("channelBox/") + k);
        refresh();
    });
    QMenu* names = m->addMenu("Channel Names");
    auto* group = new QActionGroup(names);
    const QStringList kinds{"Nice", "Long", "Short"};
    for (int i = 0; i < kinds.size(); ++i) {
        QAction* a = names->addAction(kinds[i]);
        a->setCheckable(true);
        a->setChecked(cb_setting("names", 0).toInt() == i);
        group->addAction(a);
        QObject::connect(a, &QAction::triggered, [i, refresh] {
            QSettings().setValue("channelBox/names", i);
            refresh();
        });
    }
}

} // namespace

ChannelBox::ChannelBox(QWidget* parent) : Panel(parent) {
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(0, 0, 0, 0);
    lay->setSpacing(0);
    // The reference's menu bar over the channels: Channels, Edit, Object.
    auto* bar = new QMenuBar(this);
    bar->setNativeMenuBar(false);
    bar->setObjectName("panelMenus");
    QMenu* channels = bar->addMenu("Channels");
    connect(channels, &QMenu::aboutToShow, channels, [channels] {
        channels->clear();
        fill_channels_menu(channels, false);
    });
    QMenu* edit = bar->addMenu("Edit");
    edit->addAction("Expressions...", [] { mel_action("ExpressionEditor"); });
    edit->addAction("Set Driven Key...", [] { mel_action("SetDrivenKeyOptions"); });
    edit->addAction("Connection Editor...", [] { mel_action("ConnectionEditor"); });
    edit->addAction("Graph Editor...", [] { mel_action("GraphEditor"); });
    edit->addSeparator();
    edit->addAction("Channel Control...", [] { mel_action("ChannelControlEditor"); });
    // Object: the selected objects; picking one shows it here.
    QMenu* object = bar->addMenu("Object");
    connect(object, &QMenu::aboutToShow, object, [object] {
        object->clear();
        QJsonArray sel = state()["selection"].toArray();
        QStringList nodes;
        for (const QJsonValue& v : sel) {
            QString n = v.toString().section('.', 0, 0);
            if (!nodes.contains(n)) nodes << n;
        }
        if (nodes.isEmpty()) object->addAction("Nothing selected")->setEnabled(false);
        for (const QString& n : nodes) {
            QAction* a = object->addAction(n.section('|', -1), [nodes, n] {
                // The lead object is the last selected: move this one to the end.
                QJsonArray order;
                for (const QString& o : nodes)
                    if (o != n) order.append(o);
                order.append(n);
                exec("select", QJsonObject{{"args", order}, {"replace", true}});
            });
            a->setCheckable(true);
            a->setChecked(n == nodes.last());
        }
    });
    lay->addWidget(bar);
    scroll_ = new QScrollArea(this);
    scroll_->setWidgetResizable(true);
    scroll_->setFrameShape(QFrame::NoFrame);
    lay->addWidget(scroll_);
    setObjectName("channelBox");
    setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this, &QWidget::customContextMenuRequested, this, [this](const QPoint& p) {
        QMenu m;
        if (QGuiApplication::keyboardModifiers() & Qt::ControlModifier) {
            // The node of the row or heading under the click: an input, a shape or the object.
            QString node;
            for (QWidget* w = childAt(p); w && node.isEmpty() && w != this; w = w->parentWidget()) {
                node = w->property("node").toString();
                if (node.isEmpty()) {
                    for (QObject* c : w->children())
                        if (c->property("node").isValid() && node.isEmpty()) node = c->property("node").toString();
                }
            }
            if (node.isEmpty() && body_) node = body_->property("mainNode").toString();
            if (node.isEmpty()) return;
            fill_edit_menu(&m, node, [this] { refresh(); });
        } else {
            fill_channels_menu(&m, true);
        }
        m.exec(mapToGlobal(p));
    });
}

void ChannelBox::add_channels(QVBoxLayout* lay, const QJsonArray& channels) {
    QFontMetrics fm(font());
    int w = 60;
    for (const QJsonValue& c : channels) w = qMax(w, fm.horizontalAdvance(c.toObject()["nice"].toString()) + 8);
    for (const QJsonValue& c : channels) lay->addWidget(channel_row(c.toObject(), body_, w));
}

void ChannelBox::refresh() {
    QJsonObject cb = query("channelBox").toObject();
    g_channel_labels.clear();
    g_channel_fields.clear();
    int scroll = scroll_->verticalScrollBar()->value();
    body_ = new QWidget();
    auto* lay = new QVBoxLayout(body_);
    lay->setContentsMargins(4, 4, 4, 4);
    lay->setSpacing(1);
    if (cb.isEmpty()) {
        // Nothing selected: the picked channels go too, as in the reference (#136).
        if (!g_channel_sel.isEmpty()) clear_channel_selection();
        lay->addStretch(1);
        scroll_->setWidget(body_);
        return;
    }
    // Object name, editable to rename.
    // Settings > Show Namespace off drops the namespace from the name shown.
    const QString full = cb["name"].toString();
    auto* name = new QLineEdit(cb_setting("namespace", true).toBool() ? full : full.section(':', -1), body_);
    name->setObjectName("channelHeader");
    QString path = cb["path"].toString();
    connect(name, &QLineEdit::textEdited, name, [name] { name->setProperty("editing", true); });
    connect(name, &QLineEdit::editingFinished, name, [name, path] {
        if (!name->property("editing").toBool()) return;
        name->setProperty("editing", false);
        exec("rename", args(QJsonArray{path, name->text().trimmed()}));
    });
    name->setProperty("node", path);
    lay->addWidget(name);
    body_->setProperty("mainNode", path);
    add_channels(lay, cb["channels"].toArray());
    QJsonArray shapes = cb["shapes"].toArray();
    if (!shapes.isEmpty()) {
        lay->addSpacing(6);
        lay->addWidget(section("SHAPES", body_));
        for (const QJsonValue& s : shapes) {
            auto* l = new QLabel(s.toObject()["name"].toString(), body_);
            l->setProperty("node", s.toObject()["path"].toString());
            l->setObjectName("channelNode");
            lay->addWidget(l);
            add_channels(lay, s.toObject()["channels"].toArray());
        }
    }
    QJsonArray inputs = cb["inputs"].toArray();
    if (!inputs.isEmpty()) {
        lay->addSpacing(6);
        lay->addWidget(section("INPUTS", body_));
        for (const QJsonValue& v : inputs) {
            QJsonObject in = v.toObject();
            QString p = in["path"].toString();
            auto* b = new QPushButton(in["name"].toString(), body_);
            b->setProperty("node", p);
            b->setObjectName("channelNodeButton");
            b->setFlat(true);
            connect(b, &QPushButton::clicked, this, [this, p] {
                // The Show Manipulator tool shows the node picked here.
                query("uiShowManip", p);
                if (open_inputs_.contains(p))
                    open_inputs_.remove(p);
                else
                    open_inputs_.insert(p);
                refresh();
            });
            lay->addWidget(b);
            if (open_inputs_.contains(p)) {
                body_->setProperty("inInputs", true);
                add_channels(lay, in["channels"].toArray());
                body_->setProperty("inInputs", false);
            }
        }
    }
    lay->addStretch(1);
    scroll_->setWidget(body_);
    scroll_->verticalScrollBar()->setValue(scroll);
}

// ---------------------------------------------------------------------------------------------------------------------

LayerEditor::LayerEditor(QWidget* parent) : Panel(parent) {
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(2, 2, 2, 2);
    lay->setSpacing(2);
    auto* bar = new QToolBar(this);
    bar->setIconSize(QSize(16, 16));
    bar->addAction(icon("layer"), "Create a new layer", [] { exec("createDisplayLayer", QJsonObject{{"empty", true}}); });
    bar->addAction(icon("layerSelected"), "Create a new layer and assign selected objects", [] { exec("createDisplayLayer"); });
    lay->addWidget(bar);
    list_ = new QTreeWidget(this);
    // V, P, T or R, colour, name.
    list_->setColumnCount(5);
    list_->setHeaderHidden(true);
    list_->setRootIsDecorated(false);
    list_->header()->setStretchLastSection(true);
    for (int i = 0; i < 4; ++i) list_->setColumnWidth(i, 22);
    list_->setDragDropMode(QAbstractItemView::InternalMove);
    list_->setSelectionMode(QAbstractItemView::SingleSelection);
    lay->addWidget(list_, 1);
    // A drag in the list is the new order, top first.
    connect(list_->model(), &QAbstractItemModel::rowsMoved, this, [this] {
        for (int i = 0; i < list_->topLevelItemCount(); ++i)
            exec_quiet("reorderDisplayLayer", QJsonObject{{"args", QJsonArray{list_->topLevelItem(i)->data(0, Qt::UserRole).toString(), list_->topLevelItemCount() - 1 - i}}});
        Host::get()->poke();
    });
    connect(list_, &QTreeWidget::itemClicked, this, [this](QTreeWidgetItem* it, int col) {
        QString layer = it->data(0, Qt::UserRole).toString();
        if (col == 0) {
            bool vis = it->data(0, Qt::UserRole + 1).toBool();
            exec("setAttr", args(QJsonArray{layer + ".visibility", !vis}));
        } else if (col == 1) {
            bool play = it->data(1, Qt::UserRole).toBool();
            exec("setAttr", args(QJsonArray{layer + ".hideOnPlayback", play}));
        } else if (col == 2) {
            // Cycles normal, template, reference as the reference's display type button.
            int t = (it->data(2, Qt::UserRole).toInt() + 1) % 3;
            exec("setAttr", args(QJsonArray{layer + ".displayType", t}));
        } else if (col == 3) {
            ColorChoice c = choose_display_color(this, "Layer Color", it->data(3, Qt::UserRole).toInt(), it->data(3, Qt::UserRole + 1).toBool(), it->data(3, Qt::UserRole + 2).value<QColor>());
            if (c.accepted) {
                exec("setAttr", args(QJsonArray{layer + ".overrideRGBColors", c.use_rgb}));
                if (c.use_rgb) {
                    exec("setAttr", QJsonObject{{"args", QJsonArray{layer + ".overrideColorRGB", c.rgb.redF(), c.rgb.greenF(), c.rgb.blueF()}}, {"type", "double3"}});
                } else {
                    exec("setAttr", args(QJsonArray{layer + ".color", c.index}));
                }
            }
        }
    });
    connect(list_, &QTreeWidget::itemDoubleClicked, this, [](QTreeWidgetItem* it, int col) {
        if (col == 4) exec("select", args(it->data(4, Qt::UserRole).toJsonArray()));
    });
    list_->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(list_, &QWidget::customContextMenuRequested, this, [this](const QPoint& p) {
        QTreeWidgetItem* it = list_->itemAt(p);
        if (!it) return;
        QString layer = it->data(0, Qt::UserRole).toString();
        QJsonArray members = it->data(4, Qt::UserRole).toJsonArray();
        QMenu m;
        m.addAction("Add Selected Objects", [layer] {
            QJsonArray a{layer};
            for (const QJsonValue& s : state()["selection"].toArray()) a.append(s);
            exec("editDisplayLayerMembers", args(a));
        });
        m.addAction("Remove Selected Objects", [] { exec("removeFromDisplayLayer"); });
        m.addAction("Select Objects", [members] { exec("select", args(members)); });
        m.addSeparator();
        m.addAction("Delete Layer", [layer] { exec("deleteDisplayLayer", args(QJsonArray{layer})); });
        m.exec(list_->viewport()->mapToGlobal(p));
    });
}

void LayerEditor::refresh() {
    list_->clear();
    for (const QJsonValue& v : query("layers").toArray()) {
        QJsonObject l = v.toObject();
        auto* it = new QTreeWidgetItem(list_);
        bool vis = l["visible"].toBool();
        bool play = l.contains("playback") ? l["playback"].toBool() : true;
        int type = l["displayType"].toInt();
        it->setText(0, vis ? "V" : "");
        it->setText(1, play ? "P" : "");
        it->setText(2, type == 1 ? "T" : type == 2 ? "R" : "");
        it->setText(4, l["name"].toString());
        it->setData(0, Qt::UserRole, l["name"].toString());
        it->setData(0, Qt::UserRole + 1, vis);
        it->setData(1, Qt::UserRole, play);
        it->setData(2, Qt::UserRole, type);
        it->setData(4, Qt::UserRole, l["members"].toArray());
        for (int c = 0; c < 3; ++c) it->setTextAlignment(c, Qt::AlignCenter);
        if (l["color"].isArray()) {
            QJsonArray c = l["color"].toArray();
            it->setBackground(3, QColor::fromRgbF(c[0].toDouble(), c[1].toDouble(), c[2].toDouble()));
        }
        QJsonArray rgb = l["rgb"].toArray();
        it->setData(3, Qt::UserRole, l["colorIndex"].toInt());
        it->setData(3, Qt::UserRole + 1, l["useRgb"].toBool());
        it->setData(3, Qt::UserRole + 2, QColor::fromRgbF(rgb[0].toDouble(), rgb[1].toDouble(), rgb[2].toDouble()));
        it->setToolTip(3, "Layer color. Click to choose.");
        it->setFlags((it->flags() | Qt::ItemIsDragEnabled) & ~Qt::ItemIsDropEnabled);
        it->setToolTip(4, QString("%1 members. Double-click to select them.").arg(l["members"].toArray().size()));
    }
}

} // namespace az

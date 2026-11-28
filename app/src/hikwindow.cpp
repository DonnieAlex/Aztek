// Humanoid IK, laid out as the reference's Humanoid IK panel (#168): the menu button, Character and Source, a row of tool
// buttons, then the Skeleton, Definition, Controls and Properties tabs. Definition and Controls draw the body, with views
// of the hands, feet, spine, clavicles, neck and roll bones to go into; under Definition the status and the picked
// joint's values. It reads hikInfo and acts through the hik commands. The figures are drawn here, Aztek's own.
#include "editors.h"
#include "panels.h"

#include <QCheckBox>
#include <QComboBox>
#include <QDialog>
#include <QDialogButtonBox>
#include <QDoubleSpinBox>
#include <QFileDialog>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QInputDialog>
#include <QJsonArray>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMessageBox>
#include <QMouseEvent>
#include <QPainter>
#include <QPainterPath>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QRadialGradient>
#include <QScrollArea>
#include <QSlider>
#include <QSpinBox>
#include <QStackedWidget>
#include <QTabWidget>
#include <QToolButton>
#include <QVBoxLayout>

#include <functional>

#include "host.h"

namespace az {

namespace {

// A joint's place in a view, in a box 1 wide and 1.5 tall.
struct Spot {
    QString slot;
    double x, y;
};

// A way into another view: where it sits, which view, and whether it's a roll bone's (a square, as the reference marks them).
struct Door {
    QString view;
    double x, y;
    bool square;
};

const QColor ASSIGNED(120, 224, 70), EMPTY(150, 150, 150), GLOW(40, 150, 255);

QList<Spot> body_spots() {
    QList<Spot> s = {{"Head", .50, .13}, {"Neck", .50, .245}, {"Spine", .50, .44}, {"Hips", .50, .62}, {"Reference", .50, 1.43}};
    for (int side = 0; side < 2; ++side) {
        const QString n = side == 0 ? "Left" : "Right";
        const double d = side == 0 ? 1 : -1;
        // The character faces the viewer: its left is on the right of the figure.
        s += {{n + "Shoulder", .5 + d * .08, .30}, {n + "Arm", .5 + d * .17, .33}, {n + "ForeArm", .5 + d * .25, .53}, {n + "Hand", .5 + d * .32, .71},
              {n + "UpLeg", .5 + d * .08, .68}, {n + "Leg", .5 + d * .10, .98}, {n + "Foot", .5 + d * .11, 1.30}, {n + "ToeBase", .5 + d * .13, 1.40}};
    }
    return s;
}

QList<Door> body_doors(bool definition) {
    QList<Door> d = {{"Spine", .64, .44, false}, {"Neck", .64, .22, false}};
    for (int side = 0; side < 2; ++side) {
        const QString n = side == 0 ? "Left" : "Right";
        const double k = side == 0 ? 1 : -1;
        d += {{n + "Hand", .5 + k * .43, .76, false}, {n + "Foot", .5 + k * .22, 1.40, false}, {n + "Clavicle", .5 + k * .26, .26, false}};
        if (definition) d += {{n + "ArmRoll", .5 + k * .27, .38, true}, {n + "ForeArmRoll", .5 + k * .35, .52, true}, {n + "UpLegRoll", .5 + k * .2, .80, true}, {n + "LegRoll", .5 + k * .2, 1.02, true}};
    }
    return d;
}

// A view's joints: the hands and feet with their digits, the spine and neck from the bottom up, the clavicles, the roll bones.
QList<Spot> view_spots(const QString& view) {
    QList<Spot> s;
    const bool right = view.startsWith("Right");
    const QString side = right ? "Right" : "Left";
    const double d = right ? -1 : 1;
    const auto mx = [d](double x) { return d > 0 ? x : 1 - x; };
    if (view.endsWith("Hand") || view.endsWith("Foot")) {
        const bool hand = view.endsWith("Hand");
        const QString limb = side + (hand ? "Hand" : "Foot");
        s << Spot{limb, mx(.30), 1.25};
        s << Spot{side + (hand ? "FingerBase" : "ToeBase"), mx(.36), 1.05};
        const char* digits[] = {"Thumb", "Index", "Middle", "Ring", "Pinky", "ExtraFinger"};
        for (int k = 0; k < 6; ++k) {
            const double x0 = hand ? (k == 0 ? .62 : .26 + k * .11) : .22 + k * .11;
            const double y0 = hand ? (k == 0 ? .95 : .78) : .70;
            s << Spot{side + (hand ? "InHand" : "InFoot") + digits[k], mx(x0 - .03), y0 + .12};
            for (int b = 1; b <= 4; ++b) {
                const double dx = hand && k == 0 ? .07 : 0.0;
                s << Spot{limb + digits[k] + QString::number(b), mx(x0 + dx * b), y0 - b * (hand ? .13 : .10)};
            }
        }
    } else if (view == "Spine" || view == "Neck") {
        for (int i = 1; i <= 9; ++i) s << Spot{view + QString::number(i), .5, 1.30 - i * .12};
    } else if (view.endsWith("Clavicle")) {
        s << Spot{side + "Shoulder", mx(.42), .70} << Spot{side + "ShoulderExtra", mx(.66), .62};
    } else if (view.endsWith("Roll")) {
        const QString seg = view.mid(side.size()).chopped(4);
        for (int k = 1; k <= 5; ++k) s << Spot{"Leaf" + side + seg + "Roll" + QString::number(k), mx(.40 + k * .05), .30 + k * .17};
    }
    return s;
}

// The joint sticks of the control rig figure.
const char* const BONES[][2] = {
    {"Hips", "Spine"}, {"Spine", "Neck"}, {"Neck", "Head"}, {"Spine", "LeftShoulder"}, {"LeftShoulder", "LeftArm"}, {"LeftArm", "LeftForeArm"}, {"LeftForeArm", "LeftHand"},
    {"Spine", "RightShoulder"}, {"RightShoulder", "RightArm"}, {"RightArm", "RightForeArm"}, {"RightForeArm", "RightHand"}, {"Hips", "LeftUpLeg"}, {"LeftUpLeg", "LeftLeg"},
    {"LeftLeg", "LeftFoot"}, {"LeftFoot", "LeftToeBase"}, {"Hips", "RightUpLeg"}, {"RightUpLeg", "RightLeg"}, {"RightLeg", "RightFoot"}, {"RightFoot", "RightToeBase"},
};

// The figure: a body, or a part of it, the joints to pick, the ways into the closer views, and a way back.
class Figure : public QWidget {
public:
    enum Mode { Definition, Controls };
    std::function<void(const QString&)> picked_slot;
    std::function<void(const QString&, const QPoint&)> slot_menu;
    std::function<void()> view_changed;
    QHash<QString, bool> filled;
    QString picked, status = "green";
    bool rig = false;

    Figure(Mode m, QWidget* parent = nullptr) : QWidget(parent), mode_(m) {
        setMinimumSize(250, 330);
        setMouseTracking(true);
        setContextMenuPolicy(Qt::CustomContextMenu);
        connect(this, &QWidget::customContextMenuRequested, this, [this](const QPoint& at) {
            if (const Spot* s = spot_at(at); s && slot_menu) slot_menu(s->slot, mapToGlobal(at));
        });
    }
    QString view() const { return view_; }
    void set_view(const QString& v) {
        view_ = v;
        update();
        if (view_changed) view_changed();
    }
    QSize sizeHint() const override { return {270, 380}; }

protected:
    QRectF box() const {
        const double h = height() - 8, w = width() - 8;
        const double s = std::min(w, h / 1.5);
        return QRectF((width() - s) / 2, (height() - s * 1.5) / 2, s, s * 1.5);
    }
    QPointF at(double x, double y) const {
        const QRectF b = box();
        return QPointF(b.left() + x * b.width(), b.top() + y * b.width());
    }
    double unit() const { return box().width(); }
    QList<Spot> spots() const { return view_.isEmpty() ? body_spots() : view_spots(view_); }
    const Spot* spot_at(const QPoint& p) const {
        static QList<Spot> cache;
        cache = spots();
        const Spot* best = nullptr;
        double dist = unit() * .05;
        for (const Spot& s : cache) {
            const double d = QLineF(at(s.x, s.y), p).length();
            if (d < dist) {
                dist = d;
                best = &s;
            }
        }
        return best;
    }
    // The body, drawn as a glowing outline: Aztek's own simple figure.
    void draw_body(QPainter& p) {
        QPainterPath b;
        const double u = unit();
        auto limb = [&](double x0, double y0, double x1, double y1, double w) {
            QPainterPath l;
            QLineF line(at(x0, y0), at(x1, y1));
            QLineF n = line.normalVector();
            n.setLength(w * u / 2);
            const QPointF o = n.p2() - n.p1();
            l.moveTo(line.p1() + o);
            l.lineTo(line.p2() + o);
            l.lineTo(line.p2() - o);
            l.lineTo(line.p1() - o);
            l.closeSubpath();
            l.addEllipse(line.p1(), w * u / 2, w * u / 2);
            l.addEllipse(line.p2(), w * u / 2, w * u / 2);
            b = b.united(l);
        };
        if (view_.isEmpty()) {
            b.addEllipse(at(.5, .13), .075 * u, .095 * u);
            limb(.5, .22, .5, .27, .08);
            limb(.5, .33, .5, .62, .26);
            for (double d : {1.0, -1.0}) {
                limb(.5 + d * .1, .31, .5 + d * .17, .33, .09);
                limb(.5 + d * .17, .33, .5 + d * .25, .53, .085);
                limb(.5 + d * .25, .53, .5 + d * .31, .70, .07);
                b.addEllipse(at(.5 + d * .33, .75), .04 * u, .06 * u);
                limb(.5 + d * .08, .66, .5 + d * .10, .98, .12);
                limb(.5 + d * .10, .98, .5 + d * .11, 1.30, .09);
                limb(.5 + d * .11, 1.32, .5 + d * .14, 1.40, .06);
            }
        } else if (view_.endsWith("Hand") || view_.endsWith("Foot")) {
            const bool hand = view_.endsWith("Hand");
            const double d = view_.startsWith("Right") ? -1 : 1;
            const auto mx = [d](double x) { return d > 0 ? x : 1 - x; };
            limb(mx(.40), 1.0, mx(.36), 1.45, .36);
            for (int k = 0; k < 5; ++k) {
                const double x0 = hand ? (k == 0 ? .62 : .26 + k * .11) : .22 + k * .11;
                const double y0 = hand ? (k == 0 ? .95 : .78) : .70;
                const double dx = hand && k == 0 ? .07 : 0.0;
                limb(mx(x0), y0 + .15, mx(x0 + dx * 3), y0 - 3 * (hand ? .13 : .10), .08);
            }
        } else if (view_ == "Spine") {
            // The back, from the shoulders down to the waist.
            QPolygonF back({at(.02, .05), at(.98, .05), at(.86, .55), at(.78, 1.1), at(.84, 1.5), at(.16, 1.5), at(.22, 1.1), at(.14, .55)});
            b.addPolygon(back);
            b.closeSubpath();
        } else if (view_ == "Neck") {
            // The neck between the head and the shoulders.
            b.addEllipse(at(.5, .0), .38 * u, .32 * u);
            QPolygonF neck({at(.34, .25), at(.66, .25), at(.68, 1.15), at(1.0, 1.35), at(1.0, 1.5), at(.0, 1.5), at(.0, 1.35), at(.32, 1.15)});
            b.addPolygon(neck);
            b.closeSubpath();
        } else {
            const double d = view_.startsWith("Right") ? -1 : 1;
            const auto mx = [d](double x) { return d > 0 ? x : 1 - x; };
            if (view_.endsWith("Clavicle")) {
                limb(mx(.1), .75, mx(.9), .55, .3);
            } else {
                limb(mx(.35), .1, mx(.75), 1.4, .38);
            }
        }
        p.setBrush(Qt::NoBrush);
        for (int w = 9; w >= 1; w -= 2) {
            QColor c = GLOW;
            c.setAlpha(w == 1 ? 230 : 30);
            p.setPen(QPen(c, w));
            p.drawPath(b);
        }
    }
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing);
        QRadialGradient bg(rect().center(), std::max(width(), height()) * .7);
        bg.setColorAt(0, QColor(48, 48, 48));
        bg.setColorAt(1, QColor(14, 14, 14));
        p.fillRect(rect(), bg);
        const double u = unit();
        if (mode_ == Definition || !rig) draw_body(p);
        const QList<Spot> list = spots();
        QHash<QString, QPointF> where;
        for (const Spot& s : list) where[s.slot] = at(s.x, s.y);
        if (mode_ == Controls && rig) {
            // Sticks for the bones, which select their control; circles for the effectors.
            p.setPen(QPen(QColor(200, 200, 200), .02 * u, Qt::SolidLine, Qt::RoundCap));
            if (view_.isEmpty()) {
                for (const auto& l : BONES)
                    if (where.contains(l[0]) && where.contains(l[1])) p.drawLine(where[l[0]], where[l[1]]);
            } else {
                for (int i = 0; i + 1 < list.size(); ++i)
                    if (list[i].slot.left(list[i].slot.size() - 1) == list[i + 1].slot.left(list[i + 1].slot.size() - 1) && filled.value(list[i + 1].slot)) p.drawLine(where[list[i].slot], where[list[i + 1].slot]);
            }
        }
        for (const Spot& s : list) {
            const bool on = filled.value(s.slot);
            // Without a control rig driving it the Controls figure is the body alone, as the reference's.
            if (mode_ == Controls && (!rig || (!view_.isEmpty() && !on))) continue;
            const QPointF c = at(s.x, s.y);
            const bool effector = mode_ == Controls && (s.slot.endsWith("Hand") || s.slot.endsWith("Foot") || s.slot == "Hips");
            p.setPen(picked == s.slot ? QPen(Qt::white, 2) : QPen(QColor(20, 20, 20), 1));
            p.setBrush(on ? (effector ? ASSIGNED : (mode_ == Controls ? QColor(205, 205, 205) : ASSIGNED)) : EMPTY);
            const double r = (view_.isEmpty() ? .028 : .035) * u * (effector ? 1.4 : 1);
            if (mode_ == Definition) {
                // Diamonds, as the reference's bone markers.
                QPolygonF d({c + QPointF(0, -r * 1.4), c + QPointF(r, 0), c + QPointF(0, r * 1.4), c + QPointF(-r, 0)});
                p.drawPolygon(d);
            } else {
                p.drawEllipse(c, r, r);
            }
        }
        // The ways in, and the way back.
        if (view_.isEmpty()) {
            for (const Door& d : body_doors(mode_ == Definition)) {
                const QPointF c = at(d.x, d.y);
                const double r = .03 * u;
                p.setPen(QPen(d.square ? QColor(230, 40, 40) : QColor(200, 200, 200), 1.5));
                p.setBrush(QColor(30, 30, 30));
                if (d.square) p.drawRect(QRectF(c.x() - r, c.y() - r, 2 * r, 2 * r));
                else {
                    p.drawEllipse(c, r, r);
                    p.drawLine(c + QPointF(-r * .45, -r * .15), c + QPointF(0, r * .35));
                    p.drawLine(c + QPointF(r * .45, -r * .15), c + QPointF(0, r * .35));
                }
            }
        } else {
            const QPointF c(22, 22);
            p.setPen(QPen(QColor(220, 220, 220), 2));
            p.setBrush(Qt::NoBrush);
            p.drawEllipse(c, 13, 13);
            p.setBrush(QColor(220, 220, 220));
            p.drawPolygon(QPolygonF({c + QPointF(-6, 4), c + QPointF(6, 4), c + QPointF(0, -5)}));
            // The other hand or foot, small, top right: a click there goes to it.
            if (mode_ == Controls && (view_.endsWith("Hand") || view_.endsWith("Foot"))) {
                p.setPen(QPen(QColor(160, 160, 160), 1.5));
                p.setBrush(Qt::NoBrush);
                const QRectF m(width() - 46, 8, 38, 38);
                p.drawRoundedRect(m, 6, 6);
                p.drawText(m, Qt::AlignCenter, view_.startsWith("Left") ? "R" : "L");
            }
        }
        // The definition's state, top right: a tick when it's valid.
        if (mode_ == Definition) {
            const QPointF c(width() - 26, 26);
            const QColor col = status == "red" ? QColor(214, 64, 58) : status == "yellow" ? QColor(232, 196, 60) : QColor(92, 190, 92);
            p.setPen(Qt::NoPen);
            p.setBrush(col);
            p.drawEllipse(c, 15, 15);
            p.setPen(QPen(Qt::white, 3, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
            if (status == "red") {
                p.drawLine(c + QPointF(-6, -6), c + QPointF(6, 6));
                p.drawLine(c + QPointF(6, -6), c + QPointF(-6, 6));
            } else {
                p.drawPolyline(QPolygonF({c + QPointF(-7, 0), c + QPointF(-2, 5), c + QPointF(7, -6)}));
            }
        }
        if (!picked.isEmpty()) {
            p.setPen(QColor(200, 200, 200));
            p.drawText(rect().adjusted(8, 0, -8, -6), Qt::AlignLeft | Qt::AlignBottom, picked);
        }
    }
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() != Qt::LeftButton) return;
        const QPointF pos = e->position();
        if (!view_.isEmpty()) {
            if (QLineF(pos, QPointF(22, 22)).length() < 16) {
                set_view("");
                return;
            }
            if (mode_ == Controls && (view_.endsWith("Hand") || view_.endsWith("Foot")) && QRectF(width() - 46, 8, 38, 38).contains(pos)) {
                set_view((view_.startsWith("Left") ? "Right" : "Left") + view_.mid(view_.startsWith("Left") ? 4 : 5));
                return;
            }
        } else {
            for (const Door& d : body_doors(mode_ == Definition))
                if (QLineF(pos, at(d.x, d.y)).length() < .045 * unit()) {
                    set_view(d.view);
                    return;
                }
        }
        if (const Spot* s = spot_at(e->position().toPoint())) {
            picked = s->slot;
            update();
            if (picked_slot) picked_slot(picked);
        }
    }
    void mouseMoveEvent(QMouseEvent* e) override {
        const Spot* s = spot_at(e->position().toPoint());
        setToolTip(s ? s->slot : QString());
    }

private:
    Mode mode_;
    QString view_;
};

// A section that folds, as the reference's frames do: an arrow and a title over its rows.
QWidget* section(const QString& title, QWidget* body) {
    auto* w = new QWidget;
    auto* v = new QVBoxLayout(w);
    v->setContentsMargins(0, 0, 0, 0);
    v->setSpacing(2);
    auto* head = new QToolButton;
    head->setText(title);
    head->setCheckable(true);
    head->setChecked(true);
    head->setArrowType(Qt::DownArrow);
    head->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
    head->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    head->setStyleSheet("QToolButton { font-weight: bold; text-align: left; border: none; background: palette(mid); padding: 3px; }");
    QObject::connect(head, &QToolButton::toggled, body, [head, body](bool on) {
        body->setVisible(on);
        head->setArrowType(on ? Qt::DownArrow : Qt::RightArrow);
    });
    v->addWidget(head);
    v->addWidget(body);
    return w;
}

class HumanoidIkPanel : public Panel {
public:
    explicit HumanoidIkPanel(QWidget* parent) : Panel(parent) {
        auto* v = new QVBoxLayout(this);
        v->setContentsMargins(4, 4, 4, 4);
        v->setSpacing(4);
        // The menu button, then Character and Source.
        auto* top = new QGridLayout;
        auto* menu_button = new QToolButton;
        menu_button->setText("HIK");
        menu_button->setToolTip("Humanoid IK menu");
        menu_button->setFixedSize(40, 40);
        menu_button->setPopupMode(QToolButton::InstantPopup);
        menu_button->setMenu(build_menu());
        top->addWidget(menu_button, 0, 0, 2, 1);
        top->addWidget(new QLabel("Character:"), 0, 1, Qt::AlignRight);
        character_ = new QComboBox;
        top->addWidget(character_, 0, 2);
        top->addWidget(new QLabel("Source:"), 1, 1, Qt::AlignRight);
        source_ = new QComboBox;
        top->addWidget(source_, 1, 2);
        top->setColumnStretch(2, 1);
        v->addLayout(top);

        // The tool buttons, changing with the tab as the reference's do.
        tools_ = new QStackedWidget;
        tools_->setSizePolicy(QSizePolicy::Preferred, QSizePolicy::Fixed);
        v->addWidget(tools_);

        tabs_ = new QTabWidget;
        tabs_->setDocumentMode(true);
        v->addWidget(tabs_, 1);
        tabs_->addTab(build_skeleton(), "Skeleton");
        tabs_->addTab(build_definition(), "Definition");
        tabs_->addTab(build_controls(), "Controls");
        tabs_->addTab(build_properties(), "Properties");
        tools_->addWidget(tool_row({{"+", "Create Skeleton: a humanoid skeleton as set below, with its definition", [this] { create_skeleton(); }}}));
        tools_->addWidget(tool_row({{"+", "Create Character Definition", [this] { create_character(); }},
                                    {"Lock", "Lock or unlock the definition; its T-pose is where the joints are now", [] { exec("hikToggleLockDefinition"); }},
                                    {"Mirror", "Mirror Matching: fill empty slots from their opposites", [this] { if (!name_.isEmpty()) exec("hikMirrorMatching", QJsonObject{{"args", QJsonArray{name_}}}); }},
                                    {"Load", "Load Skeleton Definition...", [this] { load_template(); }},
                                    {"Save", "Save Skeleton Definition...", [this] { save_template(); }},
                                    {"Del", "Delete Character Definition", [this] { if (!name_.isEmpty()) exec("delete", QJsonObject{{"args", QJsonArray{name_}}}); }}}));
        tools_->addWidget(tool_row({{"+", "Create Control Rig", [this] { exec("hikCreateControlRig", QJsonObject{{"args", QJsonArray{name_}}}); }},
                                    {"Del", "Delete Control Rig", [this] { exec("hikDeleteControlRig", QJsonObject{{"args", QJsonArray{name_}}}); }},
                                    {"Stance", "Stance pose", [] { exec("hikSetCurrentSource", QJsonObject{{"args", QJsonArray{"Stance"}}}); }}}));
        tools_->addWidget(tool_row({}));
        connect(tabs_, &QTabWidget::currentChanged, tools_, &QStackedWidget::setCurrentIndex);
        tabs_->setCurrentIndex(1);

        connect(character_, &QComboBox::activated, this, [this](int) {
            exec("hikSetCurrentCharacter", QJsonObject{{"args", QJsonArray{character_->currentText()}}});
        });
        connect(source_, &QComboBox::activated, this, [this](int) {
            exec("hikSetCurrentSource", QJsonObject{{"args", QJsonArray{source_->currentText()}}});
        });
        schedule();
    }

protected:
    void refresh() override {
        const QJsonObject info = ask("hikInfo").toObject();
        const QJsonObject me = info["character"].toObject();
        name_ = me["name"].toString();
        const QSignalBlocker a(character_), b(source_);
        character_->clear();
        for (const auto& c : info["characters"].toArray()) character_->addItem(c.toString());
        character_->setCurrentText(name_);
        source_->clear();
        source_->addItems({"None", "Stance", "Control Rig"});
        for (const auto& c : info["characters"].toArray())
            if (c.toString() != name_) source_->addItem(c.toString());
        source_->setCurrentText(me["source"].toString("None"));
        slots_ = me["slots"].toObject();
        QHash<QString, bool> filled;
        for (auto it = slots_.begin(); it != slots_.end(); ++it) filled[it.key()] = !it.value().toString().isEmpty();
        const QJsonObject st = me["status"].toObject();
        definition_->filled = filled;
        definition_->status = name_.isEmpty() || st["valid"].toBool() ? "green" : "red";
        definition_->update();
        controls_->filled = filled;
        controls_->rig = me["controlRig"].toBool() && me["source"].toString() == "Control Rig";
        controls_->update();
        QString text = st["summary"].toString();
        for (const auto& m : st["messages"].toArray()) text += "\n" + m.toString();
        status_->setPlainText(name_.isEmpty() ? "No character: + makes a definition, or the Skeleton tab a skeleton with one." : text);
        show_joint();
        property_node_ = me["propertyNode"].toString();
        const QJsonObject props = me["properties"].toObject();
        updating_ = true;
        for (auto it = prop_spins_.begin(); it != prop_spins_.end(); ++it) it.value()->setValue(props[it.key()].toDouble());
        for (auto it = prop_checks_.begin(); it != prop_checks_.end(); ++it) it.value()->setChecked(props[it.key()].toDouble() != 0);
        for (auto it = prop_modes_.begin(); it != prop_modes_.end(); ++it) it.value()->setCurrentIndex(int(props[it.key()].toDouble()));
        updating_ = false;
    }

private:
    struct Tool {
        QString text, tip;
        std::function<void()> fn;
    };
    QWidget* tool_row(const QList<Tool>& tools) {
        auto* w = new QWidget;
        auto* h = new QHBoxLayout(w);
        h->setContentsMargins(0, 0, 0, 0);
        h->setSpacing(2);
        for (const Tool& t : tools) {
            auto* b = new QToolButton;
            b->setText(t.text);
            b->setToolTip(t.tip);
            b->setAutoRaise(true);
            connect(b, &QToolButton::clicked, this, t.fn);
            h->addWidget(b);
        }
        h->addStretch(1);
        return w;
    }

    // Skeleton: the reference's frames of a skeleton to make, Scale and Bones, Roll Bones, Finger Bones and Toe Bones.
    QWidget* build_skeleton() {
        auto* inner = new QWidget;
        auto* v = new QVBoxLayout(inner);
        v->setContentsMargins(2, 2, 2, 2);
        auto form = [](QWidget* w) {
            auto* f = new QFormLayout(w);
            f->setLabelAlignment(Qt::AlignLeft);
            return f;
        };
        auto spin = [this](const QString& key, int lo, int hi, int val) {
            auto* s = new QSpinBox;
            s->setRange(lo, hi);
            s->setValue(val);
            skel_ints_[key] = s;
            return s;
        };
        auto check = [this](const QString& key, const QString& label, bool on) {
            auto* c = new QCheckBox(label);
            c->setChecked(on);
            skel_checks_[key] = c;
            return c;
        };
        auto* scale = new QWidget;
        auto* f1 = form(scale);
        scale_ = new QDoubleSpinBox;
        scale_->setRange(.01, 100);
        scale_->setValue(1);
        f1->addRow("Character Scale", scale_);
        f1->addRow("Spine", spin("spineCount", 1, 10, 3));
        f1->addRow("Neck", spin("neckCount", 0, 10, 1));
        f1->addRow("Shoulder / Clavicle", spin("shoulderCount", 0, 2, 1));
        v->addWidget(section("Scale and Bones", scale));
        auto* roll = new QWidget;
        auto* f2 = form(roll);
        f2->addRow("Upper-Arms", spin("upperArmRolls", 0, 5, 0));
        f2->addRow("Lower-Arms", spin("lowerArmRolls", 0, 5, 0));
        f2->addRow("Upper-Legs", spin("upperLegRolls", 0, 5, 0));
        f2->addRow("Lower-Legs", spin("lowerLegRolls", 0, 5, 0));
        v->addWidget(section("Roll Bones", roll));
        auto digits = [&](const QString& title, const QStringList& keys, const QStringList& labels, const QList<bool>& on, const QString& bones, const QString& base,
                          const QString& base_label, const QString& in, const QString& in_label) {
            auto* w = new QWidget;
            auto* g = new QGridLayout(w);
            for (int i = 0; i < keys.size(); ++i) g->addWidget(check(keys[i], labels[i], on[i]), i / 2, i % 2);
            const int r = (keys.size() + 1) / 2;
            g->addWidget(new QLabel("Num Bones"), r, 0);
            g->addWidget(spin(bones, 1, 4, 3), r, 1);
            g->addWidget(check(base, base_label, false), r + 1, 0);
            g->addWidget(check(in, in_label, false), r + 1, 1);
            v->addWidget(section(title, w));
        };
        digits("Finger Bones", {"index", "middle", "ring", "pinky", "thumb", "extraFinger"}, {"Index", "Middle", "Ring", "Pinky", "Thumb", "Extra"}, {true, true, true, true, true, false},
               "fingerBones", "fingerBase", "Finger-Base", "inHandBones", "In-Hand Bones");
        digits("Toe Bones", {"toeIndex", "toeMiddle", "toeRing", "toePinky", "footThumb", "extraToe"}, {"Index", "Middle", "Ring", "Pinky", "Big Toe", "Foot-Thumb"},
               {false, false, false, false, false, false}, "toeBones", "toeBase", "Toe-Base", "inFootBones", "In-Foot Bones");
        skel_checks_["toeBase"]->setChecked(true);
        auto* make = new QPushButton("Create Skeleton");
        make->setToolTip("A humanoid skeleton in T-pose as set above, with a character definition for it");
        connect(make, &QPushButton::clicked, this, [this] { create_skeleton(); });
        v->addWidget(make);
        v->addStretch(1);
        auto* area = new QScrollArea;
        area->setWidget(inner);
        area->setWidgetResizable(true);
        area->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        return area;
    }
    void create_skeleton() {
        bool ok = false;
        const QString n = QInputDialog::getText(this, "Create Skeleton", "Name", QLineEdit::Normal, "Character1", &ok);
        if (!ok || n.isEmpty()) return;
        QJsonObject call{{"name", n}, {"character", true}, {"characterScale", scale_->value()}};
        for (auto it = skel_ints_.begin(); it != skel_ints_.end(); ++it) call[it.key()] = it.value()->value();
        for (auto it = skel_checks_.begin(); it != skel_checks_.end(); ++it) call[it.key()] = it.value()->isChecked();
        exec("hikCreateSkeleton", call);
    }

    // Definition: the figure, the status, and the picked slot's joint with its values.
    QWidget* build_definition() {
        auto* inner = new QWidget;
        auto* v = new QVBoxLayout(inner);
        v->setContentsMargins(2, 2, 2, 2);
        definition_ = new Figure(Figure::Definition);
        definition_->picked_slot = [this](const QString& slot) {
            // As the reference's: picking a slot selects its joint in the scene.
            const QString j = slots_[slot].toString();
            if (!j.isEmpty()) exec("select", QJsonObject{{"args", QJsonArray{j}}});
            show_joint();
        };
        definition_->slot_menu = [this](const QString& slot, const QPoint& at) {
            definition_->picked = slot;
            QMenu m;
            m.addAction("Assign Selected Bone", [this] { assign_selected(); });
            m.addAction("Remove Assignment", [this, slot] {
                if (!name_.isEmpty()) exec("hikRemoveCharacterObject", QJsonObject{{"args", QJsonArray{name_, slot}}});
            });
            m.addAction("Select Bone", [this, slot] {
                if (!slots_[slot].toString().isEmpty()) exec("select", QJsonObject{{"args", QJsonArray{slots_[slot].toString()}}});
            });
            m.exec(at);
        };
        v->addWidget(definition_, 3);
        v->addWidget(new QLabel("Status"));
        status_ = new QPlainTextEdit;
        status_->setReadOnly(true);
        status_->setMaximumHeight(70);
        v->addWidget(status_);
        auto* grid = new QGridLayout;
        grid->setVerticalSpacing(2);
        int row = 0;
        auto line = [&](const QString& label, int fields) {
            grid->addWidget(new QLabel(label), row, 0, Qt::AlignRight);
            QList<QLineEdit*> out;
            for (int i = 0; i < fields; ++i) {
                auto* e = new QLineEdit;
                e->setReadOnly(true);
                grid->addWidget(e, row, 1 + i * (3 / fields), 1, 3 / fields);
                out << e;
            }
            ++row;
            return out;
        };
        node_ = line("Humanoid IK node", 1)[0];
        bone_ = line("Bone", 1)[0];
        scene_sel_ = line("Scene Selection", 1)[0];
        const QStringList three = {"Translate", "Rotate", "Scale", "Pre-rotate", "Post-rotate", "Min rot limit", "Max rot limit", "Parent offset R"};
        for (const QString& t : three) vectors_[t] = line(t, 3);
        order_ = line("Rotation order", 1)[0];
        v->addLayout(grid);
        auto* area = new QScrollArea;
        area->setWidget(inner);
        area->setWidgetResizable(true);
        area->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        return area;
    }
    // The picked slot's joint and its values, as the reference lists them under the figure.
    void show_joint() {
        const QString slot = definition_->picked;
        const QString j = slots_[slot].toString();
        node_->setText(slot.isEmpty() ? "No node selected." : slot);
        bone_->setText(j);
        const QJsonArray sel = ask("ls", QJsonObject{{"selection", true}}).toArray();
        scene_sel_->setText(sel.isEmpty() ? QString() : sel.first().toString());
        const QHash<QString, QString> attr = {{"Translate", "translate"}, {"Rotate", "rotate"}, {"Scale", "scale"}, {"Pre-rotate", "jointOrient"}, {"Post-rotate", "rotateAxis"},
                                              {"Min rot limit", "minRotLimit"}, {"Max rot limit", "maxRotLimit"}, {"Parent offset R", "rotatePivot"}};
        for (auto it = vectors_.begin(); it != vectors_.end(); ++it) {
            const QJsonArray val = j.isEmpty() ? QJsonArray() : ask("getAttr", QJsonObject{{"args", QJsonArray{j + "." + attr[it.key()]}}}).toArray();
            const QJsonArray xyz = !val.isEmpty() && val.first().isArray() ? val.first().toArray() : val;
            for (int i = 0; i < 3; ++i) it.value()[i]->setText(i < xyz.size() ? QString::number(xyz[i].toDouble(), 'f', 2) : QString());
        }
        static const QStringList orders = {"xyz", "yzx", "zxy", "xzy", "yxz", "zyx"};
        order_->setText(j.isEmpty() ? QString() : orders.value(ask("getAttr", QJsonObject{{"args", QJsonArray{j + ".rotateOrder"}}}).toInt()));
    }

    // Controls: the control rig's figure, sticks for the bones and circles for the effectors, and the Humanoid IK Controls frame.
    QWidget* build_controls() {
        auto* w = new QWidget;
        auto* v = new QVBoxLayout(w);
        v->setContentsMargins(2, 2, 2, 2);
        controls_ = new Figure(Figure::Controls);
        controls_->picked_slot = [this](const QString& slot) { select_control(slot); };
        v->addWidget(controls_, 1);
        auto* frame = new QWidget;
        auto* f = new QFormLayout(frame);
        auto slider = [&](const QString& label, bool works) {
            auto* row = new QWidget;
            auto* h = new QHBoxLayout(row);
            h->setContentsMargins(0, 0, 0, 0);
            auto* num = new QLabel("0.00");
            auto* s = new QSlider(Qt::Horizontal);
            s->setRange(0, 100);
            s->setEnabled(works);
            if (!works) s->setToolTip("Not in Aztek yet");
            h->addWidget(num);
            h->addWidget(s, 1);
            f->addRow(label, row);
            connect(s, &QSlider::valueChanged, num, [num](int x) { num->setText(QString::number(x / 100.0, 'f', 2)); });
            return s;
        };
        blend_t_ = slider("IK Blend T", true);
        slider("IK Blend R", false);
        slider("IK Pull", false);
        connect(blend_t_, &QSlider::sliderReleased, this, [this] {
            if (!ik_.isEmpty()) exec("setAttr", QJsonObject{{"args", QJsonArray{ik_ + ".ikBlend", blend_t_->value() / 100.0}}});
        });
        v->addWidget(section("Humanoid IK Controls", frame));
        return w;
    }
    // A click on the control rig figure: the effector for a hand or foot, else the bone's own control; a finger's stick
    // its whole finger.
    void select_control(const QString& slot) {
        if (name_.isEmpty()) return;
        QJsonArray pick;
        ik_.clear();
        auto ctrl = [this](const QString& s) { return QString("%1_Ctrl_%2").arg(name_, s); };
        const bool exists_hand = slot == "LeftHand" || slot == "RightHand", exists_foot = slot == "LeftFoot" || slot == "RightFoot";
        if (exists_hand || exists_foot) {
            const QString side = slot.startsWith("Left") ? "Left" : "Right";
            pick.append(ctrl(side + (exists_hand ? "HandEffector" : "AnkleEffector")));
            ik_ = QString("%1_%2%3IK").arg(name_, side, exists_hand ? "Hand" : "Foot");
        } else if (slot.contains("Hand") && slot.at(slot.size() - 1).isDigit()) {
            const QString base = slot.chopped(1);
            for (int b = 1; b <= 4; ++b) {
                const QString s = base + QString::number(b);
                if (slots_[s].toString().isEmpty()) continue;
                const bool has_ctrl = ask("objExists", QJsonObject{{"args", QJsonArray{ctrl(s)}}}).toBool();
                pick.append(has_ctrl ? ctrl(s) : slots_[s].toString());
            }
        } else {
            const bool has_ctrl = ask("objExists", QJsonObject{{"args", QJsonArray{ctrl(slot)}}}).toBool();
            pick.append(has_ctrl ? ctrl(slot) : slots_[slot].toString());
        }
        if (!ik_.isEmpty() && ask("objExists", QJsonObject{{"args", QJsonArray{ik_}}}).toBool()) {
            const QSignalBlocker b(blend_t_);
            blend_t_->setValue(int(ask("getAttr", QJsonObject{{"args", QJsonArray{ik_ + ".ikBlend"}}}).toDouble(1) * 100));
        } else {
            ik_.clear();
        }
        if (!pick.isEmpty()) exec("select", QJsonObject{{"args", pick}});
    }
    void load_template() {
        const QString f = QFileDialog::getOpenFileName(this, "Load Skeleton Definition", QString(), "Templates (*.xml)");
        if (!f.isEmpty()) exec("hikLoadTemplate", QJsonObject{{"args", QJsonArray{f, name_}}});
    }
    void save_template() {
        const QString f = QFileDialog::getSaveFileName(this, "Save Skeleton Definition", QString(), "Templates (*.xml)");
        if (!f.isEmpty()) exec("hikSaveTemplate", QJsonObject{{"args", QJsonArray{f, name_}}});
    }
    void set_prop(const QString& name, double v) {
        if (!updating_ && !property_node_.isEmpty()) exec("setAttr", QJsonObject{{"args", QJsonArray{property_node_ + "." + name, v}}});
    }
    QWidget* build_properties() {
        auto* inner = new QWidget;
        auto* form = new QFormLayout(inner);
        const auto mode = [&](const QString& label, const QString& attr) {
            auto* c = new QComboBox;
            c->addItems({"Off", "Auto", "User"});
            connect(c, &QComboBox::activated, this, [this, attr](int i) { set_prop(attr, i); });
            prop_modes_[attr] = c;
            form->addRow(label, c);
        };
        const auto spin = [&](const QString& label, const QString& attr, double lo, double hi) {
            auto* s = new QDoubleSpinBox;
            s->setRange(lo, hi);
            connect(s, &QDoubleSpinBox::editingFinished, this, [this, attr, s] { set_prop(attr, s->value()); });
            prop_spins_[attr] = s;
            form->addRow(label, s);
        };
        const auto check = [&](const QString& label, const QString& attr) {
            auto* c = new QCheckBox;
            connect(c, &QCheckBox::clicked, this, [this, attr](bool on) { set_prop(attr, on ? 1 : 0); });
            prop_checks_[attr] = c;
            form->addRow(label, c);
        };
        form->addRow(new QLabel("Retarget Specific"));
        check("Match Source", "MatchSource");
        mode("Action Space Comp Mode", "ScaleCompensationMode");
        spin("Action Space Compensation", "ScaleCompensation", 0, 1000);
        check("Mirror Animation", "Mirror");
        mode("Hips Level Mode", "HipsHeightCompensationMode");
        spin("Hips Level", "HipsHeightCompensation", -1000, 1000);
        mode("Feet Spacing Mode", "AnkleProximityCompensationMode");
        spin("Feet Spacing", "AnkleProximityCompensation", -1000, 1000);
        mode("Ankle Height Comp Mode", "AnkleHeightCompensationMode");
        spin("Ankle Height Compensation", "AnkleHeightCompensation", -1000, 1000);
        form->addRow(new QLabel("Reach"));
        spin("Left Hand Reach T", "ReachActorLeftWrist", 0, 100);
        spin("Right Hand Reach T", "ReachActorRightWrist", 0, 100);
        auto* area = new QScrollArea;
        area->setWidget(inner);
        area->setWidgetResizable(true);
        return area;
    }
    QMenu* build_menu() {
        auto* m = new QMenu(this);
        m->addAction("Create Character", this, [this] { create_character(); });
        m->addAction("Rename Character...", this, [this] {
            bool ok = false;
            const QString n = QInputDialog::getText(this, "Rename Character", "Name", QLineEdit::Normal, name_, &ok);
            if (ok && !n.isEmpty()) {
                exec("rename", QJsonObject{{"args", QJsonArray{name_, n}}});
                exec("hikSetCurrentCharacter", QJsonObject{{"args", QJsonArray{n}}});
            }
        });
        m->addAction("Delete Character", this, [this] { if (!name_.isEmpty()) exec("delete", QJsonObject{{"args", QJsonArray{name_}}}); });
        m->addSeparator();
        m->addAction("Save Skeleton Definition...", this, [this] {
            const QString f = QFileDialog::getSaveFileName(this, "Save Template", QString(), "Templates (*.xml)");
            if (!f.isEmpty()) exec("hikSaveTemplate", QJsonObject{{"args", QJsonArray{f, name_}}});
        });
        m->addAction("Load Skeleton Definition...", this, [this] {
            const QString f = QFileDialog::getOpenFileName(this, "Load Template", QString(), "Templates (*.xml)");
            if (!f.isEmpty()) exec("hikLoadTemplate", QJsonObject{{"args", QJsonArray{f, name_}}});
        });
        m->addSeparator();
        m->addAction("Bake to Skeleton", this, [this] { exec("hikBakeToSkeleton", QJsonObject{{"args", QJsonArray{name_}}}); });
        m->addAction("Bake to Skeleton Options...", this, [this] { bake_options("hikBakeToSkeleton"); });
        m->addAction("Bake to Control Rig", this, [this] { exec("hikBakeToControlRig", QJsonObject{{"args", QJsonArray{name_}}}); });
        m->addAction("Bake to Control Rig Options...", this, [this] { bake_options("hikBakeToControlRig"); });
        m->addSeparator();
        m->addAction("Delete Control Rig", this, [this] { exec("hikDeleteControlRig", QJsonObject{{"args", QJsonArray{name_}}}); });
        return m;
    }
    void bake_options(const QString& command) {
        QDialog d(this);
        d.setWindowTitle("Bake Options");
        auto* form = new QFormLayout(&d);
        auto* start = new QDoubleSpinBox;
        auto* end = new QDoubleSpinBox;
        start->setRange(-1e9, 1e9);
        end->setRange(-1e9, 1e9);
        const QJsonObject st = state();
        start->setValue(st["playbackMin"].toDouble(1));
        end->setValue(st["playbackMax"].toDouble(120));
        auto* by = new QDoubleSpinBox;
        by->setRange(0.01, 1000);
        by->setValue(1);
        auto* euler = new QCheckBox;
        euler->setChecked(true);
        form->addRow("Start Time", start);
        form->addRow("End Time", end);
        form->addRow("Sample By", by);
        form->addRow("Euler Filter", euler);
        auto* box = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel);
        connect(box, &QDialogButtonBox::accepted, &d, &QDialog::accept);
        connect(box, &QDialogButtonBox::rejected, &d, &QDialog::reject);
        form->addRow(box);
        if (d.exec() == QDialog::Accepted)
            exec(command, QJsonObject{{"args", QJsonArray{name_}}, {"time", QString("%1:%2").arg(start->value()).arg(end->value())}, {"sampleBy", by->value()}, {"eulerFilter", euler->isChecked()}});
    }
    void create_character() {
        bool ok = false;
        const QString n = QInputDialog::getText(this, "Create Character", "Name", QLineEdit::Normal, "Character1", &ok);
        if (ok && !n.isEmpty()) exec("hikCreateCharacter", QJsonObject{{"args", QJsonArray{n}}});
    }
    void assign_selected() {
        const QJsonArray sel = ask("ls", QJsonObject{{"selection", true}}).toArray();
        if (name_.isEmpty() || definition_->picked.isEmpty() || sel.isEmpty()) {
            log("Pick a slot in the figure and select a joint, then assign.", 2);
            return;
        }
        exec("setCharacterObject", QJsonObject{{"args", QJsonArray{sel.first().toString(), name_, definition_->picked, 0}}});
    }
    QComboBox* character_ = nullptr;
    QComboBox* source_ = nullptr;
    QStackedWidget* tools_ = nullptr;
    QTabWidget* tabs_ = nullptr;
    Figure* definition_ = nullptr;
    Figure* controls_ = nullptr;
    QPlainTextEdit* status_ = nullptr;
    QLineEdit* node_ = nullptr;
    QLineEdit* bone_ = nullptr;
    QLineEdit* scene_sel_ = nullptr;
    QLineEdit* order_ = nullptr;
    QHash<QString, QList<QLineEdit*>> vectors_;
    QDoubleSpinBox* scale_ = nullptr;
    QHash<QString, QSpinBox*> skel_ints_;
    QHash<QString, QCheckBox*> skel_checks_;
    QSlider* blend_t_ = nullptr;
    QString ik_;
    QJsonObject slots_;
    QString name_, property_node_;
    QHash<QString, QDoubleSpinBox*> prop_spins_;
    QHash<QString, QCheckBox*> prop_checks_;
    QHash<QString, QComboBox*> prop_modes_;
    bool updating_ = false;
};

} // namespace

QWidget* human_ik_panel(QWidget* parent) {
    return new HumanoidIkPanel(parent);
}

} // namespace az

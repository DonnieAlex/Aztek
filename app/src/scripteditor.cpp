// The Script Editor: the history pane over tabs of MEL and Python, with syntax colours, line numbers, Execute and the
// usual file and options menus.
#include "panels.h"
#include "host.h"
#include "icons.h"
#include "markingmenu.h"
#include "shelves.h"

#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QCheckBox>
#include <QDialog>
#include <QDir>
#include <QDesktopServices>
#include <QDrag>
#include <QFile>
#include <QFileDialog>
#include <QFileInfo>
#include <QFrame>
#include <QGridLayout>
#include <QGuiApplication>
#include <QInputDialog>
#include <QJsonArray>
#include <QKeyEvent>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMenuBar>
#include <QMessageBox>
#include <QMimeData>
#include <QMouseEvent>
#include <QPainter>
#include <QPlainTextEdit>
#include <QProxyStyle>
#include <QPushButton>
#include <QScrollBar>
#include <QStackedWidget>
#include <QSettings>
#include <QStandardPaths>
#include <QShortcut>
#include <QSplitter>
#include <QStyleOptionMenuItem>
#include <QStyleOptionSlider>
#include <QSyntaxHighlighter>
#include <QTabBar>
#include <QTextBlock>
#include <QTextDocument>
#include <QTextOption>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>
#include <functional>

namespace az {

namespace {

bool light_interface() { return QGuiApplication::palette().color(QPalette::Window).lightnessF() > 0.5; }

// History colours by kind (output, result, error, echo, warning), for dark and for light themes (#88).
const char* history_colour(int kind) {
    static const char* dark[] = {"#d2d2d2", "#9fd0ff", "#ff8080", "#a0a0a0", "#f0c060"};
    static const char* light[] = {"#1e1e1e", "#0b5cad", "#b3201b", "#5c5c5c", "#8a5a00"};
    return (light_interface() ? light : dark)[qBound(0, kind, 4)];
}

class Highlighter : public QSyntaxHighlighter {
public:
    Highlighter(QTextDocument* d, bool py) : QSyntaxHighlighter(d), python_(py) {
        const bool l = light_interface();
        QTextCharFormat kw, str, com, num, var, flag, member;
        kw.setForeground(l ? QColor(0, 110, 0) : QColor(0, 224, 0));
        kw.setFontWeight(QFont::Bold);
        str.setForeground(l ? QColor(120, 80, 0) : QColor(224, 192, 0));
        com.setForeground(l ? QColor(60, 110, 60) : QColor(127, 159, 127));
        num.setForeground(l ? QColor(120, 40, 160) : QColor(190, 150, 230));
        var.setForeground(l ? QColor(0, 115, 125) : QColor(120, 215, 215));
        flag.setForeground(l ? QColor(80, 110, 20) : QColor(170, 200, 130));
        member.setForeground(l ? QColor(0, 115, 125) : QColor(0, 229, 229));
        QString kws = py ? "and|as|assert|break|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|None|nonlocal|not|or|pass|raise|return|True|False|try|while|with|yield"
                         : "if|else|for|in|while|do|switch|case|default|break|continue|return|proc|global|int|float|string|vector|matrix|source";
        rules_.append({QRegularExpression("\\b(" + kws + ")\\b"), kw});
        rules_.append({QRegularExpression("\\b[0-9]+(\\.[0-9]*)?\\b"), num});
        rules_.append({QRegularExpression("(?<=\\.)[A-Za-z_][A-Za-z0-9_]*"), member});
        if (!py) {
            rules_.append({QRegularExpression("\\$[A-Za-z_][A-Za-z0-9_]*"), var});
            rules_.append({QRegularExpression("(?<=\\s)-[A-Za-z][A-Za-z0-9]*"), flag});
        }
        rules_.append({QRegularExpression("\"(\\\\.|[^\"\\\\])*\"|'(\\\\.|[^'\\\\])*'"), str});
        rules_.append({QRegularExpression(py ? "#[^\n]*" : "//[^\n]*"), com});
    }
protected:
    void highlightBlock(const QString& text) override {
        for (const auto& r : rules_) {
            auto it = r.first.globalMatch(text);
            while (it.hasNext()) {
                auto m = it.next();
                setFormat(m.capturedStart(), m.capturedLength(), r.second);
            }
        }
    }
private:
    bool python_;
    QList<QPair<QRegularExpression, QTextCharFormat>> rules_;
};

// The reference's scroll bars: an 8 pixel groove, a bordered handle and triangle arrows at the ends.
class ScriptScrollBar : public QScrollBar {
public:
    ScriptScrollBar(Qt::Orientation o, QWidget* parent) : QScrollBar(o, parent) {
        if (o == Qt::Vertical) setFixedWidth(8);
        else setFixedHeight(8);
    }
protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        const bool v = orientation() == Qt::Vertical;
        p.fillRect(rect(), QColor(68, 68, 68));
        QStyleOptionSlider opt;
        initStyleOption(&opt);
        const int arrow = 8;
        QRect first = v ? QRect(0, 0, width(), arrow) : QRect(0, 0, arrow, height());
        QRect last = v ? QRect(0, height() - arrow, width(), arrow) : QRect(width() - arrow, 0, arrow, height());
        p.setPen(QColor(111, 111, 111));
        if (v) p.drawLine(0, 0, 0, height());
        else p.drawLine(0, 0, width(), 0);
        const QRect slider = style()->subControlRect(QStyle::CC_ScrollBar, &opt, QStyle::SC_ScrollBarSlider, this);
        if (slider.isValid() && slider.height() > 0 && slider.width() > 0) {
            p.fillRect(slider, QColor(94, 94, 94));
            p.setPen(QColor(111, 111, 111));
            p.drawRect(slider.adjusted(0, 0, -1, -1));
        }
        p.setPen(Qt::NoPen);
        p.setBrush(QColor(200, 200, 200));
        auto triangle = [&](const QRect& r, int dx, int dy) {
            const QPointF c = QRectF(r).center();
            QPolygonF t;
            if (dy < 0) t = {c + QPointF(-3, 1.5), c + QPointF(3, 1.5), c + QPointF(0, -2)};
            else if (dy > 0) t = {c + QPointF(-3, -1.5), c + QPointF(3, -1.5), c + QPointF(0, 2)};
            else if (dx < 0) t = {c + QPointF(1.5, -3), c + QPointF(1.5, 3), c + QPointF(-2, 0)};
            else t = {c + QPointF(-1.5, -3), c + QPointF(-1.5, 3), c + QPointF(2, 0)};
            p.drawPolygon(t);
        };
        if (v) {
            triangle(first, 0, -1);
            triangle(last, 0, 1);
        } else {
            triangle(first, -1, 0);
            triangle(last, 1, 0);
        }
    }
};

class Gutter;

// A script tab: line numbers in a gutter, and a middle drag of the selected text to a shelf.
class ScriptEdit : public QPlainTextEdit {
public:
    ScriptEdit(const QString& language, QWidget* parent);
    QString language;
    QString path;
    void set_numbers(bool on) {
        numbers_ = on;
        update_margin();
    }
    int gutter_width() const {
        if (!numbers_) return 0;
        int digits = QString::number(qMax(1, blockCount())).size();
        return qMax(26, 12 + fontMetrics().horizontalAdvance('9') * digits);
    }
    void paint_gutter(QPaintEvent* e);
protected:
    void resizeEvent(QResizeEvent* e) override;
    void mousePressEvent(QMouseEvent* e) override;
    void changeEvent(QEvent* e) override {
        QPlainTextEdit::changeEvent(e);
        // A theme changed: the highlighting follows it.
        if (e->type() == QEvent::PaletteChange || e->type() == QEvent::StyleChange) {
            delete highlighter_;
            highlighter_ = new Highlighter(document(), language == "python");
        }
    }
private:
    Highlighter* highlighter_ = nullptr;
    void update_margin() { setViewportMargins(gutter_width(), 0, 0, 0); }
    bool numbers_ = false;
    QWidget* gutter_;
    friend class Gutter;
};

class Gutter : public QWidget {
public:
    explicit Gutter(ScriptEdit* e) : QWidget(e), edit_(e) {}
protected:
    void paintEvent(QPaintEvent* e) override { edit_->paint_gutter(e); }
private:
    ScriptEdit* edit_;
};

ScriptEdit::ScriptEdit(const QString& lang, QWidget* parent) : QPlainTextEdit(parent), language(lang) {
    setProperty("scriptLanguage", lang);
    gutter_ = new Gutter(this);
    connect(this, &QPlainTextEdit::blockCountChanged, this, [this] { update_margin(); });
    connect(this, &QPlainTextEdit::updateRequest, this, [this](const QRect& r, int dy) {
        if (dy) gutter_->scroll(0, dy);
        else gutter_->update(0, r.y(), gutter_->width(), r.height());
    });
    highlighter_ = new Highlighter(document(), lang == "python");
    setObjectName("scriptInput");
    setLineWrapMode(QPlainTextEdit::NoWrap);
    setFrameShape(QFrame::NoFrame);
    setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOn);
    setVerticalScrollBarPolicy(Qt::ScrollBarAsNeeded);
    document()->setDocumentMargin(2);
    setHorizontalScrollBar(new ScriptScrollBar(Qt::Horizontal, this));
    setVerticalScrollBar(new ScriptScrollBar(Qt::Vertical, this));
}

void ScriptEdit::resizeEvent(QResizeEvent* e) {
    QPlainTextEdit::resizeEvent(e);
    QRect c = contentsRect();
    gutter_->setGeometry(QRect(c.left(), c.top(), gutter_width(), c.height()));
}

void ScriptEdit::paint_gutter(QPaintEvent* e) {
    QPainter p(gutter_);
    // The gutter from the reference: a shade off the editor's background, with a hairline at its right and numbers
    // in the editor's muted text colour.
    const QColor base = palette().color(QPalette::Base);
    p.fillRect(e->rect(), light_interface() ? base.darker(108) : QColor(58, 58, 58));
    p.setPen(light_interface() ? QColor(150, 150, 150) : QColor(184, 184, 184));
    QTextBlock b = firstVisibleBlock();
    int n = b.blockNumber();
    int top = qRound(blockBoundingGeometry(b).translated(contentOffset()).top());
    while (b.isValid() && top <= e->rect().bottom()) {
        p.drawText(0, top, gutter_->width() - 4, fontMetrics().height(), Qt::AlignRight, QString::number(n + 1));
        b = b.next();
        top += qRound(blockBoundingRect(b).height());
        ++n;
    }
    p.setPen(light_interface() ? QColor(190, 190, 190) : QColor(74, 74, 74));
    p.drawLine(gutter_->width() - 1, e->rect().top(), gutter_->width() - 1, e->rect().bottom());
}

void ScriptEdit::mousePressEvent(QMouseEvent* e) {
    if (e->button() == Qt::MiddleButton && textCursor().hasSelection()) {
        auto* drag = new QDrag(this);
        auto* m = new QMimeData;
        m->setText(textCursor().selectedText().replace(QChar(0x2029), '\n'));
        m->setData("application/x-aztek-script-language", language.toUtf8());
        drag->setMimeData(m);
        drag->exec(Qt::CopyAction);
        return;
    }
    QPlainTextEdit::mousePressEvent(e);
}

ScriptEdit* page_at(const QList<QWidget*>& pages, int i) {
    return i >= 0 && i < pages.size() ? dynamic_cast<ScriptEdit*>(pages[i]) : nullptr;
}

// The tab strip: the reference's + button after the last tab, which adds a tab.
class ScriptTabBar : public QTabBar {
public:
    explicit ScriptTabBar(QWidget* parent = nullptr) : QTabBar(parent) {
        setExpanding(false);
        setDrawBase(false);
        setFocusPolicy(Qt::NoFocus);
        setIconSize(QSize(16, 16));
        setUsesScrollButtons(true);
    }
    std::function<void()> new_tab;
    QSize sizeHint() const override { return QSize(4096, 20); }
    QSize tabSizeHint(int index) const override {
        return QSize(42 + fontMetrics().horizontalAdvance(tabText(index)), 20);
    }
protected:
    QRect plus_rect() const {
        if (count() == 0) return QRect(5, 1, 26, 18);
        QRect last = tabRect(count() - 1);
        if (!last.isValid()) return QRect();
        QRect r(last.right() + 5, last.top() + 1, 26, 18);
        if (r.right() > width() - 3 || !rect().contains(r)) return QRect();
        return r;
    }
    void paintEvent(QPaintEvent* e) override {
        {
            QPainter bg(this);
            bg.fillRect(rect(), QColor(55, 55, 55));
        }
        QTabBar::paintEvent(e);
        QRect r = plus_rect();
        if (!r.isValid()) return;
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing);
        p.setPen(QPen(QColor(90, 90, 90), 1));
        p.setBrush(QColor(63, 63, 63));
        p.drawRoundedRect(QRectF(r).adjusted(0.5, 0.5, -0.5, -0.5), 2, 2);
        p.setPen(QPen(QColor(205, 205, 205), 1.6));
        QPointF c = QRectF(r).center();
        p.drawLine(c - QPointF(5, 0), c + QPointF(5, 0));
        p.drawLine(c - QPointF(0, 5), c + QPointF(0, 5));
    }
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() == Qt::LeftButton && plus_rect().contains(e->pos())) {
            if (new_tab) new_tab();
            return;
        }
        QTabBar::mousePressEvent(e);
    }
};

// The toolbar, laid out left to right at the reference's sizes and clipped when the window is narrow.
class ScriptToolBar : public QWidget {
public:
    explicit ScriptToolBar(QWidget* parent = nullptr) : QWidget(parent) {
        setFixedHeight(26);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
        setMinimumWidth(1);
    }
    QToolButton* button(const QString& icon_name, const QString& tip) {
        auto* b = new QToolButton(this);
        b->setObjectName("scriptToolButton");
        b->setIcon(icon(icon_name));
        b->setIconSize(QSize(16, 16));
        b->setFixedSize(23, 23);
        b->setAutoRaise(true);
        b->setToolTip(tip);
        b->setFocusPolicy(Qt::NoFocus);
        add(b, false);
        return b;
    }
    QLineEdit* field(int w, const QString& tip, const QString& name) {
        auto* e = new QLineEdit(this);
        e->setObjectName(name);
        e->setFixedSize(w, 20);
        e->setToolTip(tip);
        add(e, false);
        return e;
    }
    void separator() {
        auto* f = new QFrame(this);
        f->setFixedSize(1, 23);
        f->setStyleSheet("background:#5c5c5c; border:none;");
        add(f, true);
    }
protected:
    void resizeEvent(QResizeEvent*) override { relayout(); }
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.fillRect(rect(), QColor(68, 68, 68));
    }
private:
    struct Item {
        QWidget* w;
        bool sep;
    };
    void add(QWidget* w, bool sep) {
        items_.append({w, sep});
        relayout();
    }
    void relayout() {
        int x = 5;
        for (const Item& it : items_) {
            if (it.sep) {
                x += 4;
                it.w->move(x, 2);
                x += 5;
            } else {
                it.w->move(x, 2 + (23 - it.w->height()) / 2);
                x += it.w->width() + 4;
            }
        }
    }
    QList<Item> items_;
};

// The reference's menu indicators: an outlined square with a check, and a filled radio dot.
class ScriptMenuStyle : public QProxyStyle {
public:
    ScriptMenuStyle() : QProxyStyle(QApplication::style()) {}
    void drawPrimitive(PrimitiveElement e, const QStyleOption* opt, QPainter* p, const QWidget* w) const override {
        if (e == PE_IndicatorMenuCheckMark || e == PE_IndicatorRadioButton) {
            const auto* mo = static_cast<const QStyleOptionMenuItem*>(opt);
            const bool on = mo && mo->checked;
            p->save();
            p->setRenderHint(QPainter::Antialiasing, true);
            if (e == PE_IndicatorRadioButton) {
                const QRectF r(opt->rect);
                const QPointF c = r.center();
                const qreal rad = qMin(r.width(), r.height()) / 2.0 - 1.0;
                p->setPen(QPen(QColor(120, 120, 120), 1));
                p->setBrush(QColor(58, 58, 58));
                p->drawEllipse(c, rad, rad);
                if (on) {
                    p->setPen(Qt::NoPen);
                    p->setBrush(QColor(240, 240, 240));
                    p->drawEllipse(c, rad - 3, rad - 3);
                }
            } else {
                const QRectF r = QRectF(opt->rect).adjusted(1.5, 1.5, -1.5, -1.5);
                p->setPen(QPen(QColor(120, 120, 120), 1));
                p->setBrush(QColor(58, 58, 58));
                p->drawRect(r);
                if (on) {
                    p->setPen(QPen(QColor(240, 240, 240), 1.8, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
                    p->drawPolyline(QPolygonF({{r.left() + 2.5, r.top() + 6.0}, {r.left() + 5.0, r.top() + 8.5}, {r.left() + 9.5, r.top() + 3.5}}));
                }
            }
            p->restore();
            return;
        }
        QProxyStyle::drawPrimitive(e, opt, p, w);
    }
    void drawControl(ControlElement e, const QStyleOption* opt, QPainter* p, const QWidget* w) const override {
        if (e == CE_MenuTearoff) {
            const QRect r = opt->rect;
            p->fillRect(r, QColor(64, 64, 64));
            p->setPen(QColor(126, 126, 126));
            for (int x = r.left() + 8; x < r.right() - 8; x += 4) p->drawPoint(x, r.center().y());
            p->setPen(QColor(47, 47, 47));
            p->drawLine(r.left(), r.bottom(), r.right(), r.bottom());
            return;
        }
        QProxyStyle::drawControl(e, opt, p, w);
    }
    int pixelMetric(PixelMetric m, const QStyleOption* o = nullptr, const QWidget* w = nullptr) const override {
        if (m == PM_MenuTearoffHeight) return 15;
        if (m == PM_SmallIconSize) return 16;
        if (m == PM_MenuHMargin) return 0;
        if (m == PM_MenuVMargin) return 0;
        return QProxyStyle::pixelMetric(m, o, w);
    }
    QSize sizeFromContents(ContentsType t, const QStyleOption* o, const QSize& s, const QWidget* w) const override {
        QSize z = QProxyStyle::sizeFromContents(t, o, s, w);
        if (t == CT_MenuItem) z.setHeight(21);
        return z;
    }
};

const char* const kScriptMenuStyleTemplate = R"(
QMenu#scriptMenu { background:#525252; color:#e6e6e6; border:1px solid #333333; padding:0; }
QMenu#scriptMenu::item { background:transparent; padding:2px 26px 2px 26px; min-height:17px; }
QMenu#scriptMenu::item:selected { background:#4a6d8c; color:#ffffff; }
QMenu#scriptMenu::item:disabled { color:#8a8a8a; }
QMenu#scriptMenu::separator { height:1px; background:#6f6f6f; margin:0 8px; }
QMenu#scriptMenu::right-arrow { width:11px; height:11px; }
QMenu#scriptMenu::indicator { width:13px; height:13px; }
QMenu#scriptMenu::indicator:non-exclusive:unchecked { image: url(@DIR/check_off.png); }
QMenu#scriptMenu::indicator:non-exclusive:checked { image: url(@DIR/check_on.png); }
QMenu#scriptMenu::indicator:exclusive:unchecked { image: url(@DIR/radio_off.png); }
QMenu#scriptMenu::indicator:exclusive:checked { image: url(@DIR/radio_on.png); }
)";

// The menu indicator pictures: Maya's outlined square with a check, and the filled radio dot. Written once to the
// cache directory and named in the menus' style sheet.
const QString& script_icon_dir() {
    static const QString dir = [] {
        const QString path = QStandardPaths::writableLocation(QStandardPaths::CacheLocation) + "/scripteditor";
        QDir().mkpath(path);
        return path;
    }();
    return dir;
}

QString script_menu_style() {
    static QString style = [] {
        const QString dir = script_icon_dir();
        auto box = [&](const QString& name, bool on) {
            QPixmap pm(13, 13);
            pm.fill(Qt::transparent);
            QPainter p(&pm);
            p.setRenderHint(QPainter::Antialiasing);
            p.setPen(QPen(QColor(122, 122, 122), 1));
            p.setBrush(QColor(58, 58, 58));
            p.drawRect(QRectF(0.5, 0.5, 12, 12));
            if (on) {
                p.setPen(QPen(QColor(240, 240, 240), 1.7, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
                p.drawPolyline(QPolygonF({{3.0, 6.5}, {5.5, 9.0}, {10.0, 4.0}}));
            }
            pm.save(dir + "/" + name + ".png");
        };
        auto dot = [&](const QString& name, bool on) {
            QPixmap pm(13, 13);
            pm.fill(Qt::transparent);
            QPainter p(&pm);
            p.setRenderHint(QPainter::Antialiasing);
            p.setPen(QPen(QColor(122, 122, 122), 1));
            p.setBrush(QColor(58, 58, 58));
            p.drawEllipse(QPointF(6.5, 6.5), 6, 6);
            if (on) {
                p.setPen(Qt::NoPen);
                p.setBrush(QColor(240, 240, 240));
                p.drawEllipse(QPointF(6.5, 6.5), 3, 3);
            }
            pm.save(dir + "/" + name + ".png");
        };
        box("check_off", false);
        box("check_on", true);
        dot("radio_off", false);
        dot("radio_on", true);
        QString q = QString::fromUtf8(kScriptMenuStyleTemplate);
        q.replace("@DIR", dir);
        return q;
    }();
    return style;
}

const char* const kScriptStyleTemplate = R"(
QMenuBar#scriptMenuBar { background:#444444; color:#dcdcdc; border-top:1px solid #808080; }
QMenuBar#scriptMenuBar::item { padding:5px 7px; }
QMenuBar#scriptMenuBar::item:selected { background:#4a8bbd; color:#ffffff; }
QToolButton#scriptToolButton, QToolButton#seEchoButton, QToolButton#seLineNumbersButton { border:none; background:transparent; padding:0; }
QToolButton#scriptToolButton:hover, QToolButton#seEchoButton:hover, QToolButton#seLineNumbersButton:hover { background:#505050; }
QToolButton#scriptToolButton:checked, QToolButton#seEchoButton:checked, QToolButton#seLineNumbersButton:checked { background:#4a8bbd; }
QLineEdit#scriptSearchField, QLineEdit#scriptGotoField { background:#232323; border:1px solid #3a3a3a; border-radius:2px; color:#e0e0e0; padding:0 3px; selection-background-color:#4a8bbd; }
QLineEdit#scriptSearchField:focus, QLineEdit#scriptGotoField:focus { border:1px solid #4a8bbd; }
QSplitter#scriptSplit { background:#444444; }
QSplitter#scriptSplit::handle { background:#444444; border:none; }
QSplitter#scriptSplit::handle:vertical { height:8px; }
QPlainTextEdit#scriptHistory { background:#2f2f2f; color:#d2d2d2; border:none; selection-background-color:#4a6d8c; }
QPlainTextEdit#scriptInput { background:#2b2b2b; color:#d2d2d2; border:none; selection-background-color:#4a6d8c; }
QStackedWidget#scriptStack { background:#2b2b2b; }
QTabBar#scriptTabBar { background:#373737; border:none; }

QTabBar#scriptTabBar::tab { background:#373737; color:#d0d0d0; border:1px solid #5a5a5a; border-bottom:1px solid #8a8a8a; border-top-left-radius:2px; border-top-right-radius:2px; padding:0 2px 0 5px; margin:0 1px 0 0; }
QTabBar#scriptTabBar::tab:selected { background:#3f3f3f; }
QTabBar#scriptTabBar::tab:!selected { color:#c4c4c4; }
QTabBar#scriptTabBar::tab:hover:!selected { background:#3b3b3b; }
QTabBar#scriptTabBar::scroller { width:18px; }
QTabBar#scriptTabBar QToolButton { background:#373737; border:none; }
QToolButton#scriptTabClose { border:none; background:transparent; padding:0; }
QToolButton#scriptTabClose:hover { background:#4a4a4a; border-radius:2px; }
QPlainTextEdit#scriptHistory QScrollBar:vertical, QPlainTextEdit#scriptInput QScrollBar:vertical { background:#444444; width:8px; margin:0; border:none; }
QPlainTextEdit#scriptHistory QScrollBar::handle:vertical, QPlainTextEdit#scriptInput QScrollBar::handle:vertical { background:#5e5e5e; border:1px solid #6f6f6f; min-height:24px; }
QPlainTextEdit#scriptHistory QScrollBar:horizontal, QPlainTextEdit#scriptInput QScrollBar:horizontal { background:#444444; height:8px; margin:0; border:none; }
QPlainTextEdit#scriptHistory QScrollBar::handle:horizontal, QPlainTextEdit#scriptInput QScrollBar::handle:horizontal { background:#5e5e5e; border:1px solid #6f6f6f; min-width:24px; }
QPlainTextEdit#scriptHistory QScrollBar::add-line, QPlainTextEdit#scriptHistory QScrollBar::sub-line, QPlainTextEdit#scriptInput QScrollBar::add-line, QPlainTextEdit#scriptInput QScrollBar::sub-line { background:#4a4a4a; border:none; width:8px; height:8px; }
QPlainTextEdit#scriptHistory QScrollBar::add-page, QPlainTextEdit#scriptHistory QScrollBar::sub-page, QPlainTextEdit#scriptInput QScrollBar::add-page, QPlainTextEdit#scriptInput QScrollBar::sub-page { background:#444444; }
)";

QString script_style() {
    static QString style = [] {
        QString q = QString::fromUtf8(kScriptStyleTemplate);
        q.replace("@DIR", script_icon_dir());
        return q;
    }();
    return style;
}

const char* const kScriptFindStyleTemplate = R"(
QDialog#scriptFind { background:#444444; }
QDialog#scriptFind QLabel { color:#dcdcdc; }
QDialog#scriptFind QLineEdit { background:#232323; border:1px solid #3a3a3a; color:#e6e6e6; padding:0 4px; }
QDialog#scriptFind QLineEdit:focus { border:1px solid #4a8bbd; }
QDialog#scriptFind QPushButton { background:#575757; border:1px solid #6a6a6a; border-radius:3px; color:#e6e6e6; padding:3px 4px; }
QDialog#scriptFind QPushButton:hover { background:#626262; }
QDialog#scriptFind QPushButton:pressed { background:#4a4a4a; }
QDialog#scriptFind QCheckBox { color:#dcdcdc; spacing:6px; }
QDialog#scriptFind QCheckBox::indicator { width:13px; height:13px; }
QDialog#scriptFind QCheckBox::indicator:unchecked { image: url(@DIR/check_off.png); }
QDialog#scriptFind QCheckBox::indicator:checked { image: url(@DIR/check_on.png); }
)";

QString script_find_style() {
    static QString style = [] {
        QString q = QString::fromUtf8(kScriptFindStyleTemplate);
        q.replace("@DIR", script_icon_dir());
        return q;
    }();
    return style;
}

QFont script_font() {
    QFont mono("DejaVu Sans Mono");
    mono.setStyleHint(QFont::Monospace);
    mono.setPointSizeF(9.5);
    return mono;
}

} // namespace

ScriptEditor::ScriptEditor(QWidget* parent) : QWidget(parent) {
    setObjectName("scriptEditor");
    setStyleSheet(script_style());
    auto* lay = new QVBoxLayout(this);
    lay->setContentsMargins(0, 0, 0, 0);
    lay->setSpacing(0);
    QSettings s;
    show_numbers_ = s.value("scriptEditor/lineNumbers", true).toBool();
    stack_trace_ = s.value("scriptEditor/stackTrace", true).toBool();
    echo_all_ = s.value("scriptEditor/echoAll", false).toBool();

    auto* mb = new QMenuBar(this);
    mb->setObjectName("scriptMenuBar");
    mb->setNativeMenuBar(false);
    QFont mbf = mb->font();
    mbf.setPixelSize(10);
    mb->setFont(mbf);
    lay->addWidget(mb);

    // Every menu is the reference's: a tear-off strip, the same background, item height and indicators.
    auto dress = [](QMenu* m) {
        QFont mf = m->font();
        mf.setPixelSize(11);
        m->setFont(mf);
        m->setObjectName("scriptMenu");
        m->setStyleSheet(script_menu_style());
        static ScriptMenuStyle style;
        m->setStyle(&style);
        m->setTearOffEnabled(true);
    };
    auto make_menu = [mb, &dress](const QString& title) {
        QMenu* m = mb->addMenu(title);
        dress(m);
        return m;
    };
    auto make_sub = [&dress](QMenu* parent, const QString& title) {
        QMenu* m = parent->addMenu(title);
        dress(m);
        return m;
    };
    // A ticked setting kept between sessions, applied by `on`.
    auto toggle = [this](QMenu* m, const QString& label, const QString& key, bool fallback, std::function<void(bool)> on = nullptr) {
        QAction* a = m->addAction(label);
        a->setCheckable(true);
        a->setChecked(option(key, fallback));
        connect(a, &QAction::toggled, this, [key, on](bool v) {
            QSettings().setValue("scriptEditor/" + key, v);
            if (on) on(v);
        });
        option_actions_.insert(key, a);
        return a;
    };

    QMenu* file = make_menu("File");
    file->setMinimumWidth(195);
    QAction* open_act = file->addAction("Open Script...", this, [this] {
        QString f = QFileDialog::getOpenFileName(this, "Open Script", QSettings().value("project/current").toString(), "Scripts (*.mel *.py);;All files (*)");
        if (!f.isEmpty()) load_file(f);
    });
    QAction* source_act = file->addAction("Source Script...", [] {
        QString f = QFileDialog::getOpenFileName(nullptr, "Source Script", QSettings().value("project/current").toString(), "Scripts (*.mel *.py)");
        if (f.isEmpty()) return;
        QFile in(f);
        if (!in.open(QIODevice::ReadOnly)) return;
        if (f.endsWith(".py", Qt::CaseInsensitive)) python(QString::fromUtf8(in.readAll()));
        else mel(QString::fromUtf8(in.readAll()));
    });
    QAction* save_act = file->addAction("Save Script...", this, [this] { save_current(true); });
    file->addAction("Save All Script Tabs", this, [this] { save_all_tabs(); });
    file->addSeparator();
    QAction* shelf_act = file->addAction("Save Script to Shelf...", this, [this] { save_to_shelf(); });

    QMenu* edit = make_menu("Edit");
    edit->setMinimumWidth(199);
    edit->addAction("Undo", [this] { if (auto* e = current()) e->undo(); });
    edit->addAction("Redo", [this] { if (auto* e = current()) e->redo(); });
    edit->addSeparator();
    auto pane = [this]() -> QPlainTextEdit* { return history_->hasFocus() ? history_ : current(); };
    edit->addAction("Cut", [this] { if (auto* e = current()) e->cut(); });
    edit->addAction("Copy", [pane] { if (auto* p = pane()) p->copy(); });
    edit->addAction("Paste", [this] { if (auto* e = current()) e->paste(); });
    edit->addAction("Select All", [pane] { if (auto* p = pane()) p->selectAll(); });
    edit->addSeparator();
    edit->addAction("Goto line...", this, [this] { goto_line(); });
    edit->addAction("Search and Replace...", this, [this] { open_search_replace(); });
    edit->addSeparator();
    edit->addAction("Indent Selection", [this] { indent(true); });
    edit->addAction("Unindent Selection", [this] { indent(false); });
    edit->addSeparator();
    edit->addAction("Clear History", [this] { history_->clear(); });
    edit->addAction("Clear Input", [this] { if (auto* e = current()) e->clear(); });
    edit->addAction("Clear All", [this] { history_->clear(); if (auto* e = current()) e->clear(); });

    QMenu* hist = make_menu("History");
    hist->setMinimumWidth(281);
    toggle(hist, "Batch Render Messages", "batchRenderMessages", true);
    toggle(hist, "Echo All Commands", "echoAll", false, [this](bool on) {
        echo_all_ = on;
        exec_quiet("uiPref", QJsonObject{{"args", QJsonArray{"echoAllCommands", on}}});
        if (auto* b = findChild<QToolButton*>("seEchoButton")) {
            QSignalBlocker block(b);
            b->setChecked(on);
        }
    });
    toggle(hist, "Line numbers in errors", "lineNumbersInErrors", true);
    toggle(hist, "Show Stack Trace", "stackTrace", true, [this](bool on) { stack_trace_ = on; });
    hist->addSeparator();
    QMenu* output = make_sub(hist, "History Output");
    output->setMinimumWidth(181);
    auto* outputs = new QActionGroup(output);
    const QStringList shown{"Show Both", "Show MEL Only", "Show Python Only"};
    for (int i = 0; i < shown.size(); ++i) {
        QAction* a = output->addAction(shown[i]);
        a->setCheckable(true);
        a->setChecked(QSettings().value("scriptEditor/historyOutput", 0).toInt() == i);
        outputs->addAction(a);
        connect(a, &QAction::triggered, this, [i] { QSettings().setValue("scriptEditor/historyOutput", i); });
    }
    hist->addSeparator();
    toggle(hist, "Suppress Command Results", "suppressResults", false);
    toggle(hist, "Suppress Info Messages", "suppressInfo", false);
    toggle(hist, "Suppress Warning Messages", "suppressWarnings", false);
    toggle(hist, "Suppress Error Messages", "suppressErrors", false);
    toggle(hist, "Suppress Duplicate Variable Messages", "suppressDuplicateVariables", false);
    toggle(hist, "Suppress Stack Window", "suppressStackWindow", false);

    QMenu* cmd = make_menu("Command");
    cmd->setMinimumWidth(206);
    toggle(cmd, "Show line numbers", "lineNumbers", true, [this](bool on) {
        show_numbers_ = on;
        for (int i = 0; i < tab_bar_->count(); ++i)
            if (ScriptEdit* e = page_at(pages_, i)) e->set_numbers(on);
        if (auto* b = findChild<QToolButton*>("seLineNumbersButton")) {
            QSignalBlocker block(b);
            b->setChecked(on);
        }
    });
    toggle(cmd, "Use tabs for indent", "tabsForIndent", false);
    toggle(cmd, "Show tabs and spaces", "showTabsAndSpaces", false, [this](bool) { apply_editor_options(); });
    toggle(cmd, "Auto-close braces", "autoCloseBraces", true);
    toggle(cmd, "Command Completion", "commandCompletion", false)->setEnabled(false);
    toggle(cmd, "Object Path Completion", "objectPathCompletion", false)->setEnabled(false);
    toggle(cmd, "Show Tooltip Help", "tooltipHelp", false)->setEnabled(false);
    toggle(cmd, "Show quick help", "quickHelp", false)->setEnabled(false);
    cmd->addSeparator();
    cmd->addAction("Execute", [this] { run(false); });

    QMenu* tabs = make_menu("Tabs");
    tabs->setMinimumWidth(195);
    tabs->addAction("New Tab...", [this] { new_tab_prompt(); });
    tabs->addAction("Rename Tab...", [this] { rename_tab(tab_bar_->currentIndex()); });
    tabs->addAction("Go to Previous Tab", [this] { step_tab(-1); });
    tabs->addAction("Go to Next Tab", [this] { step_tab(1); });
    tabs->addAction("Delete Tab", [this] { close_tab(tab_bar_->currentIndex()); });
    tabs->addAction("Delete All Other Tabs", [this] { delete_other_tabs(); });

    QMenu* help = make_menu("Help");
    help->setMinimumWidth(236);
    const QString repo = "https://github.com/srlegrand/Aztek";
    auto link = [help](const QString& label, const QString& url) { help->addAction(label, [url] { QDesktopServices::openUrl(QUrl(url)); }); };
    QAction* search = help->addAction(icon("seMenuSearch"), "Search");
    search->setShortcut(QKeySequence("Ctrl+F"));
    connect(search, &QAction::triggered, this, [this] {
        search_field_->setFocus();
        search_field_->selectAll();
    });
    help->addAction("Find Menu")->setEnabled(false);
    QAction* aztek_help = help->addAction(icon("seMenuHelp"), "Aztek Help");
    aztek_help->setShortcut(QKeySequence("F1"));
    connect(aztek_help, &QAction::triggered, [] { QDesktopServices::openUrl(QUrl("https://srlegrand.github.io/Aztek/")); });
    QMenu* news = make_sub(help, "What's New");
    news->addAction("Release Notes", [] { QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek/releases")); });
    help->addSeparator();
    link("Help on MEL", repo + "/blob/main/docs/COMMANDS.md");
    link("Help on Python", repo + "/blob/main/docs/COMMANDS.md");
    help->addAction("Help on Selected Command", [this] { quick_help(true); });
    link("Help on Script Editor", repo + "/tree/main/docs");
    help->addSeparator();
    link("MEL Command Reference", repo + "/blob/main/docs/COMMANDS.md");
    link("Python Command Reference", repo + "/blob/main/docs/COMMANDS.md");
    help->addAction("Node and Attribute Reference")->setEnabled(false);
    help->addSeparator();
    link("Aztek Home Page", "https://srlegrand.github.io/Aztek/");

    // The toolbar, control for control as scriptEditorPanel.mel builds it.
    auto* bar = new ScriptToolBar(this);
    lay->addWidget(bar);
    auto* open_b = bar->button("seOpenScript", "Open Script...");
    connect(open_b, &QToolButton::clicked, open_act, &QAction::trigger);
    auto* source_b = bar->button("seSourceScript", "Source Script...");
    connect(source_b, &QToolButton::clicked, source_act, &QAction::trigger);
    auto* save_b = bar->button("seSaveScript", "Save Script...");
    connect(save_b, &QToolButton::clicked, save_act, &QAction::trigger);
    auto* shelf_b = bar->button("seSaveToShelf", "Save Script to Shelf...");
    connect(shelf_b, &QToolButton::clicked, shelf_act, &QAction::trigger);
    bar->separator();
    auto* clear_hist_b = bar->button("seClearHistory", "Clear History");
    connect(clear_hist_b, &QToolButton::clicked, this, [this] { history_->clear(); });
    auto* clear_input_b = bar->button("seClearInput", "Clear Input");
    connect(clear_input_b, &QToolButton::clicked, this, [this] { if (auto* e = current()) e->clear(); });
    auto* clear_all_b = bar->button("seClearAll", "Clear All");
    connect(clear_all_b, &QToolButton::clicked, this, [this] { history_->clear(); if (auto* e = current()) e->clear(); });
    bar->separator();
    auto* show_hist_b = bar->button("seShowHistory", "Show History");
    connect(show_hist_b, &QToolButton::clicked, this, [this] { set_pane_mode(0); });
    auto* show_input_b = bar->button("seShowInput", "Show Input");
    connect(show_input_b, &QToolButton::clicked, this, [this] { set_pane_mode(1); });
    auto* show_both_b = bar->button("seShowBoth", "Show Both");
    connect(show_both_b, &QToolButton::clicked, this, [this] { set_pane_mode(2); });
    bar->separator();
    auto* echo_b = bar->button("seEchoCommands", "Echo All Commands");
    echo_b->setObjectName("seEchoButton");
    echo_b->setCheckable(true);
    echo_b->setChecked(echo_all_);
    connect(echo_b, &QToolButton::toggled, this, [this](bool on) {
        if (!option_actions_.contains("echoAll")) return;
        QAction* a = option_actions_.value("echoAll");
        if (a->isChecked() != on) a->setChecked(on);
    });
    auto* numbers_b = bar->button("seShowLineNumbers", "Show line numbers");
    numbers_b->setObjectName("seLineNumbersButton");
    numbers_b->setCheckable(true);
    numbers_b->setChecked(show_numbers_);
    connect(numbers_b, &QToolButton::toggled, this, [this](bool on) {
        if (!option_actions_.contains("lineNumbers")) return;
        QAction* a = option_actions_.value("lineNumbers");
        if (a->isChecked() != on) a->setChecked(on);
    });
    bar->separator();
    auto* exec_all_b = bar->button("seExecuteAll", "ExecuteAll");
    connect(exec_all_b, &QToolButton::clicked, this, [this] { run_all(); });
    auto* exec_b = bar->button("seExecute", "Execute");
    connect(exec_b, &QToolButton::clicked, this, [this] { run(false); });
    bar->separator();
    search_field_ = bar->field(113, "Search field", "scriptSearchField");
    connect(search_field_, &QLineEdit::returnPressed, this, [this] { find(false); });
    auto* search_down_b = bar->button("seSearchDown", "Search down");
    connect(search_down_b, &QToolButton::clicked, this, [this] { find(false); });
    auto* search_up_b = bar->button("seSearchUp", "Search up");
    connect(search_up_b, &QToolButton::clicked, this, [this] { find(true); });
    bar->separator();
    goto_field_ = bar->field(27, "Goto line field", "scriptGotoField");
    auto goto_now = [this] {
        bool ok = false;
        const int line = goto_field_->text().trimmed().toInt(&ok);
        if (ok) goto_line_number(line);
    };
    connect(goto_field_, &QLineEdit::returnPressed, this, goto_now);
    auto* go_b = bar->button("seGotoLine", "Go");
    connect(go_b, &QToolButton::clicked, this, goto_now);

    auto* replace_shortcut = new QShortcut(QKeySequence("Ctrl+H"), this);
    connect(replace_shortcut, &QShortcut::activated, this, [this] { open_search_replace(); });

    split_ = new QSplitter(Qt::Vertical, this);
    split_->setObjectName("scriptSplit");
    split_->setHandleWidth(8);
    split_->setChildrenCollapsible(false);
    history_ = new QPlainTextEdit(split_);
    history_->setMinimumWidth(1);
    history_->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
    history_->setReadOnly(true);
    history_->setObjectName("scriptHistory");
    history_->setMaximumBlockCount(20000);
    history_->setFrameShape(QFrame::NoFrame);
    history_->setLineWrapMode(QPlainTextEdit::NoWrap);
    history_->setWordWrapMode(QTextOption::NoWrap);
    history_->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    history_->setVerticalScrollBarPolicy(Qt::ScrollBarAsNeeded);
    history_->setFont(script_font());
    history_->document()->setDocumentMargin(2);
    history_->setHorizontalScrollBar(new ScriptScrollBar(Qt::Horizontal, history_));
    history_->setVerticalScrollBar(new ScriptScrollBar(Qt::Vertical, history_));
    input_pane_ = new QWidget(split_);
    input_pane_->setObjectName("scriptInputPane");
    input_pane_->setMinimumWidth(1);
    input_pane_->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
    auto* input_layout = new QVBoxLayout(input_pane_);
    input_layout->setContentsMargins(0, 0, 0, 0);
    input_layout->setSpacing(0);
    auto* tb = new ScriptTabBar(input_pane_);
    tab_bar_ = tb;
    tb->setObjectName("scriptTabBar");
    tb->new_tab = [this] { new_tab_prompt(); };
    tb->setExpanding(false);
    tb->setMovable(true);
    tb->setDrawBase(false);
    tb->setFixedHeight(20);
    input_layout->addWidget(tab_bar_);
    stack_ = new QStackedWidget(input_pane_);
    stack_->setObjectName("scriptStack");
    stack_->setMinimumWidth(1);
    input_layout->addWidget(stack_, 1);
    connect(tb, &QTabBar::currentChanged, this, [this](int i) {
        if (i >= 0 && i < pages_.size()) stack_->setCurrentWidget(pages_[i]);
    });
    connect(tb, &QTabBar::tabMoved, this, [this](int from, int to) {
        if (from >= 0 && from < pages_.size() && to >= 0 && to < pages_.size()) pages_.move(from, to);
    });
    tb->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(tb, &QWidget::customContextMenuRequested, this, [this, tb](const QPoint& p) {
        int i = tb->tabAt(p);
        if (i >= 0) tb->setCurrentIndex(i);
        QMenu m;
        m.setObjectName("scriptMenu");
        m.setStyleSheet(script_menu_style());
        static ScriptMenuStyle style;
        m.setStyle(&style);
        m.setTearOffEnabled(true);
        m.addAction("New MEL Tab", [this] { add_tab("mel", "MEL", QString()); });
        m.addAction("New Python Tab", [this] { add_tab("python", "Python", QString()); });
        if (i >= 0) {
            m.addAction("Rename Current Tab", [this, i] { rename_tab(i); });
            m.addAction("Delete Current Tab", [this, i] { close_tab(i); });
            m.addAction("Delete All Other Tabs", [this] { delete_other_tabs(); });
        }
        m.exec(tb->mapToGlobal(p));
    });
    connect(tb, &QTabBar::tabBarDoubleClicked, this, [this](int i) { if (i >= 0) rename_tab(i); });
    split_->addWidget(history_);
    split_->addWidget(input_pane_);
    split_->setStyleSheet("QSplitter::handle { background:#444444; border:none; } QSplitter::handle:vertical { height:8px; }");
    split_->setStretchFactor(0, 1);
    split_->setStretchFactor(1, 1);
    QList<int> sizes{315, 313};
    const QVariantList saved = s.value("scriptEditor/split").toList();
    if (saved.size() == 2) sizes = {saved[0].toInt(), saved[1].toInt()};
    both_sizes_ = sizes;
    split_->setSizes(sizes);
    lay->addWidget(split_, 1);
    lay->setSizeConstraint(QLayout::SetNoConstraint);
    lay->activate();
    setMinimumSize(143, 150);
    history_->installEventFilter(this);
    // Right-click is the reference's marking menu, over the history and over the input.
    history_->viewport()->installEventFilter(this);

    // The tabs left open last time, with their text.
    int n = s.beginReadArray("scriptEditor/tabs");
    for (int i = 0; i < n; ++i) {
        s.setArrayIndex(i);
        add_tab(s.value("language").toString(), s.value("name").toString(), s.value("text").toString(), s.value("path").toString());
    }
    s.endArray();
    if (tb->count() == 0) {
        add_tab("mel", "MEL", QString());
        add_tab("python", "Python", QString());
    }
    tb->setCurrentIndex(0);
    pane_mode_ = qBound(0, s.value("scriptEditor/paneMode", 2).toInt(), 2);
    if (pane_mode_ != 2) set_pane_mode(pane_mode_);
}

ScriptEditor::~ScriptEditor() {
    store_tabs();
}

QPlainTextEdit* ScriptEditor::current() const {
    return qobject_cast<QPlainTextEdit*>(stack_->currentWidget());
}

void ScriptEditor::showEvent(QShowEvent* e) {
    QWidget::showEvent(e);
    if (split_sized_ || !split_) return;
    split_sized_ = true;
    // The reference opens on equal panes; a saved split keeps its proportion.
    const int total = qMax(1, split_->height() - split_->handleWidth());
    int first = total / 2;
    const int saved = both_sizes_.value(0) + both_sizes_.value(1);
    if (saved > 2 && both_sizes_.value(0) > 1) first = qBound(40, total * both_sizes_.value(0) / saved, total - 40);
    split_->setSizes({first, total - first});
    if (auto* e = page_at(pages_, tab_bar_->currentIndex())) e->setFocus();
}

void ScriptEditor::set_pane_mode(int mode) {
    if (pane_mode_ == 2 && mode != 2) both_sizes_ = split_->sizes();
    pane_mode_ = mode;
    const bool both = mode == 2;
    history_->setVisible(mode != 1);
    input_pane_->setVisible(mode != 0);
    split_->setHandleWidth(both ? 8 : 0);
    if (both) {
        const int total = qMax(1, split_->height() - split_->handleWidth());
        const int saved = both_sizes_.value(0) + both_sizes_.value(1);
        int first = total / 2;
        if (saved > 2 && both_sizes_.value(0) > 1) first = qBound(40, total * both_sizes_.value(0) / saved, total - 40);
        split_->setSizes({first, total - first});
    }
    QSettings().setValue("scriptEditor/paneMode", mode);
}

QPlainTextEdit* ScriptEditor::search_pane() const {
    QWidget* f = QApplication::focusWidget();
    while (f) {
        if (f == history_) return history_;
        if (ScriptEdit* e = dynamic_cast<ScriptEdit*>(f)) return e;
        f = f->parentWidget();
    }
    return current();
}

void ScriptEditor::run_all() {
    for (int i = 0; i < pages_.size(); ++i) {
        ScriptEdit* e = page_at(pages_, i);
        if (!e) continue;
        const QString src = e->toPlainText();
        if (src.trimmed().isEmpty()) continue;
        if (e->language == "python") python(src);
        else mel(src);
    }
    store_tabs();
}

void ScriptEditor::load_file(const QString& f) {
    QFile in(f);
    if (!in.open(QIODevice::ReadOnly)) return;
    add_tab(f.endsWith(".py", Qt::CaseInsensitive) ? "python" : "mel", QFileInfo(f).fileName(), QString::fromUtf8(in.readAll()), f);
}

void ScriptEditor::store_tabs() {
    if (!tab_bar_ || !split_) return;
    QSettings s;
    s.beginWriteArray("scriptEditor/tabs");
    for (int i = 0; i < pages_.size(); ++i) {
        ScriptEdit* e = page_at(pages_, i);
        if (!e) continue;
        s.setArrayIndex(i);
        s.setValue("language", e->language);
        s.setValue("name", tab_bar_->tabText(i));
        s.setValue("text", e->toPlainText());
        s.setValue("path", e->path);
    }
    s.endArray();
    const QList<int> sizes = split_->sizes();
    s.setValue("scriptEditor/split", QVariantList{sizes.value(0), sizes.value(1)});
}

void ScriptEditor::add_tab(const QString& language, const QString& name, const QString& text, const QString& path) {
    const QString lang = language == "python" ? "python" : "mel";
    auto* e = new ScriptEdit(lang, stack_);
    e->setFont(script_font());
    e->setTabStopDistance(4 * e->fontMetrics().horizontalAdvance(' '));
    e->installEventFilter(this);
    e->viewport()->installEventFilter(this);
    e->set_numbers(show_numbers_);
    e->setPlainText(text);
    e->path = path;
    pages_.append(e);
    stack_->addWidget(e);
    const int index = tab_bar_->addTab(icon(lang == "python" ? "sePythonTab" : "seMelTab"), name);
    tab_bar_->setCurrentIndex(index);
    auto* close = new QToolButton;
    close->setObjectName("scriptTabClose");
    close->setIcon(icon("seTabClose"));
    close->setIconSize(QSize(9, 9));
    close->setFixedSize(14, 14);
    close->setAutoRaise(true);
    close->setFocusPolicy(Qt::NoFocus);
    close->setCursor(Qt::ArrowCursor);
    connect(close, &QToolButton::clicked, this, [this, e] { close_tab(pages_.indexOf(e)); });
    tab_bar_->setTabButton(index, QTabBar::RightSide, close);
    apply_editor_options();
}

void ScriptEditor::rename_tab(int i) {
    if (i < 0) return;
    QDialog d(this);
    d.setWindowTitle("Rename Tab");
    auto* form = new QVBoxLayout(&d);
    form->setContentsMargins(12, 12, 12, 10);
    form->setSpacing(8);
    form->addWidget(new QLabel("New Tab Name:", &d));
    auto* edit = new QLineEdit(tab_bar_->tabText(i), &d);
    edit->selectAll();
    form->addWidget(edit);
    auto* row = new QHBoxLayout;
    row->addStretch(1);
    auto* rename = new QPushButton("Rename", &d);
    rename->setDefault(true);
    auto* cancel = new QPushButton("Cancel", &d);
    row->addWidget(rename);
    row->addWidget(cancel);
    form->addLayout(row);
    connect(rename, &QPushButton::clicked, &d, &QDialog::accept);
    connect(cancel, &QPushButton::clicked, &d, &QDialog::reject);
    connect(edit, &QLineEdit::returnPressed, &d, &QDialog::accept);
    if (d.exec() != QDialog::Accepted) return;
    const QString n = edit->text().trimmed();
    if (!n.isEmpty()) tab_bar_->setTabText(i, n);
}

void ScriptEditor::close_tab(int i) {
    if (i < 0) return;
    if (tab_bar_->count() < 2) {
        log("// Warning: The script editor must contain at least one executer tab.", 4);
        return;
    }
    ScriptEdit* e = page_at(pages_, i);
    if (e && !e->toPlainText().isEmpty()) {
        const QString message = i == tab_bar_->currentIndex() ? QString("Delete the current tab?")
                                                             : QString("Delete %1 tab?").arg(tab_bar_->tabText(i));
        QMessageBox box(QMessageBox::Question, "Delete Tab", message, QMessageBox::Yes | QMessageBox::No, this);
        box.setDefaultButton(QMessageBox::No);
        if (box.exec() != QMessageBox::Yes) return;
    }
    tab_bar_->removeTab(i);
    QWidget* w = pages_.takeAt(i);
    stack_->removeWidget(w);
    w->deleteLater();
}

void ScriptEditor::save_current(bool as) {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e) return;
    QString f = e->path;
    if (as || f.isEmpty()) {
        QString filter = e->language == "python" ? "Python (*.py)" : "MEL (*.mel)";
        f = QFileDialog::getSaveFileName(this, "Save Script", QSettings().value("project/current").toString(), filter);
        if (f.isEmpty()) return;
    }
    QFile out(f);
    if (!out.open(QIODevice::WriteOnly)) {
        log("// Error: can't write " + f + " //", 2);
        return;
    }
    out.write(e->toPlainText().toUtf8());
    e->path = f;
    tab_bar_->setTabText(tab_bar_->currentIndex(), QFileInfo(f).fileName());
}

void ScriptEditor::clear_history() {
    history_->clear();
}

void ScriptEditor::set_input(const QString& text) {
    if (QPlainTextEdit* e = current()) e->setPlainText(text);
}

void ScriptEditor::append(const QJsonArray& lines) {
    for (const QJsonValue& v : lines) {
        QJsonObject l = v.toObject();
        int k = qBound(0, l["kind"].toInt(), 4);
        QString text = l["text"].toString();
        // Python's traceback lines are errors with file lines; Show Stack Trace off keeps only the last.
        if (!stack_trace_ && k == 2 && (text.startsWith("Traceback") || text.startsWith("  File "))) continue;
        // History > Suppress: results, info (comment lines), warnings and errors.
        if ((k == 1 && option("suppressResults", false)) || (k == 4 && option("suppressWarnings", false)) || (k == 2 && option("suppressErrors", false))
            || (k == 0 && text.startsWith("// ") && option("suppressInfo", false)))
            continue;
        history_->appendHtml(QString("<span style='color:%1; white-space:pre'>%2</span>").arg(history_colour(k), text.toHtmlEscaped()));
        // The kind kept on the line, so a theme change can colour it again.
        history_->document()->lastBlock().setUserState(k);
    }
    history_->verticalScrollBar()->setValue(history_->verticalScrollBar()->maximum());
}

void ScriptEditor::run(bool clear_after) {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e) return;
    QString src = e->textCursor().hasSelection() ? e->textCursor().selectedText().replace(QChar(0x2029), '\n') : e->toPlainText();
    if (e->language == "python")
        python(src);
    else
        mel(src);
    if (clear_after) e->clear();
    store_tabs();
}

void ScriptEditor::open_search_replace() {
    find_target_ = search_pane();
    if (!find_dialog_) {
        find_dialog_ = new QDialog(this);
        find_dialog_->setObjectName("scriptFind");
        find_dialog_->setWindowTitle("Search and Replace");
        find_dialog_->setStyleSheet(script_find_style());
        find_dialog_->setModal(false);
        find_dialog_->setFixedSize(440, 160);
        auto* g = new QGridLayout(find_dialog_);
        g->setContentsMargins(10, 10, 10, 10);
        g->setHorizontalSpacing(8);
        g->setVerticalSpacing(6);
        g->setColumnMinimumWidth(0, 80);
        g->setColumnStretch(1, 1);
        auto* find_label = new QLabel("Find what:", find_dialog_);
        find_label->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
        find_ = new QLineEdit(find_dialog_);
        find_->setObjectName("scriptFindField");
        find_->setFixedHeight(22);
        auto* search_b = new QPushButton("Search", find_dialog_);
        search_b->setFixedSize(78, 26);
        connect(search_b, &QPushButton::clicked, this, [this] { find(false); });
        connect(find_, &QLineEdit::returnPressed, this, [this] { find(false); });
        auto* replace_label = new QLabel("Replace with:", find_dialog_);
        replace_label->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
        replace_ = new QLineEdit(find_dialog_);
        replace_->setObjectName("scriptReplaceField");
        replace_->setFixedHeight(22);
        auto* replace_b = new QPushButton("Replace", find_dialog_);
        replace_b->setFixedSize(78, 26);
        connect(replace_b, &QPushButton::clicked, this, [this] { replace(false); });
        connect(replace_, &QLineEdit::returnPressed, this, [this] { replace(false); });
        auto* options = new QLabel("Options:", find_dialog_);
        options->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
        find_down_ = new QCheckBox("Search Down", find_dialog_);
        find_down_->setChecked(true);
        find_case_ = new QCheckBox("Match Case", find_dialog_);
        find_wrap_ = new QCheckBox("Wrap", find_dialog_);
        find_wrap_->setChecked(true);
        auto* replace_all_b = new QPushButton("Replace All", find_dialog_);
        replace_all_b->setFixedSize(78, 26);
        connect(replace_all_b, &QPushButton::clicked, this, [this] { replace(true); });
        auto* close_b = new QPushButton("Close", find_dialog_);
        close_b->setFixedSize(61, 22);
        connect(close_b, &QPushButton::clicked, find_dialog_, &QDialog::hide);

        g->addWidget(find_label, 0, 0);
        g->addWidget(find_, 0, 1);
        g->addWidget(search_b, 0, 2);
        g->addWidget(replace_label, 1, 0);
        g->addWidget(replace_, 1, 1);
        g->addWidget(replace_b, 1, 2);
        g->addWidget(options, 2, 0, Qt::AlignRight | Qt::AlignVCenter);
        g->addWidget(find_down_, 2, 1);
        g->addWidget(find_case_, 3, 1);
        g->addWidget(find_wrap_, 4, 1);
        g->addWidget(replace_all_b, 2, 2, 3, 1, Qt::AlignTop);
        g->addWidget(close_b, 5, 2, Qt::AlignLeft | Qt::AlignBottom);
        g->setRowMinimumHeight(5, 22);
    }
    find_dialog_->show();
    find_dialog_->raise();
    find_dialog_->activateWindow();
    find_->setFocus();
    find_->selectAll();
}

void ScriptEditor::find(bool back) {
    QDialog* dialog = find_dialog_;
    const bool in_dialog = dialog && dialog->isVisible();
    QPlainTextEdit* target = in_dialog && find_target_ ? find_target_ : search_pane();
    if (!target) return;
    QString needle = in_dialog ? (find_ ? find_->text() : QString()) : (search_field_ && !search_field_->text().isEmpty() ? search_field_->text() : word_at_cursor());
    if (needle.isEmpty()) return;
    QTextDocument::FindFlags flags;
    const bool backward = in_dialog ? !find_down_->isChecked() : back;
    if (backward) flags |= QTextDocument::FindBackward;
    if (in_dialog ? (find_case_ && find_case_->isChecked()) : false) flags |= QTextDocument::FindCaseSensitively;
    const bool wrap = in_dialog ? (find_wrap_ && find_wrap_->isChecked()) : true;
    bool found = target->find(needle, flags);
    if (!found && wrap) {
        QTextCursor c = target->textCursor();
        c.movePosition(backward ? QTextCursor::End : QTextCursor::Start);
        target->setTextCursor(c);
        found = target->find(needle, flags);
    }
    if (!found) QApplication::beep();
}

int ScriptEditor::replace(bool all) {
    if (!find_target_ || find_target_->isReadOnly() || !find_ || find_->text().isEmpty()) return 0;
    QPlainTextEdit* target = find_target_;
    const QString needle = find_->text();
    const Qt::CaseSensitivity cs = (find_case_ && find_case_->isChecked()) ? Qt::CaseSensitive : Qt::CaseInsensitive;
    if (!all) {
        QTextCursor c = target->textCursor();
        if (c.hasSelection() && c.selectedText().compare(needle, cs) == 0) c.insertText(replace_->text());
        find(false);
        return 1;
    }
    QTextDocument::FindFlags flags;
    if (find_case_ && find_case_->isChecked()) flags |= QTextDocument::FindCaseSensitively;
    QTextCursor edit(target->document());
    edit.beginEditBlock();
    int n = 0;
    QTextCursor at = target->document()->find(needle, 0, flags);
    while (!at.isNull()) {
        at.insertText(replace_->text());
        ++n;
        at = target->document()->find(needle, at, flags);
    }
    edit.endEditBlock();
    log(QString("// %1 replaced //").arg(n), 1);
    return n;
}

void ScriptEditor::recolour_history() {
    QTextCursor c(history_->document());
    for (QTextBlock b = history_->document()->begin(); b.isValid(); b = b.next()) {
        c.setPosition(b.position());
        c.setPosition(b.position() + b.length() - 1, QTextCursor::KeepAnchor);
        QTextCharFormat f;
        f.setForeground(QColor(history_colour(qMax(0, b.userState()))));
        c.mergeCharFormat(f);
    }
}

bool ScriptEditor::eventFilter(QObject* o, QEvent* ev) {
    // Right-click on the history or an input: the marking menu.
    if (ev->type() == QEvent::MouseButtonPress && static_cast<QMouseEvent*>(ev)->button() == Qt::RightButton) {
        auto* w = qobject_cast<QWidget*>(o);
        auto* pane = w ? qobject_cast<QPlainTextEdit*>(w->parentWidget()) : nullptr;
        if (pane && (pane == history_ || dynamic_cast<ScriptEdit*>(pane))) {
            marking_menu(pane, static_cast<QMouseEvent*>(ev)->globalPosition().toPoint());
            return true;
        }
    }
    if (ev->type() == QEvent::ContextMenu && o->parent() && (o->parent() == history_ || dynamic_cast<ScriptEdit*>(o->parent()))) return true;
    if (ev->type() == QEvent::KeyPress && dynamic_cast<ScriptEdit*>(o)) {
        auto* e = static_cast<ScriptEdit*>(o);
        auto* k = static_cast<QKeyEvent*>(ev);
        // Tab indents with a tab or four spaces, as Command > Use tabs for indent says.
        if (k->key() == Qt::Key_Tab && k->modifiers() == Qt::NoModifier && !e->textCursor().hasSelection()) {
            e->insertPlainText(option("tabsForIndent", false) ? "\t" : "    ");
            return true;
        }
        // Auto-close braces: the closer goes in after the cursor.
        static const QHash<QString, QString> pairs{{"(", ")"}, {"[", "]"}, {"{", "}"}, {"\"", "\""}};
        if (option("autoCloseBraces", true) && pairs.contains(k->text())) {
            QTextCursor c = e->textCursor();
            c.insertText(k->text() + pairs[k->text()]);
            c.movePosition(QTextCursor::Left);
            e->setTextCursor(c);
            return true;
        }
    }
    if (o == history_ && (ev->type() == QEvent::PaletteChange || ev->type() == QEvent::StyleChange)) recolour_history();
    if (ev->type() == QEvent::KeyPress) {
        auto* k = static_cast<QKeyEvent*>(ev);
        if (k->key() == Qt::Key_F && k->modifiers() == Qt::ControlModifier && !(dynamic_cast<QLineEdit*>(o))) {
            search_field_->setFocus();
            search_field_->selectAll();
            return true;
        }
        if (k->key() == Qt::Key_H && k->modifiers() == Qt::ControlModifier) {
            open_search_replace();
            return true;
        }
        if (k->key() == Qt::Key_F1) {
            QDesktopServices::openUrl(QUrl("https://srlegrand.github.io/Aztek/"));
            return true;
        }
        bool enter = k->key() == Qt::Key_Return || k->key() == Qt::Key_Enter;
        // Ctrl+Enter runs and keeps the text; keypad Enter runs and clears it.
        if (enter && (k->modifiers() & Qt::ControlModifier)) {
            run(false);
            return true;
        }
        if (k->key() == Qt::Key_Enter && (k->modifiers() & Qt::KeypadModifier)) {
            run(true);
            return true;
        }
    }
    return QWidget::eventFilter(o, ev);
}

bool ScriptEditor::option(const QString& key, bool fallback) const {
    return QSettings().value("scriptEditor/" + key, fallback).toBool();
}

// Show tabs and spaces, on every tab.
void ScriptEditor::apply_editor_options() {
    for (int i = 0; i < pages_.size(); ++i) {
        ScriptEdit* e = page_at(pages_, i);
        if (!e) continue;
        QTextOption o = e->document()->defaultTextOption();
        o.setFlags(option("showTabsAndSpaces", false) ? (o.flags() | QTextOption::ShowTabsAndSpaces) : (o.flags() & ~QTextOption::ShowTabsAndSpaces));
        e->document()->setDefaultTextOption(o);
    }
}

// New Tab...: which language, as the reference asks.
void ScriptEditor::new_tab_prompt() {
    QMessageBox box(this);
    box.setWindowTitle("Source Type");
    box.setText("Specify executer source language:");
    box.setIcon(QMessageBox::NoIcon);
    QAbstractButton* mel = box.addButton("MEL", QMessageBox::AcceptRole);
    QAbstractButton* python = box.addButton("Python", QMessageBox::AcceptRole);
    box.addButton("Cancel", QMessageBox::RejectRole);
    box.setDefaultButton(qobject_cast<QPushButton*>(mel));
    box.exec();
    if (box.clickedButton() == mel) add_tab("mel", "MEL", QString());
    else if (box.clickedButton() == python) add_tab("python", "Python", QString());
}

void ScriptEditor::step_tab(int by) {
    const int n = tab_bar_->count();
    if (n > 0) tab_bar_->setCurrentIndex((tab_bar_->currentIndex() + by + n) % n);
}

void ScriptEditor::delete_other_tabs() {
    QWidget* keep = stack_->currentWidget();
    for (int i = tab_bar_->count() - 1; i >= 0; --i)
        if (pages_.value(i) != keep) {
            tab_bar_->removeTab(i);
            QWidget* w = pages_.takeAt(i);
            stack_->removeWidget(w);
            w->deleteLater();
        }
}

// Every tab with a file saved to it; those without one ask where.
void ScriptEditor::save_all_tabs() {
    const int was = tab_bar_->currentIndex();
    for (int i = 0; i < tab_bar_->count(); ++i) {
        tab_bar_->setCurrentIndex(i);
        save_current(false);
    }
    tab_bar_->setCurrentIndex(was);
    store_tabs();
}

// The selected text, or the whole tab, as a button on the current shelf.
void ScriptEditor::save_to_shelf() {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e) return;
    const QString src = e->textCursor().hasSelection() ? e->textCursor().selectedText().replace(QChar(0x2029), '\n') : e->toPlainText();
    if (src.trimmed().isEmpty()) return;
    bool ok = false;
    const QString label = QInputDialog::getText(this, "Save Script to Shelf", "Label:", QLineEdit::Normal, tab_bar_->tabText(tab_bar_->currentIndex()), &ok).trimmed();
    if (!ok || label.isEmpty()) return;
    if (QWidget* main = main_window_widget())
        if (auto* shelf = main->findChild<ShelfBar*>()) shelf->add_to_current(label, "script", src, e->language);
}

void ScriptEditor::goto_line() {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e) return;
    bool ok = false;
    const int line = QInputDialog::getInt(this, "Goto Line Number", "Line Number:", e->textCursor().blockNumber() + 1, 1, e->blockCount(), 1, &ok);
    if (!ok) return;
    goto_line_number(line);
}

void ScriptEditor::goto_line_number(int line) {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e || line < 1 || line > e->blockCount()) return;
    QTextCursor c(e->document()->findBlockByNumber(line - 1));
    e->setTextCursor(c);
    e->setFocus();
    e->centerCursor();
}

// The selected lines one level in or out: a tab or four spaces, as Use tabs for indent says.
void ScriptEditor::indent(bool in) {
    ScriptEdit* e = page_at(pages_, tab_bar_->currentIndex());
    if (!e) return;
    QTextCursor c = e->textCursor();
    QTextBlock first = e->document()->findBlock(c.selectionStart()), last = e->document()->findBlock(c.selectionEnd());
    const QString unit = option("tabsForIndent", false) ? "\t" : "    ";
    QTextCursor edit(e->document());
    edit.beginEditBlock();
    for (QTextBlock b = first; b.isValid(); b = b.next()) {
        QTextCursor at(b);
        if (in) {
            at.insertText(unit);
        } else {
            const QString t = b.text();
            int cut = t.startsWith('\t') ? 1 : 0;
            while (!cut && cut < 4 && cut < t.size() && t[cut] == ' ') ++cut;
            if (t.startsWith('\t')) cut = 1;
            at.movePosition(QTextCursor::Right, QTextCursor::KeepAnchor, cut);
            at.removeSelectedText();
        }
        if (b == last) break;
    }
    edit.endEditBlock();
}

// The word under the cursor, or the selection: a command's name.
QString ScriptEditor::word_at_cursor() const {
    auto* e = current();
    if (!e) return QString();
    QTextCursor c = e->textCursor();
    if (!c.hasSelection()) c.select(QTextCursor::WordUnderCursor);
    return c.selectedText().trimmed();
}

// Quick Help: the command's flags in the history; Command documentation: its page.
void ScriptEditor::quick_help(bool documentation) {
    const QString name = word_at_cursor();
    if (name.isEmpty()) return;
    if (documentation) {
        QDesktopServices::openUrl(QUrl("https://github.com/srlegrand/Aztek/blob/main/docs/COMMANDS.md#" + name.toLower()));
        return;
    }
    mel(QString("help %1;").arg(name));
}

// The reference's marking menus: tabs at the compass points, editing in the list below (#133).
void ScriptEditor::marking_menu(QPlainTextEdit* pane, QPoint global) {
    const bool input = pane != history_;
    QList<MarkItem> items;
    items.append(MarkItem{"New Tab...", 0, [this] { new_tab_prompt(); }});
    items.append(MarkItem{"Go to Next Tab", 2, [this] { step_tab(1); }});
    items.append(MarkItem{"Delete", 3, nullptr, true, false, false, false,
                          {MarkItem{"All Other Tabs", 6, [this] { delete_other_tabs(); }}, MarkItem{"Current Tab", 2, [this] { close_tab(tab_bar_->currentIndex()); }}}});
    items.append(MarkItem{"Rename Tab...", 4, [this] { rename_tab(tab_bar_->currentIndex()); }});
    items.append(MarkItem{"Save All Script Tabs", 5, [this] { save_all_tabs(); }});
    items.append(MarkItem{"Go to Previous Tab", 6, [this] { step_tab(-1); }});
    if (input) {
        items.append(MarkItem{"Execute", -1, [this] { run(false); }});
        items.append(MarkItem{"Quick Help", -1, [this] { quick_help(false); }});
        items.append(MarkItem{"Command documentation", -1, [this] { quick_help(true); }});
        items.append(MarkItem{"Undo", -1, [pane] { pane->undo(); }, true, false, false, true});
        items.append(MarkItem{"Redo", -1, [pane] { pane->redo(); }});
        items.append(MarkItem{"Cut", -1, [pane] { pane->cut(); }, true, false, false, true});
        items.append(MarkItem{"Copy", -1, [pane] { pane->copy(); }});
        items.append(MarkItem{"Paste", -1, [pane] { pane->paste(); }});
        items.append(MarkItem{"Select All", -1, [pane] { pane->selectAll(); }, true, false, false, true});
        items.append(MarkItem{"Clear Input", -1, [pane] { pane->clear(); }, true, false, false, true});
    } else {
        items.append(MarkItem{"Copy", -1, [pane] { pane->copy(); }});
        items.append(MarkItem{"Select All", -1, [pane] { pane->selectAll(); }});
        items.append(MarkItem{"Clear History", -1, [this] { history_->clear(); }, true, false, false, true});
    }
    items.append(MarkItem{"Clear All", -1, [this] { history_->clear(); if (auto* e = current()) e->clear(); }});
    MarkingMenu::show_at(global, items);
}

} // namespace az

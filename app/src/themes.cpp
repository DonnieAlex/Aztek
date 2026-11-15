#include "themes.h"

#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QFile>
#include <QFont>
#include <QMenu>
#include <QPalette>
#include <QRegularExpression>
#include <QSettings>
#include <QStyleFactory>
#include <QWidget>

namespace az {

namespace {

Theme make(const QString& id, const QString& name, bool dark, std::initializer_list<std::pair<const char*, const char*>> colours, QStringList font = {},
           int pt = 0, const QString& buttons = "rounded", int radius = 5) {
    Theme t;
    t.id = id;
    t.name = name;
    t.dark = dark;
    for (const auto& [k, v] : colours) t.c.insert(k, QColor(v));
    t.font = font;
    t.point_size = pt;
    t.buttons = buttons;
    t.radius = radius;
    return t;
}

const QStringList kSans = {"Inter", "Segoe UI Variable Text", "Segoe UI", "SF Pro Text", "Helvetica Neue", "Noto Sans", "Cantarell", "Ubuntu", "DejaVu Sans", "Arial"};

// Colour schemes as their authors publish them (Nord, Dracula, Catppuccin, One Dark and Solarized are all MIT), mapped
// onto the interface's parts.
QList<Theme> build() {
    QList<Theme> l;
    l << make("classic", "Aztek Classic", true,
              {{"window", "#444444"}, {"chrome", "#3a3a3a"}, {"base", "#2b2b2b"}, {"alt", "#393939"}, {"menu", "#525252"}, {"button", "#5d5d5d"}, {"hover", "#505050"},
               {"hi", "#5285a6"}, {"text", "#c8c8c8"}, {"bright", "#dddddd"}, {"dim", "#a8a8a8"}, {"disabled", "#7a7a7a"}, {"border", "#222222"}, {"handle", "#5e5e5e"}},
              {}, 0, "classic", 0);
    l << make("graphite", "Graphite", true,
              {{"window", "#2b2d31"}, {"chrome", "#232428"}, {"base", "#1e1f22"}, {"alt", "#26272b"}, {"menu", "#313338"}, {"button", "#3b3d44"}, {"hover", "#45474f"},
               {"hi", "#4f7bd9"}, {"text", "#dcdde1"}, {"bright", "#f2f3f5"}, {"dim", "#9a9ca3"}, {"disabled", "#6b6d74"}, {"border", "#18191c"}, {"handle", "#4a4c53"}},
              kSans, 9);
    l << make("midnight", "Midnight", true,
              {{"window", "#1b2130"}, {"chrome", "#161b27"}, {"base", "#11151f"}, {"alt", "#1a2030"}, {"menu", "#222a3c"}, {"button", "#2a3348"}, {"hover", "#33405a"},
               {"hi", "#3d7eff"}, {"text", "#cfd8e8"}, {"bright", "#eef3fb"}, {"dim", "#8592aa"}, {"disabled", "#5b6680"}, {"border", "#0c0f16"}, {"handle", "#3a4660"}},
              kSans, 9);
    l << make("nord", "Nord", true,
              {{"window", "#3b4252"}, {"chrome", "#2e3440"}, {"base", "#2e3440"}, {"alt", "#353c4a"}, {"menu", "#434c5e"}, {"button", "#4c566a"}, {"hover", "#56607a"},
               {"hi", "#5e81ac"}, {"text", "#d8dee9"}, {"bright", "#eceff4"}, {"dim", "#a3adbf"}, {"disabled", "#6c778c"}, {"border", "#272c36"}, {"handle", "#5b667c"}},
              kSans, 9);
    l << make("dracula", "Dracula", true,
              {{"window", "#282a36"}, {"chrome", "#21222c"}, {"base", "#1e1f29"}, {"alt", "#2b2d3a"}, {"menu", "#343746"}, {"button", "#44475a"}, {"hover", "#4f5368"},
               {"hi", "#bd93f9"}, {"text", "#f8f8f2"}, {"bright", "#ffffff"}, {"dim", "#a0a5c4"}, {"disabled", "#6272a4"}, {"border", "#191a21"}, {"handle", "#5a5e78"}},
              kSans, 9);
    l << make("mocha", "Catppuccin Mocha", true,
              {{"window", "#1e1e2e"}, {"chrome", "#181825"}, {"base", "#11111b"}, {"alt", "#24243a"}, {"menu", "#313244"}, {"button", "#45475a"}, {"hover", "#585b70"},
               {"hi", "#89b4fa"}, {"text", "#cdd6f4"}, {"bright", "#ffffff"}, {"dim", "#a6adc8"}, {"disabled", "#6c7086"}, {"border", "#0b0b12"}, {"handle", "#585b70"}},
              kSans, 9);
    l << make("onedark", "One Dark", true,
              {{"window", "#282c34"}, {"chrome", "#21252b"}, {"base", "#1e2227"}, {"alt", "#2c313a"}, {"menu", "#2f343e"}, {"button", "#3e4451"}, {"hover", "#4b5263"},
               {"hi", "#61afef"}, {"text", "#abb2bf"}, {"bright", "#e6e6e6"}, {"dim", "#8b929e"}, {"disabled", "#5c6370"}, {"border", "#181a1f"}, {"handle", "#4b5263"}},
              kSans, 9);
    l << make("solarized-dark", "Solarized Dark", true,
              {{"window", "#073642"}, {"chrome", "#002b36"}, {"base", "#002b36"}, {"alt", "#06313c"}, {"menu", "#0b4251"}, {"button", "#114f5f"}, {"hover", "#175b6d"},
               {"hi", "#268bd2"}, {"text", "#93a1a1"}, {"bright", "#eee8d5"}, {"dim", "#839496"}, {"disabled", "#586e75"}, {"border", "#00212b"}, {"handle", "#2a6474"}},
              kSans, 9);
    // Near-black and sleek, after the modern game engine editors: flat fills, thin borders, a bright blue.
    l << make("obsidian", "Obsidian", true,
              {{"window", "#151515"}, {"chrome", "#0f0f0f"}, {"base", "#0a0a0a"}, {"alt", "#121212"}, {"menu", "#1a1a1a"}, {"button", "#242424"}, {"hover", "#2f2f2f"},
               {"hi", "#0070e0"}, {"text", "#c0c0c0"}, {"bright", "#ffffff"}, {"dim", "#808080"}, {"disabled", "#555555"}, {"border", "#050505"}, {"handle", "#3a3a3a"}},
              {"Roboto", "Inter", "Segoe UI", "Noto Sans", "DejaVu Sans", "Arial"}, 9, "rounded", 4);
    // The Motif desktops of the early 1990s workstations: blue-grey, bevelled buttons, bold Helvetica.
    l << make("motif", "Motif", false,
              {{"window", "#a8b3c4"}, {"chrome", "#9aa6b8"}, {"base", "#c4cdda"}, {"alt", "#bac4d2"}, {"menu", "#a8b3c4"}, {"button", "#a8b3c4"}, {"hover", "#b6c0cf"},
               {"hi", "#4c6187"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#2e3440"}, {"disabled", "#6f7a8c"}, {"border", "#5c6678"}, {"handle", "#a8b3c4"}},
              {"Helvetica", "Nimbus Sans", "Nimbus Sans L", "Liberation Sans", "Arial"}, 9, "bevel", 0);
    // The desktop of 1998: silver, white fields, navy selection, raised and sunken edges.
    l << make("classic98", "Classic 98", false,
              {{"window", "#c0c0c0"}, {"chrome", "#c0c0c0"}, {"base", "#ffffff"}, {"alt", "#f0f0f0"}, {"menu", "#c0c0c0"}, {"button", "#c0c0c0"}, {"hover", "#c0c0c0"},
               {"hi", "#000080"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#404040"}, {"disabled", "#808080"}, {"border", "#808080"}, {"handle", "#c0c0c0"}},
              {"Tahoma", "Microsoft Sans Serif", "MS Sans Serif", "Liberation Sans", "Arial"}, 8, "bevel", 0);
    l << make("paper", "Paper Light", false,
              {{"window", "#f3f3f3"}, {"chrome", "#e8e8e8"}, {"base", "#ffffff"}, {"alt", "#f6f6f6"}, {"menu", "#ffffff"}, {"button", "#e6e6e9"}, {"hover", "#d9d9de"},
               {"hi", "#2f6fd8"}, {"text", "#1f1f1f"}, {"bright", "#000000"}, {"dim", "#5f5f5f"}, {"disabled", "#a0a0a0"}, {"border", "#c6c6ca"}, {"handle", "#c0c0c6"}},
              kSans, 9);
    l << make("latte", "Catppuccin Latte", false,
              {{"window", "#eff1f5"}, {"chrome", "#e6e9ef"}, {"base", "#ffffff"}, {"alt", "#f4f5f8"}, {"menu", "#ffffff"}, {"button", "#dce0e8"}, {"hover", "#ccd0da"},
               {"hi", "#1e66f5"}, {"text", "#4c4f69"}, {"bright", "#11111b"}, {"dim", "#6c6f85"}, {"disabled", "#9ca0b0"}, {"border", "#bcc0cc"}, {"handle", "#bcc0cc"}},
              kSans, 9);
    l << make("snow", "Nord Snow", false,
              {{"window", "#eceff4"}, {"chrome", "#e5e9f0"}, {"base", "#ffffff"}, {"alt", "#f2f4f8"}, {"menu", "#ffffff"}, {"button", "#d8dee9"}, {"hover", "#c9d1de"},
               {"hi", "#5e81ac"}, {"text", "#2e3440"}, {"bright", "#000000"}, {"dim", "#4c566a"}, {"disabled", "#9aa3b3"}, {"border", "#c2cad8"}, {"handle", "#b8c2d2"}},
              kSans, 9);
    l << make("solarized-light", "Solarized Light", false,
              {{"window", "#eee8d5"}, {"chrome", "#e6dfc9"}, {"base", "#fdf6e3"}, {"alt", "#f7f0dc"}, {"menu", "#fdf6e3"}, {"button", "#e2dbc4"}, {"hover", "#d6ceb5"},
               {"hi", "#268bd2"}, {"text", "#586e75"}, {"bright", "#073642"}, {"dim", "#657b83"}, {"disabled", "#93a1a1"}, {"border", "#d3cbb4"}, {"handle", "#cbc3ab"}},
              kSans, 9);
    return l;
}

// What a theme adds over the classic stylesheet: its buttons and fields.
QString skin(const Theme& t) {
    if (t.buttons == "classic") return QString();
    if (t.buttons == "bevel") {
        // Raised and sunken edges, square corners: the look of the old toolkits.
        QString s;
        s += "QPushButton { background: @button; color: @text; border: 2px outset @button; padding: 3px 12px; }\n";
        s += "QPushButton:pressed, QPushButton:checked { border: 2px inset @button; }\n";
        s += "QPushButton:disabled { color: @disabled; }\n";
        s += "QToolButton { border: 1px solid transparent; }\n";
        s += "QToolButton:hover { border: 1px outset @button; }\n";
        s += "QToolButton:checked, QToolButton:pressed { border: 1px inset @button; background: @alt; }\n";
        s += "QComboBox { background: @button; color: @text; border: 2px outset @button; padding: 1px 6px; }\n";
        s += "QComboBox QAbstractItemView { background: @base; color: @text; selection-background-color: @hi; selection-color: #ffffff; border: 1px solid @border; }\n";
        s += "QLineEdit, QSpinBox, QDoubleSpinBox, QPlainTextEdit, QTextEdit, QListView, QTreeView, QTableView { border: 2px inset @button; background: @base; color: @text; }\n";
        s += "QLineEdit { selection-background-color: @hi; selection-color: #ffffff; }\n";
        s += "QMenuBar::item:selected { background: @hi; color: #ffffff; }\n";
        s += "QMenu { background: @menu; color: @text; border: 2px outset @menu; }\n";
        s += "QMenu::item:selected { background: @hi; color: #ffffff; }\n";
        s += "QTabBar::tab { background: @button; color: @text; border: 2px outset @button; border-bottom: none; padding: 3px 10px; }\n";
        s += "QTabBar::tab:selected { background: @window; margin-bottom: -2px; }\n";
        s += "QDockWidget { color: #ffffff; }\n";
        s += "QDockWidget::title { background: @hi; padding: 3px 6px; }\n";
        s += "QScrollBar:vertical { background: @alt; width: 16px; } QScrollBar:horizontal { background: @alt; height: 16px; }\n";
        s += "QScrollBar::handle:vertical, QScrollBar::handle:horizontal { background: @button; border: 2px outset @button; }\n";
        s += "QScrollBar::add-line, QScrollBar::sub-line { background: @button; border: 2px outset @button; width: 14px; height: 14px; }\n";
        s += "QCheckBox::indicator, QRadioButton::indicator { background: @base; border: 2px inset @button; width: 10px; height: 10px; }\n";
        s += "QCheckBox::indicator:checked { background: @text; border: 2px inset @button; }\n";
        s += "QRadioButton::indicator:checked { background: @text; }\n";
        s += "QToolTip { background: #ffffe1; color: #000000; border: 1px solid #000000; }\n";
        s += "QToolButton#hotboxButton { border: 2px outset @button; background: @button; color: @text; }\n";
        s += "QToolButton#hotboxButton:hover { background: @hi; color: #ffffff; }\n";
        s += "QLabel#hotboxCenter { border: 2px inset @button; background: @window; color: @text; }\n";
        return s;
    }
    const QString r = QString::number(t.radius) + "px";
    const QString r2 = QString::number(qMax(0, t.radius - 2)) + "px";
    QString s;
    s += "QPushButton { background: @button; border: 1px solid @border; border-radius: " + r + "; padding: 4px 12px; color: @text; }\n";
    s += "QPushButton:hover { background: @hover; }\n";
    s += "QPushButton:pressed, QPushButton:checked { background: @hi; color: #ffffff; }\n";
    s += "QPushButton:disabled { color: @disabled; }\n";
    s += "QToolButton { border-radius: " + r2 + "; }\n";
    s += "QToolButton:hover { background: @hover; }\n";
    s += "QToolButton:checked, QToolButton:pressed { background: @hi; }\n";
    s += "QComboBox { background: @button; border: 1px solid @border; border-radius: " + r + "; padding: 2px 8px; color: @text; }\n";
    s += "QComboBox:hover { background: @hover; }\n";
    s += "QComboBox QAbstractItemView { background: @menu; selection-background-color: @hi; border: 1px solid @border; }\n";
    s += "QLineEdit, QSpinBox, QDoubleSpinBox { border-radius: " + r2 + "; border: 1px solid @border; background: @base; }\n";
    s += "QLineEdit:focus, QSpinBox:focus, QDoubleSpinBox:focus { border: 1px solid @hi; }\n";
    s += "QMenu { border-radius: " + r + "; padding: 4px; }\n";
    s += "QMenu::item { border-radius: " + r2 + "; }\n";
    s += "QMenuBar::item:selected { border-radius: " + r2 + "; }\n";
    s += "QTabBar::tab { border-top-left-radius: " + r2 + "; border-top-right-radius: " + r2 + "; }\n";
    s += "QScrollBar:vertical { background: transparent; width: 10px; } QScrollBar:horizontal { background: transparent; height: 10px; }\n";
    s += "QScrollBar::handle:vertical, QScrollBar::handle:horizontal { border-radius: 4px; margin: 1px; }\n";
    s += "QScrollBar::add-line, QScrollBar::sub-line { width: 0; height: 0; }\n";
    s += "QToolTip { background: @menu; color: @bright; border: 1px solid @border; border-radius: " + r2 + "; padding: 3px 6px; }\n";
    s += "QToolButton#hotboxButton, QLabel#hotboxCenter { border-radius: " + r2 + "; }\n";
    return s;
}

QString fill(QString qss, const Theme& t) {
    // Longest names first, so @hi doesn't eat @hover's start and the like.
    QStringList keys = t.c.keys();
    std::sort(keys.begin(), keys.end(), [](const QString& a, const QString& b) { return a.size() > b.size(); });
    for (const QString& k : keys) qss.replace("@" + k, t.c[k].name());
    return qss;
}

} // namespace

const QList<Theme>& themes() {
    static const QList<Theme> l = build();
    return l;
}

QString current_theme() {
    return QSettings().value("prefs/theme", "classic").toString();
}

void apply_theme(QApplication& app, const QString& id) {
    const Theme* t = &themes().front();
    for (const Theme& x : themes())
        if (x.id == id) t = &x;
    QSettings().setValue("prefs/theme", t->id);
    app.setStyle(QStyleFactory::create("Fusion"));
    const auto& c = t->c;
    QPalette p;
    p.setColor(QPalette::Window, c["window"]);
    p.setColor(QPalette::WindowText, c["text"]);
    p.setColor(QPalette::Base, c["base"]);
    p.setColor(QPalette::AlternateBase, c["alt"]);
    p.setColor(QPalette::ToolTipBase, t->buttons == "classic" ? QColor(255, 255, 220) : c["menu"]);
    p.setColor(QPalette::ToolTipText, t->buttons == "classic" ? QColor(Qt::black) : c["bright"]);
    p.setColor(QPalette::Text, c["text"]);
    p.setColor(QPalette::Button, c["button"]);
    p.setColor(QPalette::ButtonText, c["text"]);
    p.setColor(QPalette::BrightText, t->dark ? QColor(Qt::white) : QColor(Qt::black));
    p.setColor(QPalette::Highlight, c["hi"]);
    p.setColor(QPalette::HighlightedText, Qt::white);
    p.setColor(QPalette::Link, c["hi"].lighter(t->dark ? 130 : 100));
    p.setColor(QPalette::PlaceholderText, c["disabled"]);
    p.setColor(QPalette::Light, c["hover"].lighter(120));
    p.setColor(QPalette::Midlight, c["hover"]);
    p.setColor(QPalette::Mid, c["handle"]);
    p.setColor(QPalette::Dark, c["chrome"].darker(130));
    p.setColor(QPalette::Shadow, c["border"]);
    for (auto role : {QPalette::Text, QPalette::WindowText, QPalette::ButtonText}) p.setColor(QPalette::Disabled, role, c["disabled"]);
    app.setPalette(p);
    QFont f = QApplication::font();
    static const QFont system = f;
    if (t->font.isEmpty()) {
        f = system;
    } else {
        f.setFamilies(t->font);
        if (t->point_size > 0) f.setPointSize(t->point_size);
    }
    app.setFont(f);
    QFile qss(":/style.qss");
    QString base = qss.open(QIODevice::ReadOnly) ? QString::fromUtf8(qss.readAll()) : QString();
    app.setStyleSheet(fill(base + "\n" + skin(*t), *t));
    // Widgets that drew with the old palette draw again.
    for (QWidget* w : QApplication::allWidgets()) w->update();
}

void apply_saved_theme(QApplication& app) { apply_theme(app, current_theme()); }

void fill_theme_menu(QMenu* m) {
    m->clear();
    auto* group = new QActionGroup(m);
    const QString now = current_theme();
    for (bool dark : {true, false}) {
        m->addSection(dark ? "Dark" : "Light");
        for (const Theme& t : themes()) {
            if (t.dark != dark) continue;
            QAction* a = m->addAction(t.name);
            a->setCheckable(true);
            a->setChecked(t.id == now);
            group->addAction(a);
            const QString id = t.id;
            QObject::connect(a, &QAction::triggered, m, [id] { apply_theme(*qApp, id); });
        }
    }
}

} // namespace az

#include "themes.h"
#include "aztek.h"
#include "host.h"

#include <QApplication>
#include <QComboBox>
#include <QCryptographicHash>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QFont>
#include <QFontDatabase>
#include <QRandomGenerator>
#include <QImage>
#include <QJsonArray>
#include <QJsonDocument>
#include <QPainter>
#include <QPainterPath>
#include <QPalette>
#include <QSettings>
#include <QStandardPaths>
#include <QStandardItemModel>
#include <QStyleFactory>
#include <QWidget>

#include <algorithm>
#include <array>

namespace az {

namespace {

std::vector<std::function<void()>>& listeners() {
    static std::vector<std::function<void()>> l;
    return l;
}

void changed() {
    for (auto& f : listeners()) f();
}

const QStringList kSans = {"Inter", "Segoe UI Variable Text", "Segoe UI", "SF Pro Text", "Helvetica Neue", "Noto Sans", "Cantarell", "Ubuntu", "DejaVu Sans", "Arial"};
const QStringList kHelvetica = {"Helvetica", "Nimbus Sans", "Nimbus Sans L", "TeX Gyre Heros", "Liberation Sans", "Arial"};

QColor hsl(double h, double s, double l) { return QColor::fromHslF(std::fmod(h + 360.0, 360.0) / 360.0, std::clamp(s, 0.0, 1.0), std::clamp(l / 100.0, 0.0, 1.0)); }

// A modern theme from a hue, its saturation and the window's lightness (0 to 100): every other part a fixed step from
// the window, so each theme has the same range of tones whatever its brightness.
Theme tonal(const QString& id, const QString& name, double hue, double sat, double l, const char* accent) {
    Theme t;
    t.id = id;
    t.name = name;
    auto& c = t.c;
    const double ts = sat * 0.35;
    if (l < 50) {
        c["base"] = hsl(hue, sat, std::max(2.0, l - 4.5));
        c["chrome"] = hsl(hue, sat, std::max(3.0, l - 2.5));
        c["window"] = hsl(hue, sat, l);
        c["alt"] = hsl(hue, sat, l + 1.8);
        c["menu"] = hsl(hue, sat, l + 3.5);
        c["button"] = hsl(hue, sat, l + 7.5);
        c["hover"] = hsl(hue, sat, l + 12);
        c["handle"] = hsl(hue, sat, l + 15);
        c["border"] = hsl(hue, sat, std::max(0.0, l - 6.5));
        c["light"] = hsl(hue, sat, l + 18);
        c["shadow"] = hsl(hue, sat, std::max(0.0, l - 8));
        c["text"] = hsl(hue, ts, 84);
        c["bright"] = hsl(hue, ts, 97);
        c["dim"] = hsl(hue, ts, 62);
        c["disabled"] = hsl(hue, ts, std::max(38.0, l + 20));
        c["field"] = c["base"];
        c["fieldtext"] = c["text"];
    } else {
        c["base"] = hsl(hue, sat, std::min(100.0, l + 7));
        c["chrome"] = hsl(hue, sat, l - 4);
        c["window"] = hsl(hue, sat, l);
        c["alt"] = hsl(hue, sat, std::min(99.0, l + 4));
        c["menu"] = hsl(hue, sat, std::min(100.0, l + 6));
        c["button"] = hsl(hue, sat, std::min(100.0, l + 4));
        c["hover"] = hsl(hue, sat, l - 6);
        c["handle"] = hsl(hue, sat, l - 18);
        c["border"] = hsl(hue, sat, l - 20);
        c["light"] = hsl(hue, sat, 100);
        c["shadow"] = hsl(hue, sat, l - 35);
        c["text"] = hsl(hue, ts, 13);
        c["bright"] = hsl(hue, ts, 3);
        c["dim"] = hsl(hue, ts, 38);
        c["disabled"] = hsl(hue, ts, std::min(70.0, l - 28));
        c["field"] = c["base"];
        c["fieldtext"] = c["text"];
    }
    c["hi"] = QColor(accent);
    c["hitext"] = QColor("#ffffff");
    c["section"] = c["button"];
    c["sectiontext"] = c["bright"];
    t.font = kSans;
    t.point_size = 9;
    return t;
}

Theme hand(const QString& id, const QString& name, std::initializer_list<std::pair<const char*, const char*>> colours) {
    Theme t;
    t.id = id;
    t.name = name;
    for (const auto& [k, v] : colours) t.c.insert(k, QColor(v));
    if (!t.c.contains("hitext")) t.c["hitext"] = QColor("#ffffff");
    if (!t.c.contains("field")) t.c["field"] = t.c["base"];
    if (!t.c.contains("fieldtext")) t.c["fieldtext"] = t.c["text"];
    if (!t.c.contains("light")) t.c["light"] = t.c["hover"].lighter(125);
    if (!t.c.contains("shadow")) t.c["shadow"] = t.c["border"];
    if (!t.c.contains("section")) t.c["section"] = t.c["button"];
    if (!t.c.contains("sectiontext")) t.c["sectiontext"] = t.c["bright"];
    return t;
}

QList<Theme> builtins() {
    QList<Theme> l;
    // The reference's own look.
    {
        Theme t = hand("classic", "Classic",
                       {{"window", "#444444"}, {"chrome", "#3a3a3a"}, {"base", "#2b2b2b"}, {"alt", "#393939"}, {"menu", "#525252"}, {"button", "#5d5d5d"}, {"hover", "#505050"},
                        {"hi", "#5285a6"}, {"text", "#c8c8c8"}, {"bright", "#dddddd"}, {"dim", "#a8a8a8"}, {"disabled", "#7a7a7a"}, {"border", "#222222"}, {"handle", "#5e5e5e"},
                        {"light", "#6a6a6a"}, {"shadow", "#1e1e1e"}});
        t.fill = "classic";
        t.radius = 0;
        t.tabs = "boxed";
        t.arrows = "triangle";
        t.scrollbar = "standard";
        l << t;
    }
    // Brinsmead: pure black and electric cyan, monospaced type, square corners, cyan section bars.
    {
        Theme t = tonal("obsidian", "Brinsmead", 190, 0.0, 3, "#00e5ff");
        t.font = QStringList{"JetBrains Mono", "Cascadia Mono", "SF Mono", "Menlo", "Consolas", "DejaVu Sans Mono", "Liberation Mono", "Courier New"};
        t.point_size = 9;
        t.fill = "flat";
        t.bevel = "none";
        t.radius = 0;
        t.border = 1;
        t.tabs = "underline";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 3;
        t.c["section"] = QColor("#00e5ff");
        t.c["sectiontext"] = QColor("#000000");
        t.c["hitext"] = QColor("#000000");
        t.c["border"] = QColor("#1f2a2e");
        l << t;
    }
    // Flav: the first Carbon, blue-grey charcoal with teal, round corners and pill tabs.
    {
        Theme t = tonal("carbon", "Flav", 200, 0.04, 13, "#2fa59a");
        t.radius = 8;
        t.tabs = "pill";
        t.padding = 4;
        l << t;
    }
    // Kurtenbach: charcoal with tangerine, a geometric typeface, bold menus, borderless pills with big round corners.
    {
        Theme t = tonal("tangerine", "Kurtenbach", 20, 0.06, 13, "#ff7a1a");
        t.font = QStringList{"Montserrat", "Poppins", "Futura", "Century Gothic", "Avenir Next", "Avenir", "URW Gothic", "Gill Sans", "Verdana"};
        t.point_size = 9;
        t.menu_bold = true;
        t.fill = "gradient";
        t.radius = 11;
        t.border = 0;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 5;
        t.c["hitext"] = QColor("#1a0d00");
        l << t;
    }
    // Kovacs: deep navy and gold, serif type with italic menus, round-topped tabs, gold-lettered section bars.
    {
        Theme t = tonal("midnight", "Kovacs", 226, 0.5, 13, "#f2c14e");
        t.font = QStringList{"Georgia", "Charter", "Iowan Old Style", "Palatino Linotype", "Palatino", "Noto Serif", "DejaVu Serif", "Liberation Serif", "Times New Roman"};
        t.point_size = 10;
        t.menu_italic = true;
        t.fill = "flat";
        t.radius = 4;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "standard";
        t.padding = 3;
        t.c["hitext"] = QColor("#141a33");
        t.c["section"] = QColor("#1d2a5c");
        t.c["sectiontext"] = QColor("#f2c14e");
        l << t;
    }
    // Shaq: the game engine editor of the 2020s in its dark scheme: near-black recessed panels, charcoal panels and
    // headers, black entry fields, its bright blue for what is picked, Roboto, small round corners.
    {
        Theme t = hand("shaq", "Shaq",
                       {{"window", "#242424"}, {"chrome", "#151515"}, {"base", "#0f0f0f"}, {"alt", "#1a1a1a"}, {"menu", "#1a1a1a"}, {"button", "#383838"}, {"hover", "#575757"},
                        {"hi", "#0070e0"}, {"hitext", "#ffffff"}, {"text", "#c0c0c0"}, {"bright", "#ffffff"}, {"dim", "#808080"}, {"disabled", "#585858"}, {"border", "#0f0f0f"},
                        {"handle", "#383838"}, {"section", "#2f2f2f"}, {"sectiontext", "#ffffff"}, {"light", "#444444"}, {"shadow", "#0a0a0a"},
                        {"field", "#0f0f0f"}, {"fieldtext", "#c0c0c0"}});
        t.font = QStringList{"Roboto", "Inter", "Segoe UI", "Noto Sans", "DejaVu Sans", "Arial"};
        t.point_size = 9;
        t.fill = "flat";
        t.radius = 4;
        t.border = 1;
        t.tabs = "boxed";
        t.arrows = "triangle";
        t.scrollbar = "thin";
        t.padding = 4;
        l << t;
    }
    // Nicolas: near-black with a bold electric violet and gold, heavy section bars, crisp corners and bold menus.
    {
        Theme t = hand("violet", "Nicolas",
                       {{"window", "#121218"}, {"chrome", "#0b0b10"}, {"base", "#08080c"}, {"alt", "#17171f"}, {"menu", "#1a1a24"}, {"button", "#22222e"}, {"hover", "#2e2e3e"},
                        {"hi", "#7b3cff"}, {"hitext", "#ffffff"}, {"text", "#e6e6f0"}, {"bright", "#ffffff"}, {"dim", "#9a9ab0"}, {"disabled", "#55556a"}, {"border", "#2a2a3a"},
                        {"handle", "#3a3a50"}, {"section", "#fdb927"}, {"sectiontext", "#0b0b10"}, {"light", "#3a3a4e"}, {"shadow", "#000000"}});
        t.font = QStringList{"Inter", "Roboto", "Segoe UI", "Noto Sans", "DejaVu Sans", "Arial"};
        t.point_size = 9;
        t.menu_bold = true;
        t.radius = 3;
        t.border = 1;
        t.tabs = "underline";
        t.padding = 4;
        l << t;
    }
    // Martino: Softimage XSI, matched to its screens: warm grey panels and lists, soft raised buttons a shade lighter
    // with rounded corners (lit white when on), blue-grey section and panel title bars with black bold type, dark
    // rounded entry fields with light type, a white bar with black type for what is picked, Tahoma.
    {
        Theme t = hand("martino", "Martino",
                       {{"window", "#aca8a7"}, {"chrome", "#a7a3a2"}, {"base", "#aca8a7"}, {"alt", "#a6a2a1"}, {"menu", "#cccbc9"}, {"button", "#b6b3b1"}, {"hover", "#c6c3c1"},
                        {"hi", "#ffffff"}, {"hitext", "#000000"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#3a3938"}, {"disabled", "#858281"}, {"border", "#7a7776"},
                        {"handle", "#b6b3b1"}, {"section", "#899aa4"}, {"sectiontext", "#000000"}, {"light", "#d6d3d1"}, {"shadow", "#838182"},
                        {"field", "#353334"}, {"fieldtext", "#f2f0f1"}});
        t.font = QStringList{"Tahoma", "Verdana", "DejaVu Sans", "Segoe UI", "Arial"};
        t.point_size = 8;
        t.menu_bold = false;
        t.fill = "gradient";
        t.bevel = "none";
        t.radius = 6;
        t.border = 1;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "standard";
        t.padding = 3;
        l << t;
    }
    // Keating: the Dracula scheme as its author publishes it (MIT), pink highlights, purple section bars, rounded type and very round corners.
    {
        Theme t = hand("dracula", "Keating",
                       {{"window", "#282a36"}, {"chrome", "#21222c"}, {"base", "#1b1c24"}, {"alt", "#2c2e3c"}, {"menu", "#343746"}, {"button", "#44475a"}, {"hover", "#565a72"},
                        {"hi", "#ff79c6"}, {"hitext", "#1b1c24"}, {"text", "#f8f8f2"}, {"bright", "#ffffff"}, {"dim", "#a0a5c4"}, {"disabled", "#6272a4"}, {"border", "#16171e"},
                        {"handle", "#5a5e78"}, {"section", "#bd93f9"}, {"sectiontext", "#1b1c24"}});
        t.font = QStringList{"Nunito", "Varela Round", "Quicksand", "Comfortaa", "Arial Rounded MT Bold", "Segoe UI", "Noto Sans", "Verdana"};
        t.point_size = 9;
        t.fill = "flat";
        t.radius = 14;
        t.border = 1;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 5;
        l << t;
    }
    // Springer: the Solarized dark scheme as its author publishes it (MIT), with its yellow and orange: condensed type, single bevels, square edges.
    {
        Theme t = hand("solarized-dark", "Springer",
                       {{"window", "#073642"}, {"chrome", "#04303b"}, {"base", "#002b36"}, {"alt", "#06313c"}, {"menu", "#0b4251"}, {"button", "#114f5f"}, {"hover", "#1a6274"},
                        {"hi", "#b58900"}, {"hitext", "#002b36"}, {"text", "#93a1a1"}, {"bright", "#eee8d5"}, {"dim", "#839496"}, {"disabled", "#586e75"}, {"border", "#00212b"},
                        {"handle", "#2a6474"}, {"section", "#cb4b16"}, {"sectiontext", "#fdf6e3"}, {"light", "#2a6a7a"}, {"shadow", "#001a22"}});
        t.font = QStringList{"Roboto Condensed", "Barlow Condensed", "Arial Narrow", "Helvetica Neue Condensed", "DejaVu Sans Condensed", "Liberation Sans Narrow", "Arial"};
        t.point_size = 10;
        t.menu_bold = true;
        t.fill = "flat";
        t.bevel = "single";
        t.radius = 0;
        t.tabs = "boxed";
        t.arrows = "boxed";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // Gibson: deep forest green and lime, a humanist typeface, soft gradients.
    {
        Theme t = tonal("graphite", "Gibson", 145, 0.32, 13, "#9be15d");
        t.font = QStringList{"Lato", "Open Sans", "Source Sans 3", "Source Sans Pro", "Fira Sans", "Calibri", "Noto Sans", "DejaVu Sans"};
        t.point_size = 9;
        t.fill = "gradient";
        t.radius = 6;
        t.border = 1;
        t.tabs = "underline";
        t.arrows = "triangle";
        t.scrollbar = "thin";
        t.padding = 4;
        t.c["hitext"] = QColor("#0d1f0d");
        t.c["section"] = QColor("#21402b");
        t.c["sectiontext"] = QColor("#c8f59e");
        l << t;
    }
    // Novacek: the Nord scheme as its author publishes it (MIT): frost blues, a geometric typeface, borderless, round-topped tabs.
    {
        Theme t = hand("nord", "Novacek",
                       {{"window", "#3b4252"}, {"chrome", "#323946"}, {"base", "#2e3440"}, {"alt", "#3f4758"}, {"menu", "#434c5e"}, {"button", "#4c566a"}, {"hover", "#5a657c"},
                        {"hi", "#88c0d0"}, {"hitext", "#2e3440"}, {"text", "#d8dee9"}, {"bright", "#eceff4"}, {"dim", "#a3adbf"}, {"disabled", "#6c778c"}, {"border", "#272c36"},
                        {"handle", "#616d85"}, {"section", "#5e81ac"}, {"sectiontext", "#eceff4"}});
        t.font = QStringList{"Montserrat", "Poppins", "Futura", "Century Gothic", "Avenir Next", "Avenir", "URW Gothic", "Gill Sans", "Verdana"};
        t.point_size = 9;
        t.fill = "flat";
        t.radius = 7;
        t.border = 0;
        t.tabs = "rounded";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 4;
        l << t;
    }
    // Ozymok: plum and hot pink, Gill-style type with italic menus, gradient buttons with a single bevel.
    {
        Theme t = tonal("slate", "Ozymok", 330, 0.28, 18, "#ff4f8b");
        t.font = QStringList{"Gill Sans", "Gill Sans MT", "Josefin Sans", "Raleway", "Optima", "Candara", "Noto Sans", "DejaVu Sans"};
        t.point_size = 9;
        t.menu_italic = true;
        t.fill = "gradient";
        t.bevel = "single";
        t.radius = 3;
        t.tabs = "boxed";
        t.arrows = "boxed";
        t.scrollbar = "standard";
        t.padding = 3;
        t.c["hitext"] = QColor("#ffffff");
        t.c["section"] = QColor("#5a1f3a");
        t.c["sectiontext"] = QColor("#ffd1e3");
        l << t;
    }
    // Tureski: warm pewter grey and orange, wide Verdana-style type, double bevels, orange section bars.
    {
        Theme t = tonal("pewter", "Tureski", 30, 0.05, 44, "#e08a2e");
        t.font = QStringList{"Verdana", "Tahoma", "DejaVu Sans", "Bitstream Vera Sans", "Arial"};
        t.point_size = 10;
        t.fill = "gradient";
        t.bevel = "double";
        t.radius = 2;
        t.border = 1;
        t.tabs = "boxed";
        t.arrows = "boxed";
        t.scrollbar = "bevel";
        t.padding = 3;
        t.c["section"] = QColor("#e08a2e");
        t.c["sectiontext"] = QColor("#1e1206");
        l << t;
    }
    // Sebdvvv: the animation package of the mid 1990s on the workstations: grey panels, pink entry fields, bold italic
    // menus, bevelled buttons and round-topped tabs.
    {
        Theme t = hand("studio96", "Sebdvvv",
                       {{"window", "#a0a0a0"}, {"chrome", "#949494"}, {"base", "#bdbdbd"}, {"alt", "#b4b4b4"}, {"menu", "#b0b0b0"}, {"button", "#b2b2b2"}, {"hover", "#c2c2c2"},
                        {"hi", "#5a6f92"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#2c2c2c"}, {"disabled", "#6e6e6e"}, {"border", "#2e2e2e"}, {"handle", "#b2b2b2"},
                        {"light", "#ececec"}, {"shadow", "#5e5e5e"}, {"field", "#eab4b4"}, {"fieldtext", "#000000"}});
        t.font = kHelvetica;
        t.point_size = 9;
        t.menu_italic = true;
        t.menu_bold = true;
        t.bevel = "single";
        t.radius = 0;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // Wavefront: the workstation desktop the first release of the reference ran on: light grey, deep bevels, upright
    // Helvetica.
    {
        Theme t = hand("workstation", "Wavefront",
                       {{"window", "#c2c2c2"}, {"chrome", "#b2b2b2"}, {"base", "#dedede"}, {"alt", "#d4d4d4"}, {"menu", "#cacaca"}, {"button", "#c6c6c6"}, {"hover", "#d2d2d2"},
                        {"hi", "#52709f"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#383838"}, {"disabled", "#7c7c7c"}, {"border", "#3c3c3c"}, {"handle", "#c6c6c6"},
                        {"light", "#f4f4f4"}, {"shadow", "#6c6c6c"}, {"field", "#e4e4e4"}, {"fieldtext", "#000000"}});
        t.font = kHelvetica;
        t.point_size = 9;
        t.menu_bold = true;
        t.bevel = "single";
        t.radius = 0;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // Light grey with mid grey fields and darker section bars, the reference's look on the workstations of the 2000s.
    {
        Theme t = hand("poweranimator", "Hourihan",
                       {{"window", "#c6c6c6"}, {"chrome", "#b6b6b6"}, {"base", "#d8d8d8"}, {"alt", "#cfcfcf"}, {"menu", "#cccccc"}, {"button", "#c6c6c6"}, {"hover", "#d4d4d4"},
                        {"hi", "#56739f"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#303030"}, {"disabled", "#7a7a7a"}, {"border", "#3c3c3c"}, {"handle", "#c6c6c6"},
                        {"light", "#f2f2f2"}, {"shadow", "#727272"}, {"field", "#a9a9a9"}, {"fieldtext", "#000000"}, {"section", "#a2a2a2"}, {"sectiontext", "#000000"}});
        t.font = kHelvetica;
        t.point_size = 9;
        t.bevel = "single";
        t.radius = 0;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // The animation package of the late 1990s in its own dark scheme: slate panels, white section bars with black
    // text, black for what is on, a fixed-width face.
    {
        Theme t = hand("alias", "Alias",
                       {{"window", "#5b5d6b"}, {"chrome", "#4f515e"}, {"base", "#434551"}, {"alt", "#555764"}, {"menu", "#666875"}, {"button", "#6c6e7b"}, {"hover", "#7b7d8b"},
                        {"hi", "#000000"}, {"hitext", "#ffffff"}, {"text", "#ffffff"}, {"bright", "#ffffff"}, {"dim", "#cfd0d8"}, {"disabled", "#9a9ca8"}, {"border", "#1c1d24"},
                        {"handle", "#b3b0a8"}, {"light", "#9294a2"}, {"shadow", "#2a2b33"}, {"field", "#4b4d5a"}, {"fieldtext", "#ffffff"}, {"section", "#ffffff"},
                        {"sectiontext", "#000000"}});
        t.font = QStringList{"Terminus", "Fixed", "Monospace", "DejaVu Sans Mono", "Liberation Mono", "Courier New"};
        t.point_size = 9;
        t.bevel = "single";
        t.radius = 0;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // DonnieAlex: the open source 3D suite of the 2020s, in DonnieAlex's high-contrast take: mid-dark grey regions, its
    // blue for the panel headers and for what is on and picked, grey widgets with soft round corners, near-black entry
    // fields, a serif face, chevrons on the panels.
    {
        Theme t = hand("classic98", "DonnieAlex",
                       {{"window", "#2c2c2c"}, {"chrome", "#232323"}, {"base", "#1d1d1d"}, {"alt", "#2b2b2b"}, {"menu", "#181818"}, {"button", "#545454"}, {"hover", "#656565"},
                        {"hi", "#4772b3"}, {"hitext", "#ffffff"}, {"text", "#e6e6e6"}, {"bright", "#ffffff"}, {"dim", "#a5a5a5"}, {"disabled", "#737373"}, {"border", "#191919"},
                        {"handle", "#545454"}, {"section", "#4772b3"}, {"sectiontext", "#e6e6e6"}, {"light", "#5e5e5e"}, {"shadow", "#141414"},
                        {"field", "#1d1d1d"}, {"fieldtext", "#e6e6e6"}});
        t.font = QStringList{"Noto Serif"};
        t.point_size = 9;
        t.fill = "flat";
        t.radius = 5;
        t.border = 1;
        t.tabs = "rounded";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 4;
        l << t;
    }
    // Krieger: the desktop of 1998: silver, white fields, navy selection, two-tone raised and sunken edges.
    {
        Theme t = hand("desktop98", "Krieger",
                       {{"window", "#c0c0c0"}, {"chrome", "#c0c0c0"}, {"base", "#ffffff"}, {"alt", "#f4f4f4"}, {"menu", "#c0c0c0"}, {"button", "#c0c0c0"}, {"hover", "#c0c0c0"},
                        {"hi", "#000080"}, {"text", "#000000"}, {"bright", "#000000"}, {"dim", "#404040"}, {"disabled", "#808080"}, {"border", "#000000"}, {"handle", "#c0c0c0"},
                        {"light", "#ffffff"}, {"shadow", "#808080"}, {"field", "#ffffff"}, {"fieldtext", "#000000"}});
        t.font = QStringList{"Tahoma", "Microsoft Sans Serif", "MS Sans Serif", "Liberation Sans", "Arial"};
        t.point_size = 8;
        t.bevel = "double";
        t.radius = 0;
        t.tabs = "boxed";
        t.arrows = "boxed";
        t.scrollbar = "bevel";
        t.padding = 2;
        l << t;
    }
    // AshT: the white kitten with the red bow: candy pink and white, its red for what is picked and on the section bars,
    // the yellow of its nose for hover, black outlines, rounded type and the roundest pills.
    {
        Theme t = hand("ash", "AshT",
                       {{"window", "#ffd6e6"}, {"chrome", "#ffc2da"}, {"base", "#ffffff"}, {"alt", "#fff0f6"}, {"menu", "#ffffff"}, {"button", "#ffffff"}, {"hover", "#ffe680"},
                        {"hi", "#e60023"}, {"hitext", "#ffffff"}, {"text", "#2b1a1f"}, {"bright", "#000000"}, {"dim", "#7a5562"}, {"disabled", "#c79aab"}, {"border", "#1a1a1a"},
                        {"handle", "#ff8fb8"}, {"section", "#e60023"}, {"sectiontext", "#ffffff"}, {"light", "#ffffff"}, {"shadow", "#d98aa8"},
                        {"field", "#ffffff"}, {"fieldtext", "#2b1a1f"}});
        t.font = QStringList{"Nunito", "Varela Round", "Quicksand", "Comfortaa", "Arial Rounded MT Bold", "Segoe UI", "Noto Sans", "Verdana"};
        t.point_size = 9;
        t.menu_bold = true;
        t.fill = "flat";
        t.radius = 12;
        t.border = 2;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 5;
        l << t;
    }
    // Hourcade: pale sky blue with navy, a humanist typeface, navy section bars, round pills.
    {
        Theme t = tonal("sky", "Hourcade", 205, 0.45, 86, "#0b3d91");
        t.font = QStringList{"Lato", "Open Sans", "Source Sans 3", "Source Sans Pro", "Fira Sans", "Calibri", "Noto Sans", "DejaVu Sans"};
        t.point_size = 9;
        t.fill = "flat";
        t.radius = 9;
        t.border = 1;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 5;
        t.c["section"] = QColor("#0b3d91");
        t.c["sectiontext"] = QColor("#ffffff");
        l << t;
    }
    // Buxton: the Solarized light scheme as its author publishes it (MIT), with its cyan and magenta: serif type, round-topped tabs.
    {
        Theme t = hand("solarized-light", "Buxton",
                       {{"window", "#eee8d5"}, {"chrome", "#e4ddc8"}, {"base", "#fdf6e3"}, {"alt", "#f5efdc"}, {"menu", "#fdf6e3"}, {"button", "#e6dfca"}, {"hover", "#d9d1b9"},
                        {"hi", "#2aa198"}, {"hitext", "#fdf6e3"}, {"text", "#586e75"}, {"bright", "#073642"}, {"dim", "#839496"}, {"disabled", "#a7a99a"}, {"border", "#c9c1a8"},
                        {"handle", "#b8b09a"}, {"section", "#d33682"}, {"sectiontext", "#fdf6e3"}});
        t.font = QStringList{"Georgia", "Charter", "Iowan Old Style", "Palatino Linotype", "Palatino", "Noto Serif", "DejaVu Serif", "Liberation Serif", "Times New Roman"};
        t.point_size = 10;
        t.fill = "flat";
        t.radius = 3;
        t.tabs = "rounded";
        t.arrows = "triangle";
        t.scrollbar = "standard";
        t.padding = 3;
        l << t;
    }
    // Hall: cream paper and red ink, typewriter type, bold menus, square corners, black section bars.
    {
        Theme t = tonal("paper", "Hall", 42, 0.55, 91, "#c0392b");
        t.font = QStringList{"Courier Prime", "American Typewriter", "Courier New", "Courier", "Nimbus Mono PS", "Liberation Mono", "DejaVu Sans Mono"};
        t.point_size = 10;
        t.menu_bold = true;
        t.fill = "flat";
        t.radius = 0;
        t.border = 1;
        t.tabs = "underline";
        t.arrows = "triangle";
        t.scrollbar = "thin";
        t.padding = 3;
        t.c["section"] = QColor("#2b2b2b");
        t.c["sectiontext"] = QColor("#f6ecd2");
        t.c["text"] = QColor("#2a2118");
        t.c["bright"] = QColor("#000000");
        l << t;
    }
    // Stam: mint and teal, rounded type, gradients, the roundest corners and the most room.
    {
        Theme t = tonal("lilac", "Stam", 160, 0.4, 95, "#0f9d8a");
        t.font = QStringList{"Nunito", "Varela Round", "Quicksand", "Comfortaa", "Arial Rounded MT Bold", "Segoe UI", "Noto Sans", "Verdana"};
        t.point_size = 9;
        t.fill = "gradient";
        t.radius = 12;
        t.border = 1;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 6;
        t.c["section"] = QColor("#c8efe4");
        t.c["sectiontext"] = QColor("#0b5a50");
        l << t;
    }
    // Boisvert: the Mac desktop of the 2020s in its light appearance: pale grey windows and toolbars, white controls and
    // fields, its blue for what is picked, SF Pro, soft round corners, segmented tabs, scroll bars that stay out of the way.
    {
        Theme t = hand("macos", "Macintosh",
                       {{"window", "#ececec"}, {"chrome", "#e3e3e3"}, {"base", "#ffffff"}, {"alt", "#f5f5f5"}, {"menu", "#f6f6f6"}, {"button", "#ffffff"}, {"hover", "#f0f0f0"},
                        {"hi", "#007aff"}, {"hitext", "#ffffff"}, {"text", "#1d1d1f"}, {"bright", "#000000"}, {"dim", "#6e6e73"}, {"disabled", "#b0b0b5"}, {"border", "#c8c8cc"},
                        {"handle", "#c1c1c1"}, {"section", "#e3e3e8"}, {"sectiontext", "#1d1d1f"}, {"light", "#ffffff"}, {"shadow", "#b6b6bb"},
                        {"field", "#ffffff"}, {"fieldtext", "#1d1d1f"}});
        t.font = QStringList{"SF Pro Text", "SF Pro", ".AppleSystemUIFont", "Helvetica Neue", "Inter", "Segoe UI", "Noto Sans", "DejaVu Sans", "Arial"};
        t.point_size = 10;
        t.fill = "flat";
        t.radius = 6;
        t.border = 1;
        t.tabs = "pill";
        t.arrows = "chevron";
        t.scrollbar = "thin";
        t.padding = 4;
        l << t;
    }
    // Buttons, panel edges and title bars: each theme its own, so no two built-ins draw the same.
    static const QHash<QString, std::array<const char*, 3>> shape_of{
        {"obsidian", {"outline", "line", "underline"}},  {"tangerine", {"pill", "rounded", "accent"}},   {"midnight", {"glossy", "groove", "tab"}},
        {"violet", {"chunky", "thick", "inverse"}},      {"martino", {"soft", "line", "gradient"}},      {"dracula", {"pill", "rounded", "tab"}},
        {"solarized-dark", {"auto", "bevel", "accent"}}, {"graphite", {"soft", "shadow", "gradient"}},   {"nord", {"underline", "none", "underline"}},
        {"slate", {"glossy", "shadow", "inverse"}},      {"pewter", {"auto", "bevel", "gradient"}},      {"workstation", {"auto", "bevel", "auto"}},
        {"poweranimator", {"auto", "groove", "gradient"}}, {"alias", {"auto", "line", "inverse"}},       {"desktop98", {"auto", "bevel", "auto"}},
        {"sky", {"outline", "rounded", "accent"}},       {"solarized-light", {"inset", "line", "tab"}},  {"paper", {"chunky", "shadow", "underline"}},
        {"lilac", {"pill", "rounded", "gradient"}},      {"shaq", {"auto", "line", "auto"}},             {"classic98", {"auto", "rounded", "auto"}},
        {"ash", {"pill", "rounded", "tab"}},             {"macos", {"auto", "rounded", "auto"}}};
    // Flav (carbon) and Sebdvvv (studio96) as they first were: no button, frame or title style of their own.
    for (Theme& t : l) {
        const auto it = shape_of.find(t.id);
        if (it == shape_of.end()) continue;
        t.buttons = it->at(0);
        t.frames = it->at(1);
        t.titles = it->at(2);
    }
    return l;
}

QString prefs_root() {
    return take(aztek_preferences_dir());
}

QList<Theme> user_themes() {
    QList<Theme> l;
    QDir d(themes_folder());
    for (const QString& f : d.entryList({"*.json"}, QDir::Files, QDir::Name)) {
        QFile file(d.filePath(f));
        if (!file.open(QIODevice::ReadOnly)) continue;
        const QJsonDocument doc = QJsonDocument::fromJson(file.readAll());
        if (!doc.isObject()) continue;
        Theme t = Theme::from_json(doc.object());
        if (t.id.isEmpty()) t.id = QFileInfo(f).completeBaseName();
        t.builtin = false;
        l << t;
    }
    return l;
}

// ---- Pictures the style sheet points at: arrows, ticks and bevels drawn in the theme's colours.

QString assets_dir(const Theme& t) {
    // The drawing's version goes into the key, so pictures drawn by an older Aztek are not used again.
    const QByteArray key = QCryptographicHash::hash("5" + QJsonDocument(t.to_json()).toJson(QJsonDocument::Compact), QCryptographicHash::Md5).toHex().left(12);
    return QStandardPaths::writableLocation(QStandardPaths::CacheLocation) + "/themes/" + QString::fromLatin1(key);
}

QImage canvas(int w, int h) {
    QImage im(w, h, QImage::Format_ARGB32_Premultiplied);
    im.fill(Qt::transparent);
    return im;
}

// An arrow pointing down, turned to the other directions: a chevron or a solid triangle, drawn large and scaled down.
void save_arrow(const QString& path, const QColor& c, const QString& style, int turns) {
    QImage im = canvas(32, 32);
    QPainter p(&im);
    p.setRenderHint(QPainter::Antialiasing);
    p.translate(16, 16);
    p.rotate(90.0 * turns);
    p.translate(-16, -16);
    if (style == "chevron") {
        QPen pen(c, 3.4, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin);
        p.setPen(pen);
        p.drawPolyline(QPolygonF({QPointF(8.5, 12), QPointF(16, 19.5), QPointF(23.5, 12)}));
    } else {
        p.setPen(Qt::NoPen);
        p.setBrush(c);
        p.drawPolygon(QPolygonF({QPointF(7, 11), QPointF(25, 11), QPointF(16, 22)}));
    }
    p.end();
    im.save(path);
}

void save_check(const QString& path, const QColor& c) {
    QImage im = canvas(32, 32);
    QPainter p(&im);
    p.setRenderHint(QPainter::Antialiasing);
    p.setPen(QPen(c, 4.2, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
    p.drawPolyline(QPolygonF({QPointF(7, 16.5), QPointF(13.5, 23), QPointF(25, 9.5)}));
    p.end();
    im.save(path);
}

void save_dot(const QString& path, const QColor& c) {
    QImage im = canvas(32, 32);
    QPainter p(&im);
    p.setRenderHint(QPainter::Antialiasing);
    p.setPen(Qt::NoPen);
    p.setBrush(c);
    p.drawEllipse(QPointF(16, 16), 6.5, 6.5);
    p.end();
    im.save(path);
}

// A 6 by 6 picture for border-image: two pixel edges round a face. single: light top and left, shadow bottom and right,
// Motif's. double: an outer and an inner ring of each, the 1998 desktop's (light and a mid light, border and shadow).
// Sunken swaps them. open_bottom leaves the bottom as face, for tabs.
void save_bevel(const QString& path, const Theme& t, const QColor& face, bool sunken, bool open_bottom = false) {
    QImage im(6, 6, QImage::Format_ARGB32);
    im.fill(face);
    const QColor light = t.c["light"], shadow = t.c["shadow"], button = t.c["button"];
    const QColor mid((light.red() + button.red()) / 2, (light.green() + button.green()) / 2, (light.blue() + button.blue()) / 2);
    QColor otl, itl, ibr, obr;
    if (t.bevel == "double") {
        otl = sunken ? shadow : light;
        itl = sunken ? t.c["border"] : mid;
        ibr = sunken ? mid : shadow;
        obr = sunken ? light : t.c["border"];
    } else {
        otl = itl = sunken ? shadow : light;
        ibr = obr = sunken ? light : shadow;
    }
    for (int ring = 0; ring < 2; ++ring) {
        const QColor tl = ring == 0 ? otl : itl, br = ring == 0 ? obr : ibr;
        const int lo = ring, hi = 5 - ring;
        for (int i = lo; i <= hi; ++i) {
            im.setPixelColor(i, lo, tl);
            im.setPixelColor(lo, i, tl);
        }
        for (int i = lo; i <= hi; ++i) im.setPixelColor(hi, i, br);
        if (!open_bottom) {
            for (int i = lo; i <= hi; ++i) im.setPixelColor(i, hi, br);
            // Where light meets dark the corner goes dark, as both toolkits draw it.
            im.setPixelColor(lo, hi, br);
        } else {
            for (int i = hi; i <= 5; ++i) im.setPixelColor(lo, i, tl);
        }
        im.setPixelColor(hi, lo, br);
    }
    if (open_bottom) {
        for (int y = 4; y < 6; ++y)
            for (int x = 2; x < 4; ++x) im.setPixelColor(x, y, face);
        // The 1998 tab's top corners are clipped by a pixel.
        if (t.bevel == "double") {
            im.setPixelColor(0, 0, Qt::transparent);
            im.setPixelColor(5, 0, Qt::transparent);
            im.setPixelColor(1, 1, t.c["light"]);
        }
    }
    im.save(path);
}

// A radio button's box: a sunken circle (1998) or a sunken diamond (Motif), with its dot when on.
void save_radio(const QString& path, const Theme& t, bool on) {
    QImage im = canvas(26, 26);
    QPainter p(&im);
    p.setRenderHint(QPainter::Antialiasing);
    const QColor light = t.c["light"], shadow = t.c["shadow"];
    if (t.bevel == "double") {
        p.setPen(Qt::NoPen);
        p.setBrush(light);
        p.drawEllipse(QRectF(1, 1, 24, 24));
        p.setBrush(shadow);
        p.drawChord(QRectF(1, 1, 24, 24), 45 * 16, 180 * 16);
        p.setBrush(t.c["border"]);
        p.drawChord(QRectF(3, 3, 20, 20), 45 * 16, 180 * 16);
        p.setBrush(t.c["field"]);
        p.drawEllipse(QRectF(5, 5, 16, 16));
        if (on) {
            p.setBrush(t.c["fieldtext"]);
            p.drawEllipse(QRectF(9.5, 9.5, 7, 7));
        }
    } else {
        // Light on the top two edges and shadow on the bottom two when off; sunken (swapped) and filled when on.
        const QPointF top(13, 1), right(25, 13), bottom(13, 25), left(1, 13);
        p.setPen(Qt::NoPen);
        p.setBrush(on ? t.c["hi"] : t.c["button"]);
        p.drawPolygon(QPolygonF({top, right, bottom, left}));
        QPen pen(on ? shadow : light, 3.0);
        pen.setJoinStyle(Qt::MiterJoin);
        p.setPen(pen);
        p.drawPolyline(QPolygonF({left, top, right}));
        pen.setColor(on ? light : shadow);
        p.setPen(pen);
        p.drawPolyline(QPolygonF({left, bottom, right}));
    }
    p.end();
    im.save(path);
}

// The tear-off line at the top of a menu: dashes across its width.
void save_tearoff(const QString& path, const QColor& c) {
    QImage im = canvas(400, 6);
    QPainter p(&im);
    for (int x = 0; x < 400; x += 7) p.fillRect(x, 2, 4, 2, c);
    p.end();
    im.save(path);
}

void save_dither(const QString& path, const QColor& a, const QColor& b) {
    QImage im(2, 2, QImage::Format_ARGB32);
    im.setPixelColor(0, 0, a);
    im.setPixelColor(1, 1, a);
    im.setPixelColor(1, 0, b);
    im.setPixelColor(0, 1, b);
    im.save(path);
}

QString write_assets(const Theme& t) {
    const QString dir = assets_dir(t);
    if (QFile::exists(dir + "/done")) return dir;
    QDir().mkpath(dir);
    const QString style = t.arrows == "chevron" ? "chevron" : "triangle";
    const char* names[] = {"down", "left", "up", "right"};
    for (int k = 0; k < 4; ++k) {
        save_arrow(dir + "/" + names[k] + ".png", t.c["text"], style, k);
        save_arrow(dir + "/" + names[k] + "-off.png", t.c["disabled"], style, k);
        save_arrow(dir + "/" + names[k] + "-hi.png", t.c["hitext"], style, k);
    }
    const bool bevel = t.bevel != "none";
    save_check(dir + "/check.png", bevel ? t.c["fieldtext"] : t.c["hitext"]);
    save_check(dir + "/check-off.png", t.c["disabled"]);
    save_dot(dir + "/dot.png", bevel ? t.c["fieldtext"] : t.c["hitext"]);
    save_tearoff(dir + "/tearoff.png", t.c["dim"]);
    save_tearoff(dir + "/tearoff-hi.png", t.c["hi"]);
    if (bevel) {
        save_bevel(dir + "/raised.png", t, t.c["button"], false);
        save_bevel(dir + "/raised-hover.png", t, t.c["hover"], false);
        save_bevel(dir + "/sunken.png", t, t.c["button"], true);
        save_bevel(dir + "/raised-menu.png", t, t.c["menu"], false);
        save_bevel(dir + "/raised-window.png", t, t.c["window"], false);
        save_bevel(dir + "/raised-section.png", t, t.c["section"], false);
        save_bevel(dir + "/field.png", t, t.c["field"], true);
        save_bevel(dir + "/well.png", t, t.c["base"], true);
        save_bevel(dir + "/tab.png", t, t.c["button"], false, true);
        save_bevel(dir + "/tab-on.png", t, t.c["window"], false, true);
        save_dither(dir + "/dither.png", t.c["light"], t.c["button"]);
        save_radio(dir + "/radio.png", t, false);
        save_radio(dir + "/radio-on.png", t, true);
    }
    QFile done(dir + "/done");
    done.open(QIODevice::WriteOnly);
    return dir;
}

QString px(int v) { return QString::number(v) + "px"; }

QString grad(const QColor& c, int up, int down) {
    return QString("qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0 %1, stop:1 %2)").arg(c.lighter(up).name(), c.darker(down).name());
}

// What a theme adds over the classic stylesheet.
QString skin(const Theme& t, const QString& a) {
    QString s;
    const auto& c = t.c;
    const QString url = "url(" + a + "/";
    const QString pad = px(t.padding), pad2 = px(t.padding + 9);
    if (t.menu_italic || t.menu_bold) {
        QString f;
        if (t.menu_italic) f += "font-style: italic; ";
        if (t.menu_bold) f += "font-weight: bold; ";
        s += "QMenuBar, QMenu, QTabBar::tab, QDockWidget::title { " + f + "}\n";
    }
    // Selected things take the highlight's own text colour everywhere.
    s += "QMenuBar::item:selected, QMenu::item:selected { background: @hi; color: @hitext; }\n";
    s += "QTreeView::item:selected, QListView::item:selected, QTableView::item:selected { background: @hi; color: @hitext; }\n";
    s += "QLabel#channelName[picked=\"true\"] { color: @hitext; }\n";
    // The tear-off line at the top of a menu: click it and the menu floats as a window of its own.
    s += "QMenu::tearoff { image: url(" + a + "/tearoff.png); height: 8px; margin: 2px 6px; }\n";
    s += "QMenu::tearoff:selected { image: url(" + a + "/tearoff-hi.png); }\n";
    if (t.fill == "classic") return s;

    const bool bevel = t.bevel != "none";
    const QString r = px(t.radius), r2 = px(std::max(0, t.radius - 2)), bw = px(t.border);
    const QString button_bg = t.fill == "gradient" ? grad(c["button"], 110, 104) : QString("@button");
    const QString hover_bg = t.fill == "gradient" ? grad(c["hover"], 110, 104) : QString("@hover");
    const QString edge = "border-image: " + url + "%1.png) 2 2 2 2 stretch stretch; border-width: 2px;";
    auto raised = [&](const QString& img = "raised") { return edge.arg(img); };

    // Buttons.
    if (bevel) {
        s += "QPushButton { " + raised() + " color: @text; padding: " + pad + " 12px; min-height: 16px; }\n";
        s += "QPushButton:hover { " + raised("raised-hover") + " }\n";
        s += "QPushButton:pressed, QPushButton:checked { " + raised("sunken") + " }\n";
        s += "QPushButton:default { border-image: " + url + "raised.png) 2 2 2 2 stretch stretch; }\n";
    } else {
        s += "QPushButton { background: " + button_bg + "; border: " + bw + " solid @border; border-radius: " + r + "; padding: " + pad + " 12px; color: @text; min-height: 16px; }\n";
        s += "QPushButton:hover { background: " + hover_bg + "; }\n";
        s += "QPushButton:pressed, QPushButton:checked { background: @hi; border-color: @hi; color: @hitext; }\n";
        s += "QPushButton:focus { border-color: @hi; }\n";
    }
    s += "QPushButton:disabled { color: @disabled; }\n";

    // Tool buttons: flat until hovered or on.
    if (bevel) {
        s += "QToolButton { border: 2px solid transparent; background: transparent; padding: 1px; }\n";
        s += "QToolButton:hover { " + raised() + " }\n";
        s += "QToolButton:checked, QToolButton:pressed { " + raised("sunken") + " }\n";
    } else {
        s += "QToolButton { border: none; border-radius: " + r2 + "; background: transparent; padding: 2px; }\n";
        s += "QToolButton:hover { background: @hover; }\n";
        s += "QToolButton:checked, QToolButton:pressed { background: @hi; color: @hitext; }\n";
    }
    s += "QToolButton::menu-indicator { image: " + url + "down.png); width: 8px; height: 8px; subcontrol-position: bottom right; subcontrol-origin: padding; }\n";
    s += "QToolButton::menu-button { border: none; width: 12px; }\n";
    s += "QToolButton::menu-arrow { image: " + url + "down.png); width: 8px; height: 8px; }\n";

    // Fields.
    if (bevel) {
        s += "QLineEdit, QSpinBox, QDoubleSpinBox, QDateTimeEdit { " + raised("field") + " color: @fieldtext; padding: 1px 3px; selection-background-color: @hi; selection-color: @hitext; }\n";
        s += "QPlainTextEdit, QTextEdit, QListView, QTreeView, QTableView, QListWidget, QTreeWidget { " + raised("well") + " color: @text; }\n";
    } else {
        s += "QLineEdit, QSpinBox, QDoubleSpinBox, QDateTimeEdit { background: @field; color: @fieldtext; border: " + bw + " solid @border; border-radius: " + r2 + "; padding: " + px(std::max(1, t.padding - 1)) + " 5px; selection-background-color: @hi; selection-color: @hitext; }\n";
        s += "QLineEdit:focus, QSpinBox:focus, QDoubleSpinBox:focus { border-color: @hi; }\n";
        s += "QPlainTextEdit, QTextEdit, QListView, QTreeView, QTableView { border: " + bw + " solid @border; border-radius: " + r2 + "; }\n";
    }
    s += "QLineEdit:disabled, QSpinBox:disabled, QDoubleSpinBox:disabled { color: @disabled; }\n";
    // Fields that carry a state colour (the Channel Box's keyed, locked, driven) keep it.
    s += "QLineEdit#channelValue, QLineEdit#channelConnected, QLineEdit#channelKeyed, QLineEdit#channelKeyOnFrame, QLineEdit#channelChanged, QLineEdit#channelLayered, QLineEdit#channelMuted, QLineEdit#channelLocked, QLineEdit#channelHeader, QLineEdit#connectedField, QLineEdit#keyedField, QLineEdit#cmdResult[error=\"true\"] { border-image: none; border-radius: 0; padding: 0 3px; }\n";
    if (bevel) s += "QLineEdit#channelValue { background: @field; color: @fieldtext; border: none; }\n";

    // Spin box arrows.
    for (const char* k : {"Spin", "DoubleSpin"}) {
        const QString w = QString("Q%1Box").arg(k);
        if (bevel) {
            s += w + "::up-button, " + w + "::down-button { " + raised() + " width: 12px; }\n";
            s += w + "::up-button:pressed, " + w + "::down-button:pressed { " + raised("sunken") + " }\n";
        } else {
            s += w + "::up-button, " + w + "::down-button { border: none; background: transparent; width: 16px; }\n";
            s += w + "::up-button:hover, " + w + "::down-button:hover { background: @hover; border-radius: " + r2 + "; }\n";
        }
        s += w + "::up-button { subcontrol-origin: border; subcontrol-position: top right; }\n";
        s += w + "::down-button { subcontrol-origin: border; subcontrol-position: bottom right; }\n";
        s += w + "::up-arrow { image: " + url + "up.png); width: 8px; height: 8px; }\n";
        s += w + "::down-arrow { image: " + url + "down.png); width: 8px; height: 8px; }\n";
        s += w + "::up-arrow:disabled, " + w + "::up-arrow:off { image: " + url + "up-off.png); }\n";
        s += w + "::down-arrow:disabled, " + w + "::down-arrow:off { image: " + url + "down-off.png); }\n";
    }

    // Drop-down boxes.
    if (t.bevel == "double") {
        // A sunken field with a raised arrow button inside its right edge.
        s += "QComboBox { " + raised("field") + " color: @fieldtext; padding: 1px 18px 1px 4px; min-height: 16px; }\n";
        s += "QComboBox::drop-down { subcontrol-origin: padding; subcontrol-position: top right; width: 16px; " + raised() + " }\n";
        s += "QComboBox::drop-down:on { " + raised("sunken") + " }\n";
    } else if (bevel) {
        s += "QComboBox { " + raised() + " color: @text; padding: " + pad + " 20px " + pad + " 8px; min-height: 16px; }\n";
        s += "QComboBox:hover { " + raised("raised-hover") + " }\n";
        s += "QComboBox::drop-down { subcontrol-origin: padding; subcontrol-position: center right; width: 18px; border: none; background: transparent; }\n";
    } else {
        s += "QComboBox { background: " + button_bg + "; border: " + bw + " solid @border; border-radius: " + r + "; padding: " + pad + " 22px " + pad + " 9px; color: @text; min-height: 16px; }\n";
        s += "QComboBox:hover { background: " + hover_bg + "; }\n";
        s += "QComboBox:on { border-color: @hi; }\n";
        s += "QComboBox::drop-down { subcontrol-origin: padding; subcontrol-position: center right; width: 20px; border: none; background: transparent; }\n";
    }
    s += "QComboBox::down-arrow { image: " + url + "down.png); width: 10px; height: 10px; }\n";
    s += "QComboBox::down-arrow:disabled { image: " + url + "down-off.png); }\n";
    s += "QComboBox:disabled { color: @disabled; }\n";
    s += "QComboBox QAbstractItemView { background: @menu; color: @text; border: 1px solid @border; selection-background-color: @hi; selection-color: @hitext; outline: 0; padding: 2px; }\n";

    // Check boxes and radio buttons.
    if (bevel) {
        s += "QCheckBox::indicator, QTreeView::indicator, QListView::indicator { width: 9px; height: 9px; " + raised("field") + " }\n";
        s += "QCheckBox::indicator:checked, QTreeView::indicator:checked, QListView::indicator:checked { image: " + url + "check.png); }\n";
        s += "QRadioButton::indicator { width: 13px; height: 13px; border: none; image: " + url + "radio.png); }\n";
        s += "QRadioButton::indicator:checked { image: " + url + "radio-on.png); }\n";
    } else {
        s += "QCheckBox::indicator, QTreeView::indicator, QListView::indicator { width: 13px; height: 13px; border: 1px solid @handle; border-radius: " + px(std::min(4, t.radius)) + "; background: @field; }\n";
        s += "QRadioButton::indicator { width: 13px; height: 13px; border: 1px solid @handle; border-radius: 7px; background: @field; }\n";
        s += "QCheckBox::indicator:hover, QRadioButton::indicator:hover { border-color: @hi; }\n";
        s += "QCheckBox::indicator:checked, QTreeView::indicator:checked, QListView::indicator:checked { background: @hi; border-color: @hi; image: " + url + "check.png); }\n";
        s += "QRadioButton::indicator:checked { background: @hi; border-color: @hi; image: " + url + "dot.png); }\n";
    }
    s += "QCheckBox::indicator:disabled, QRadioButton::indicator:disabled { background: @window; }\n";
    s += "QCheckBox:disabled, QRadioButton:disabled { color: @disabled; }\n";

    // Trees: arrows in place of the platform's branch marks.
    s += "QTreeView::branch:has-children:!has-siblings:closed, QTreeView::branch:closed:has-children:has-siblings { border-image: none; image: " + url + "right.png); }\n";
    s += "QTreeView::branch:open:has-children:!has-siblings, QTreeView::branch:open:has-children:has-siblings { border-image: none; image: " + url + "down.png); }\n";

    // Menus.
    if (bevel) {
        s += "QMenu { " + raised("raised-menu") + " padding: 1px; }\n";
        s += "QMenu::item { padding: 3px 22px 3px 22px; }\n";
        s += "QMenuBar { background: @window; }\n";
        if (t.bevel == "single") s += "QMenuBar::item:selected { " + raised("raised-window") + " color: @text; background: transparent; }\n";
    } else {
        s += "QMenu { background: @menu; border: 1px solid @border; border-radius: " + r + "; padding: 4px; }\n";
        s += "QMenu::item { padding: 4px 24px 4px 22px; border-radius: " + r2 + "; }\n";
        s += "QMenuBar::item { border-radius: " + r2 + "; margin: 1px 1px; }\n";
    }
    s += "QMenu::right-arrow { image: " + url + "right.png); width: 9px; height: 9px; }\n";
    s += "QMenu::right-arrow:selected { image: " + url + "right-hi.png); }\n";
    s += "QMenu::indicator:non-exclusive:checked { image: " + url + "check-off.png); width: 12px; height: 12px; }\n";
    s += "QMenu::indicator:exclusive:checked { image: " + url + "dot.png); width: 12px; height: 12px; }\n";
    s += "QMenu::separator { height: 1px; background: @border; margin: 4px 8px; }\n";

    // Tabs.
    s += "QTabWidget::pane { border: none; }\n";
    if (t.tabs == "underline") {
        s += "QTabBar::tab { background: transparent; color: @dim; border: none; border-bottom: 2px solid transparent; padding: 4px 12px; margin-right: 2px; }\n";
        s += "QTabBar::tab:hover { color: @bright; }\n";
        s += "QTabBar::tab:selected { color: @bright; border-bottom: 2px solid @hi; }\n";
    } else if (t.tabs == "pill") {
        s += "QTabBar::tab { background: transparent; color: @dim; border: none; border-radius: " + px(std::max(6, t.radius + 2)) + "; padding: 3px 12px; margin: 2px 2px; }\n";
        s += "QTabBar::tab:hover { background: @hover; color: @bright; }\n";
        s += "QTabBar::tab:selected { background: @hi; color: @hitext; }\n";
    } else if (t.tabs == "rounded") {
        if (bevel) {
            s += "QTabBar::tab { background: @button; color: @text; border: 2px solid; border-color: @light @shadow @button @light; border-top-left-radius: 7px; border-top-right-radius: 7px; padding: 2px 12px; margin-right: -1px; margin-top: 2px; }\n";
            s += "QTabBar::tab:selected { background: @light; margin-top: 0; }\n";
        } else {
            s += "QTabBar::tab { background: @button; color: @dim; border: 1px solid @border; border-bottom: none; border-top-left-radius: 8px; border-top-right-radius: 8px; padding: 3px 12px; margin-right: -1px; margin-top: 2px; }\n";
            s += "QTabBar::tab:selected { background: @window; color: @bright; margin-top: 0; }\n";
        }
    } else if (bevel) {
        s += "QTabBar::tab { border-image: " + url + "tab.png) 2 2 0 2 stretch stretch; border-width: 2px 2px 0 2px; color: @text; padding: 2px 10px; margin-top: 2px; }\n";
        s += "QTabBar::tab:selected { border-image: " + url + "tab-on.png) 2 2 0 2 stretch stretch; margin-top: 0; }\n";
    } else {
        s += "QTabBar::tab { background: @chrome; color: @dim; border: 1px solid @border; border-bottom: none; border-top-left-radius: " + r2 + "; border-top-right-radius: " + r2 + "; padding: 3px 10px; margin-right: -1px; }\n";
        s += "QTabBar::tab:selected { background: @window; color: @bright; }\n";
        s += "QTabBar::tab:hover:!selected { background: @hover; }\n";
    }
    s += "QTabBar::scroller { width: 18px; }\n";
    s += "QTabBar QToolButton::left-arrow { image: " + url + "left.png); } QTabBar QToolButton::right-arrow { image: " + url + "right.png); }\n";

    // Headers.
    if (bevel) s += "QHeaderView::section { " + raised() + " color: @text; padding: 1px 4px; }\n";
    else s += "QHeaderView::section { background: @chrome; color: @dim; border: none; border-right: 1px solid @border; border-bottom: 1px solid @border; padding: 3px 6px; }\n";

    // Scroll bars.
    if (t.scrollbar == "thin") {
        s += "QScrollBar:vertical { background: transparent; width: 10px; margin: 0; } QScrollBar:horizontal { background: transparent; height: 10px; margin: 0; }\n";
        s += "QScrollBar::handle:vertical, QScrollBar::handle:horizontal { background: @handle; border-radius: 3px; margin: 2px; }\n";
        s += "QScrollBar::handle:hover { background: @dim; }\n";
        s += "QScrollBar::add-line, QScrollBar::sub-line { width: 0; height: 0; border: none; background: none; }\n";
        s += "QScrollBar::add-page, QScrollBar::sub-page { background: none; }\n";
    } else {
        const int w = t.scrollbar == "bevel" ? 16 : 14;
        const QString track = t.bevel == "double" ? "background-image: " + url + "dither.png);" : QString("background: @chrome;");
        s += "QScrollBar:vertical { " + track + " width: " + px(w) + "; margin: " + px(w) + " 0 " + px(w) + " 0; border: none; }\n";
        s += "QScrollBar:horizontal { " + track + " height: " + px(w) + "; margin: 0 " + px(w) + " 0 " + px(w) + "; border: none; }\n";
        if (t.scrollbar == "bevel" && bevel) {
            s += "QScrollBar::handle:vertical, QScrollBar::handle:horizontal { " + raised() + " min-height: 16px; min-width: 16px; }\n";
            s += "QScrollBar::add-line, QScrollBar::sub-line { " + raised() + " }\n";
            s += "QScrollBar::add-line:pressed, QScrollBar::sub-line:pressed { " + raised("sunken") + " }\n";
        } else {
            s += "QScrollBar::handle:vertical, QScrollBar::handle:horizontal { background: " + button_bg + "; border: 1px solid @border; border-radius: " + r2 + "; margin: 1px; min-height: 18px; min-width: 18px; }\n";
            s += "QScrollBar::handle:hover { background: " + hover_bg + "; }\n";
            s += "QScrollBar::add-line, QScrollBar::sub-line { background: @chrome; border: none; }\n";
        }
        s += "QScrollBar::add-line:vertical { subcontrol-origin: margin; subcontrol-position: bottom; height: " + px(w) + "; }\n";
        s += "QScrollBar::sub-line:vertical { subcontrol-origin: margin; subcontrol-position: top; height: " + px(w) + "; }\n";
        s += "QScrollBar::add-line:horizontal { subcontrol-origin: margin; subcontrol-position: right; width: " + px(w) + "; }\n";
        s += "QScrollBar::sub-line:horizontal { subcontrol-origin: margin; subcontrol-position: left; width: " + px(w) + "; }\n";
        s += "QScrollBar::up-arrow:vertical { image: " + url + "up.png); width: 8px; height: 8px; }\n";
        s += "QScrollBar::down-arrow:vertical { image: " + url + "down.png); width: 8px; height: 8px; }\n";
        s += "QScrollBar::left-arrow:horizontal { image: " + url + "left.png); width: 8px; height: 8px; }\n";
        s += "QScrollBar::right-arrow:horizontal { image: " + url + "right.png); width: 8px; height: 8px; }\n";
        s += "QScrollBar::add-page, QScrollBar::sub-page { background: none; }\n";
    }

    // Sliders.
    if (bevel) {
        s += "QSlider::groove:horizontal { " + raised("well") + " height: 2px; }\n";
        s += "QSlider::handle:horizontal { " + raised() + " width: 8px; margin: -7px 0; }\n";
        s += "QSlider::groove:vertical { " + raised("well") + " width: 2px; }\n";
        s += "QSlider::handle:vertical { " + raised() + " height: 8px; margin: 0 -7px; }\n";
    } else {
        s += "QSlider::groove:horizontal { background: @border; height: 4px; border-radius: 2px; }\n";
        s += "QSlider::sub-page:horizontal { background: @hi; border-radius: 2px; }\n";
        s += "QSlider::handle:horizontal { background: @hi; border: 3px solid @window; width: 10px; height: 10px; margin: -6px 0; border-radius: 8px; }\n";
        s += "QSlider::groove:vertical { background: @border; width: 4px; border-radius: 2px; }\n";
        s += "QSlider::add-page:vertical { background: @hi; border-radius: 2px; }\n";
        s += "QSlider::handle:vertical { background: @hi; border: 3px solid @window; width: 10px; height: 10px; margin: 0 -6px; border-radius: 8px; }\n";
    }

    // Groups, docks, progress, tooltips.
    if (bevel) {
        s += "QGroupBox { border: 2px groove @shadow; margin-top: 10px; padding-top: 4px; }\n";
        s += "QGroupBox::title { subcontrol-origin: margin; left: 8px; padding: 0 3px; color: @text; }\n";
        s += "QProgressBar { " + raised("field") + " text-align: center; color: @fieldtext; } QProgressBar::chunk { background: @hi; }\n";
    } else {
        s += "QGroupBox { border: 1px solid @border; border-radius: " + r + "; margin-top: 12px; padding-top: 6px; }\n";
        s += "QGroupBox::title { subcontrol-origin: margin; left: 10px; padding: 0 4px; color: @dim; }\n";
        s += "QProgressBar { background: @base; border: 1px solid @border; border-radius: " + r2 + "; text-align: center; color: @text; } QProgressBar::chunk { background: @hi; border-radius: " + r2 + "; }\n";
    }
    if (t.bevel == "double") {
        // The 1998 title bar: a navy to blue gradient with bold white text.
        s += QString("QDockWidget { color: #ffffff; } QDockWidget::title { background: qlineargradient(x1:0, y1:0, x2:1, y2:0, stop:0 @hi, stop:1 %1); padding: 3px 6px; font-weight: bold; }\n").arg(c["hi"].lighter(260).name());
    } else if (bevel) {
        s += "QDockWidget::title { " + raised("raised-window") + " padding: 2px 6px; }\n";
    } else {
        // What sits under the title's text (padding and border) can't be more than what sits over it: the title's height
        // counts the top twice, so a heavier bottom clips the letters that hang below the line (#187).
        s += "QDockWidget::title { background: @chrome; padding: 4px 8px 3px 8px; border-bottom: 1px solid @border; }\n";
    }
    if (bevel) s += "QToolTip { background: #ffffe1; color: #000000; border: 1px solid #000000; padding: 2px; }\n";
    else s += "QToolTip { background: @menu; color: @bright; border: 1px solid @border; border-radius: " + r2 + "; padding: 4px 7px; }\n";

    // Aztek's own parts. Section heads of the tool settings and Attribute Editor: a bar, never the highlight.
    if (bevel) {
        s += "QToolButton#foldHead, QToolButton#foldHead:checked, QToolButton#foldHead:hover, QToolButton#aeSectionButton, QToolButton#aeSectionButton:checked { " + raised("raised-section") + " color: @sectiontext; font-weight: bold; text-align: left; padding: 1px 4px; }\n";
        s += "QLabel#aeSection { " + raised("raised-section") + " color: @sectiontext; }\n";
    } else {
        s += "QToolButton#foldHead, QToolButton#foldHead:checked, QToolButton#aeSectionButton, QToolButton#aeSectionButton:checked { background: @section; color: @sectiontext; font-weight: bold; border: none; border-radius: " + r2 + "; padding: 3px 6px; text-align: left; }\n";
        s += "QLabel#aeSection { background: @section; color: @sectiontext; border-radius: " + r2 + "; }\n";
        s += "QToolButton#foldHead:hover, QToolButton#aeSectionButton:hover { background: @hover; }\n";
    }
    s += "QToolButton#hotboxButton { background: @menu; color: @text; border: 1px solid @border; border-radius: " + r2 + "; padding: 5px 10px; }\n";
    s += "QToolButton#hotboxButton:hover { background: @hi; color: @hitext; }\n";
    s += "QLabel#hotboxCenter { border-radius: " + r2 + "; }\n";
    s += "QToolButton#statusHandle { border: none; border-left: 1px solid @border; border-right: 1px solid @light; background: @chrome; }\n";
    s += "QToolButton#statusHandle:hover { background: @hi; }\n";
    return s;
}

// Buttons, panel edges and title bars, drawn over what skin() gives, so themes that share a fill and bevel still
// look apart.
QString shapes(const Theme& t) {
    QString s;
    const auto& c = t.c;
    const QString r = px(std::max(t.radius, 2)), r2 = px(std::max(0, t.radius - 1)), pad = px(t.padding);
    auto g2 = [](const QColor& a, const QColor& b) { return QString("qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0 %1, stop:1 %2)").arg(a.name(), b.name()); };
    auto gloss = [](const QColor& base) {
        return QString("qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0 %1, stop:0.48 %2, stop:0.52 %3, stop:1 %4)")
            .arg(base.lighter(135).name(), base.lighter(112).name(), base.name(), base.darker(108).name());
    };
    const QString push = "QPushButton", tool = "QToolButton";
    const QString edge_dark = c["border"].name(), mid = c["shadow"].name();
    if (t.buttons == "soft") {
        s += push + " { background: " + g2(c["button"].lighter(116), c["button"].darker(112)) + "; border: 1px solid " + edge_dark + "; border-radius: " + r + "; padding: " + pad + " 12px; color: @text; border-image: none; }\n";
        s += push + ":hover { background: " + g2(c["hover"].lighter(118), c["hover"].darker(108)) + "; }\n";
        s += push + ":pressed, " + push + ":checked { background: " + g2(c["button"].darker(122), c["button"].darker(104)) + "; border-color: " + mid + "; }\n";
        s += tool + " { border: 1px solid transparent; border-radius: " + r + "; background: transparent; border-image: none; padding: 2px; }\n";
        s += tool + ":hover { background: " + g2(c["hover"].lighter(118), c["hover"].darker(108)) + "; border-color: " + edge_dark + "; }\n";
        s += tool + ":checked, " + tool + ":pressed { background: " + g2(c["button"].darker(125), c["button"].darker(106)) + "; border-color: " + edge_dark + "; color: @text; }\n";
    } else if (t.buttons == "glossy") {
        s += push + " { background: " + gloss(c["button"]) + "; border: 1px solid " + edge_dark + "; border-radius: " + r + "; padding: " + pad + " 12px; color: @text; border-image: none; }\n";
        s += push + ":hover { background: " + gloss(c["hover"]) + "; }\n";
        s += push + ":pressed, " + push + ":checked { background: " + gloss(c["hi"]) + "; color: @hitext; border-color: " + c["hi"].darker(140).name() + "; }\n";
        s += tool + " { border: 1px solid transparent; border-radius: " + r + "; background: transparent; border-image: none; padding: 2px; }\n";
        s += tool + ":hover { background: " + gloss(c["hover"]) + "; border-color: " + edge_dark + "; }\n";
        s += tool + ":checked, " + tool + ":pressed { background: " + gloss(c["hi"]) + "; border-color: " + c["hi"].darker(140).name() + "; color: @hitext; }\n";
    } else if (t.buttons == "outline") {
        s += push + " { background: transparent; border: " + px(std::max(1, t.border)) + " solid @dim; border-radius: " + r + "; padding: " + pad + " 12px; color: @text; border-image: none; }\n";
        s += push + ":hover { border-color: @hi; color: @bright; }\n";
        s += push + ":pressed, " + push + ":checked { background: transparent; border-color: @hi; color: @hi; }\n";
        s += tool + " { border: 1px solid transparent; border-radius: " + r + "; background: transparent; border-image: none; padding: 2px; }\n";
        s += tool + ":hover { border-color: @dim; background: transparent; }\n";
        s += tool + ":checked, " + tool + ":pressed { border-color: @hi; background: transparent; color: @hi; }\n";
    } else if (t.buttons == "inset") {
        s += push + " { background: @base; border: 1px solid @border; border-top: 2px solid @shadow; border-radius: " + r2 + "; padding: " + pad + " 12px; color: @text; border-image: none; }\n";
        s += push + ":hover { background: @alt; }\n";
        s += push + ":pressed, " + push + ":checked { background: @hi; color: @hitext; border-top-color: " + c["hi"].darker(150).name() + "; }\n";
        s += tool + " { border: 1px solid transparent; border-radius: " + r2 + "; background: transparent; border-image: none; padding: 2px; }\n";
        s += tool + ":hover { background: @base; border-color: @border; }\n";
        s += tool + ":checked, " + tool + ":pressed { background: @base; border: 1px solid @border; border-top: 2px solid @shadow; color: @hi; }\n";
    } else if (t.buttons == "chunky") {
        s += push + " { background: @button; border: 2px solid @bright; border-bottom-width: 4px; border-radius: " + r2 + "; padding: " + pad + " 12px; color: @bright; font-weight: bold; border-image: none; }\n";
        s += push + ":hover { background: @hover; }\n";
        s += push + ":pressed, " + push + ":checked { background: @hi; color: @hitext; border-bottom-width: 2px; margin-top: 2px; }\n";
        s += tool + " { border: 2px solid transparent; border-radius: " + r2 + "; background: transparent; border-image: none; padding: 1px; }\n";
        s += tool + ":hover { border-color: @bright; background: @hover; }\n";
        s += tool + ":checked, " + tool + ":pressed { background: @hi; border-color: @bright; color: @hitext; }\n";
    } else if (t.buttons == "underline") {
        s += push + " { background: transparent; border: none; border-bottom: 2px solid @handle; border-radius: 0; padding: " + pad + " 10px; color: @text; border-image: none; }\n";
        s += push + ":hover { border-bottom-color: @hi; color: @bright; }\n";
        s += push + ":pressed, " + push + ":checked { border-bottom: 3px solid @hi; color: @hi; background: transparent; }\n";
        s += tool + " { border: none; border-bottom: 2px solid transparent; border-radius: 0; background: transparent; border-image: none; padding: 2px; }\n";
        s += tool + ":hover { border-bottom-color: @dim; background: transparent; }\n";
        s += tool + ":checked, " + tool + ":pressed { border-bottom-color: @hi; background: transparent; }\n";
    } else if (t.buttons == "pill") {
        s += push + " { background: @button; border: none; border-radius: 12px; padding: " + pad + " 16px; color: @text; border-image: none; }\n";
        s += push + ":hover { background: @hover; }\n";
        s += push + ":pressed, " + push + ":checked { background: @hi; color: @hitext; }\n";
        s += tool + " { border: none; border-radius: 11px; background: transparent; border-image: none; padding: 3px; }\n";
        s += tool + ":hover { background: @hover; }\n";
        s += tool + ":checked, " + tool + ":pressed { background: @hi; color: @hitext; }\n";
    }
    if (t.buttons != "auto") {
        // The Attribute Editor's and tool settings' section bars stay bars, and the feedback button its own colour.
        s += "QPushButton#feelButton { border-image: none; }\n";
    }

    // Edges round panels, and the gaps between them.
    const QString content = "QDockWidget > QWidget";
    if (t.frames == "line") {
        s += content + " { border: 1px solid @border; }\n";
        s += "QMainWindow::separator { background: @window; width: 3px; height: 3px; }\n";
    } else if (t.frames == "groove") {
        s += content + " { border: 2px groove @light; }\n";
        s += "QMainWindow::separator { background: @window; width: 4px; height: 4px; }\n";
    } else if (t.frames == "bevel") {
        s += content + " { border: 2px solid; border-color: @light @shadow @shadow @light; }\n";
        s += "QMainWindow::separator { background: @chrome; width: 5px; height: 5px; }\n";
    } else if (t.frames == "thick") {
        s += content + " { border: 3px solid @section; }\n";
        s += "QMainWindow::separator { background: @base; width: 4px; height: 4px; }\n";
    } else if (t.frames == "rounded") {
        s += content + " { border: 1px solid @border; border-radius: 8px; background: @window; }\n";
        s += "QMainWindow::separator { background: @chrome; width: 7px; height: 7px; }\n";
    } else if (t.frames == "shadow") {
        s += content + " { border: 1px solid @border; border-right: 3px solid @shadow; border-bottom: 3px solid @shadow; }\n";
        s += "QMainWindow::separator { background: @chrome; width: 6px; height: 6px; }\n";
    }
    if (t.frames != "none") s += "QMainWindow::separator:hover { background: @hi; }\n";

    // Panel title bars.
    const QString title = "QDockWidget::title";
    if (t.titles == "gradient") {
        s += title + " { background: " + g2(c["chrome"].lighter(125), c["chrome"].darker(115)) + "; border: 1px solid @border; border-image: none; padding: 3px 6px; }\n";
        s += "QDockWidget { font-weight: bold; }\n";
    } else if (t.titles == "underline") {
        s += title + " { background: transparent; border: none; border-bottom: 2px solid @hi; border-image: none; padding: 3px 4px 1px 4px; }\n";
    } else if (t.titles == "accent") {
        s += title + " { background: @chrome; border: none; border-left: 4px solid @hi; border-image: none; padding: 3px 8px; }\n";
    } else if (t.titles == "inverse") {
        s += title + " { background: @bright; border: none; border-image: none; padding: 3px 8px; }\n";
        s += "QDockWidget { color: @window; font-weight: bold; }\n";
    } else if (t.titles == "tab") {
        s += title + " { background: @section; border: 1px solid @border; border-bottom: none; border-top-left-radius: 7px; border-top-right-radius: 7px; border-image: none; padding: 3px 10px; }\n";
        s += "QDockWidget { color: @sectiontext; }\n";
    }
    return s;
}

QString fill_tokens(QString qss, const Theme& t) {
    // Longest names first, so @hi doesn't eat @hitext's or @hover's start.
    QStringList keys = t.c.keys();
    std::sort(keys.begin(), keys.end(), [](const QString& a, const QString& b) { return a.size() > b.size(); });
    for (const QString& k : keys) qss.replace("@" + k, t.c[k].name());
    return qss;
}

void apply(QApplication& app, const Theme& t) {
    app.setStyle(QStyleFactory::create("Fusion"));
    const auto& c = t.c;
    QPalette p;
    p.setColor(QPalette::Window, c["window"]);
    p.setColor(QPalette::WindowText, c["text"]);
    p.setColor(QPalette::Base, c["base"]);
    p.setColor(QPalette::AlternateBase, c["alt"]);
    p.setColor(QPalette::ToolTipBase, t.fill == "classic" ? QColor(255, 255, 220) : c["menu"]);
    p.setColor(QPalette::ToolTipText, t.fill == "classic" ? QColor(Qt::black) : c["bright"]);
    p.setColor(QPalette::Text, c["text"]);
    p.setColor(QPalette::Button, c["button"]);
    p.setColor(QPalette::ButtonText, c["text"]);
    p.setColor(QPalette::BrightText, t.dark() ? QColor(Qt::white) : QColor(Qt::black));
    p.setColor(QPalette::Highlight, c["hi"]);
    p.setColor(QPalette::HighlightedText, c["hitext"]);
    p.setColor(QPalette::Link, t.dark() ? c["hi"].lighter(130) : c["hi"]);
    p.setColor(QPalette::PlaceholderText, c["disabled"]);
    p.setColor(QPalette::Light, c["light"]);
    p.setColor(QPalette::Midlight, c["hover"]);
    p.setColor(QPalette::Mid, c["handle"]);
    p.setColor(QPalette::Dark, c["shadow"]);
    p.setColor(QPalette::Shadow, c["border"]);
    for (auto role : {QPalette::Text, QPalette::WindowText, QPalette::ButtonText}) p.setColor(QPalette::Disabled, role, c["disabled"]);
    app.setPalette(p);
    QFont f = QApplication::font();
    static const QFont system = f;
    if (t.font.isEmpty()) {
        f = system;
    } else {
        f = system;
        f.setFamilies(t.font);
        if (t.point_size > 0) f.setPointSize(t.point_size);
    }
    app.setFont(f);
    QFile qss(":/style.qss");
    const QString base = qss.open(QIODevice::ReadOnly) ? QString::fromUtf8(qss.readAll()) : QString();
    const QString assets = write_assets(t);
    app.setStyleSheet(fill_tokens(base + "\n" + skin(t, assets) + shapes(t), t));
    for (QWidget* w : QApplication::allWidgets()) w->update();
}

} // namespace

bool Theme::dark() const { return lightness() < 0.5; }

double Theme::lightness() const {
    const QColor w = c.value("window");
    return (0.2126 * w.redF() + 0.7152 * w.greenF() + 0.0722 * w.blueF());
}

QJsonObject Theme::to_json() const {
    QJsonObject o;
    o["id"] = id;
    o["name"] = name;
    QJsonObject cols;
    for (auto it = c.begin(); it != c.end(); ++it) cols[it.key()] = it.value().name();
    o["colours"] = cols;
    o["font"] = QJsonArray::fromStringList(font);
    o["pointSize"] = point_size;
    o["menuItalic"] = menu_italic;
    o["menuBold"] = menu_bold;
    o["fill"] = fill;
    o["bevel"] = bevel;
    o["radius"] = radius;
    o["border"] = border;
    o["tabs"] = tabs;
    o["arrows"] = arrows;
    o["scrollbar"] = scrollbar;
    o["padding"] = padding;
    o["buttons"] = buttons;
    o["frames"] = frames;
    o["titles"] = titles;
    return o;
}

Theme Theme::from_json(const QJsonObject& o) {
    // Anything missing comes from Aztek Classic, so an old or hand-written file still makes a whole theme.
    Theme t = builtins().front();
    t.builtin = false;
    t.id = o["id"].toString();
    t.name = o["name"].toString(t.id);
    const QJsonObject cols = o["colours"].toObject();
    for (auto it = cols.begin(); it != cols.end(); ++it) {
        const QColor col(it.value().toString());
        if (col.isValid()) t.c[it.key()] = col;
    }
    t.font.clear();
    for (const QJsonValue& v : o["font"].toArray()) t.font << v.toString();
    t.point_size = o["pointSize"].toInt(0);
    t.menu_italic = o["menuItalic"].toBool(false);
    t.menu_bold = o["menuBold"].toBool(false);
    t.fill = o["fill"].toString("flat");
    t.bevel = o["bevel"].toString("none");
    t.radius = o["radius"].toInt(4);
    t.border = o["border"].toInt(1);
    t.tabs = o["tabs"].toString("boxed");
    t.arrows = o["arrows"].toString("chevron");
    t.scrollbar = o["scrollbar"].toString("thin");
    t.padding = o["padding"].toInt(3);
    t.buttons = o["buttons"].toString("auto");
    t.frames = o["frames"].toString("none");
    t.titles = o["titles"].toString("auto");
    return t;
}

Theme random_theme() {
    auto* r = QRandomGenerator::global();
    auto pick = [r](const QStringList& l) { return l.at(int(r->bounded(l.size()))); };
    auto between = [r](double lo, double hi) { return lo + r->generateDouble() * (hi - lo); };
    // The base: a hue, how much of it, dark or light; the accent anywhere on the wheel, strong. Text keeps its contrast
    // whatever comes up, as tonal() works it out from the lightness.
    const bool dark = r->bounded(100) < 60;
    const double hue = between(0, 360), sat = r->bounded(100) < 30 ? 0.0 : between(0.03, 0.45);
    const double light = dark ? between(6, 34) : between(62, 96);
    const QColor accent = QColor::fromHslF(between(0, 1), between(0.55, 0.95), between(0.42, 0.58));
    Theme t = tonal("random", "Random", hue, sat, light, accent.name().toLatin1().constData());
    // Now and then the section bars take the accent, or a second colour of their own.
    switch (r->bounded(3)) {
    case 0:
        t.c["section"] = accent;
        t.c["sectiontext"] = accent.lightnessF() > 0.55 ? QColor("#000000") : QColor("#ffffff");
        break;
    case 1: {
        const QColor second = QColor::fromHslF(between(0, 1), between(0.4, 0.9), dark ? between(0.25, 0.4) : between(0.6, 0.78));
        t.c["section"] = second;
        t.c["sectiontext"] = second.lightnessF() > 0.55 ? QColor("#000000") : QColor("#ffffff");
        break;
    }
    default:
        break;
    }
    t.c["hitext"] = accent.lightnessF() > 0.6 ? QColor("#000000") : QColor("#ffffff");
    // A font any of the system's that writes Latin and isn't symbols, or now and then the system's own.
    if (r->bounded(100) < 85) {
        QStringList fonts;
        for (const QString& f : QFontDatabase::families(QFontDatabase::Latin)) {
            const QString low = f.toLower();
            if (low.contains("symbol") || low.contains("emoji") || low.contains("dings") || low.contains("icon") || low.contains("awesome") || low.contains("math")) continue;
            fonts << f;
        }
        if (!fonts.isEmpty()) t.font = QStringList{pick(fonts)};
        t.point_size = 7 + int(r->bounded(5));
    } else {
        t.font.clear();
        t.point_size = 0;
    }
    t.menu_italic = r->bounded(100) < 25;
    t.menu_bold = r->bounded(100) < 35;
    t.fill = pick({"classic", "flat", "flat", "gradient", "gradient"});
    t.bevel = pick({"none", "none", "single", "double"});
    t.radius = int(r->bounded(15));
    t.border = int(r->bounded(4));
    t.padding = int(r->bounded(9));
    t.tabs = pick({"boxed", "underline", "pill", "rounded"});
    t.arrows = pick({"chevron", "triangle", "boxed"});
    t.scrollbar = pick({"thin", "standard", "bevel"});
    t.buttons = pick({"auto", "soft", "glossy", "outline", "inset", "chunky", "underline", "pill"});
    t.frames = pick({"none", "line", "groove", "bevel", "thick", "rounded", "shadow"});
    t.titles = pick({"auto", "gradient", "underline", "accent", "inverse", "tab"});
    t.builtin = false;
    return t;
}

void apply_random_theme(QApplication& app) {
    Theme t = random_theme();
    // Kept as a theme of the user's, so it stays after a restart and can be opened in the editor and saved under a name.
    save_user_theme(t, nullptr);
    apply_theme(app, t.id);
}

const QList<QPair<QString, QString>>& colour_keys() {
    static const QList<QPair<QString, QString>> k = {
        {"window", "Window"}, {"chrome", "Bars and docks"}, {"base", "Lists and views"}, {"alt", "Alternate rows"}, {"menu", "Menus"},
        {"button", "Buttons"}, {"hover", "Under the pointer"}, {"hi", "Highlight"}, {"hitext", "Highlighted text"}, {"text", "Text"},
        {"bright", "Bright text"}, {"dim", "Dim text"}, {"disabled", "Disabled text"}, {"field", "Fields"}, {"fieldtext", "Field text"},
        {"section", "Section bars"}, {"sectiontext", "Section text"}, {"border", "Borders"}, {"handle", "Handles"}, {"light", "Bevel light"},
        {"shadow", "Bevel shadow"},
    };
    return k;
}

QList<Theme> themes() {
    QList<Theme> l = builtins();
    l += user_themes();
    // Aztek Classic first, then darkest to lightest.
    std::stable_sort(l.begin() + 1, l.end(), [](const Theme& a, const Theme& b) { return a.lightness() < b.lightness(); });
    return l;
}

Theme theme(const QString& id) {
    const QList<Theme> l = themes();
    for (const Theme& t : l)
        if (t.id == id) return t;
    return l.front();
}

QString current_theme() { return QSettings().value("prefs/theme", "classic").toString(); }

QString themes_folder() { return prefs_root() + "/themes"; }

void apply_theme(QApplication& app, const QString& id) {
    const Theme t = theme(id);
    QSettings().setValue("prefs/theme", t.id);
    apply(app, t);
    changed();
}

void preview_theme(QApplication& app, const Theme& t) { apply(app, t); }

void apply_saved_theme(QApplication& app) { apply_theme(app, current_theme()); }

bool save_user_theme(const Theme& t, QString* error) {
    QDir().mkpath(themes_folder());
    QFile f(themes_folder() + "/" + t.id + ".json");
    if (!f.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        if (error) *error = f.errorString();
        return false;
    }
    f.write(QJsonDocument(t.to_json()).toJson());
    changed();
    return true;
}

bool delete_user_theme(const QString& id) {
    const bool ok = QFile::remove(themes_folder() + "/" + id + ".json");
    if (ok) changed();
    return ok;
}

void on_themes_changed(std::function<void()> f) { listeners().push_back(std::move(f)); }

QComboBox* make_theme_box(QWidget* parent) {
    auto* box = new QComboBox(parent);
    box->setObjectName("themeBox");
    box->setToolTip("Theme: the interface's colours, fonts and shapes");
    box->setSizeAdjustPolicy(QComboBox::AdjustToContents);
    box->setFocusPolicy(Qt::NoFocus);
    auto fill = [box] {
        QSignalBlocker block(box);
        box->clear();
        const QList<Theme> l = themes();
        const QString now = current_theme();
        bool light_started = false;
        for (int i = 0; i < l.size(); ++i) {
            const Theme& t = l[i];
            if (i == 1 || (!t.dark() && !light_started && i > 0)) {
                if (!t.dark()) light_started = true;
                box->insertSeparator(box->count());
            }
            box->addItem(t.builtin ? t.name : t.name + "  (yours)", t.id);
            if (t.id == now) box->setCurrentIndex(box->count() - 1);
        }
        box->insertSeparator(box->count());
        box->addItem("Random!", QStringLiteral("#random"));
        box->setItemData(box->count() - 1, "Every option at random, fonts and all: pick it again for another", Qt::ToolTipRole);
        box->addItem("Theme Editor...", QStringLiteral("#editor"));
    };
    fill();
    on_themes_changed(fill);
    QObject::connect(box, &QComboBox::activated, box, [box](int i) {
        const QString id = box->itemData(i).toString();
        if (id == "#random") {
            apply_random_theme(*qApp);
            return;
        }
        if (id == "#editor") {
            const QString now = current_theme();
            for (int k = 0; k < box->count(); ++k)
                if (box->itemData(k).toString() == now) box->setCurrentIndex(k);
            show_theme_editor(box->window());
            return;
        }
        apply_theme(*qApp, id);
    });
    return box;
}

} // namespace az

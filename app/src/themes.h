// Themes: the interface's colours, font and button shapes. Not in the reference, which has one look; Aztek Classic is
// that look and the default. Chosen from the Themes button on the status line, kept between sessions.
#pragma once
#include <QColor>
#include <QHash>
#include <QList>
#include <QString>

class QApplication;
class QMenu;

namespace az {

struct Theme {
    QString id;
    QString name;
    bool dark = true;
    // window, chrome, base, alt, menu, button, hover, hi, text, bright, dim, disabled, border, handle
    QHash<QString, QColor> c;
    // Font families in order of preference, size in points; empty keeps the system's.
    QStringList font;
    int point_size = 0;
    // classic: the reference's square Fusion buttons; rounded: soft corners and flat fills.
    QString buttons = "classic";
    int radius = 0;
};

const QList<Theme>& themes();
QString current_theme();
// Applies a theme to the whole application and remembers it.
void apply_theme(QApplication& app, const QString& id);
// The saved theme, or Aztek Classic.
void apply_saved_theme(QApplication& app);
// The Themes button's menu: dark themes, then light, the current one ticked.
void fill_theme_menu(QMenu* m);

} // namespace az

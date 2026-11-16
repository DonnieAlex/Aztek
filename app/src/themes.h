// Themes: the interface's colours, fonts and the shapes of its buttons, fields, tabs, arrows and scroll bars. Not in the
// reference, which has one look; Aztek Classic is that look and the default. Chosen from the Themes box at the top right
// of the menu bar, edited in the Theme Editor, kept between sessions. Themes made in the editor are JSON files in the
// themes folder of Aztek's preferences (~/.aztek/themes, or AZTEK_PREFS/themes).
#pragma once
#include <QColor>
#include <QHash>
#include <QJsonObject>
#include <QList>
#include <QString>
#include <QStringList>
#include <functional>

class QApplication;
class QComboBox;
class QWidget;

namespace az {

struct Theme {
    QString id;
    QString name;
    bool builtin = true;
    // The colours by part: see colour_keys().
    QHash<QString, QColor> c;
    // Font family, in order of preference; empty keeps the system's. Size in points, 0 keeps the system's.
    QStringList font;
    int point_size = 0;
    // Menu bar, menus and tabs in italic or bold, as the old workstation toolkits had them.
    bool menu_italic = false;
    bool menu_bold = false;
    // classic: the reference's own look, Qt's Fusion with the colours. flat or gradient: the fill of buttons and bars.
    QString fill = "flat";
    // none, single (one light and one dark edge, Motif's) or double (two of each, the 1998 desktop's).
    QString bevel = "none";
    int radius = 4;
    int border = 1;
    // boxed, underline, pill or rounded (raised tabs with rounded tops).
    QString tabs = "boxed";
    // chevron, triangle, or boxed (a raised button round a triangle).
    QString arrows = "chevron";
    // thin (no arrows), standard, or bevel (raised handle and arrow buttons).
    QString scrollbar = "thin";
    // Space inside buttons and fields, in pixels.
    int padding = 3;

    bool dark() const;
    // How light the window colour is, 0 to 1: themes are listed by it.
    double lightness() const;
    QJsonObject to_json() const;
    static Theme from_json(const QJsonObject& o);
};

// The colour parts with a label each, in the editor's order.
const QList<QPair<QString, QString>>& colour_keys();
// Built-in themes and the ones saved from the editor, lightest last.
QList<Theme> themes();
Theme theme(const QString& id);
QString current_theme();
// Applies a theme to the whole application and remembers it.
void apply_theme(QApplication& app, const QString& id);
// Applies a theme without remembering it: the editor's preview.
void preview_theme(QApplication& app, const Theme& t);
void apply_saved_theme(QApplication& app);
// Every option at random, fonts included, still readable: Random! in the Themes box and the Theme Editor's Randomize.
Theme random_theme();
// A random theme, kept as the user's "Random" theme and applied.
void apply_random_theme(QApplication& app);
// The folder user themes are kept in.
QString themes_folder();
// Saves a theme made in the editor; returns false with a message when it can't.
bool save_user_theme(const Theme& t, QString* error);
bool delete_user_theme(const QString& id);
// Called after any theme is applied or the list changes.
void on_themes_changed(std::function<void()> f);
// The Themes box: themes by lightness, dark then light, and the Theme Editor.
QComboBox* make_theme_box(QWidget* parent);
void show_theme_editor(QWidget* parent);

} // namespace az

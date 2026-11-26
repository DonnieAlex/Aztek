// The shelves: tabs of buttons, each a script. Saved between sessions; edited in the Shelf Editor or by dragging scripts
// and menu items on.
#pragma once
#include <QJsonArray>
#include <QJsonObject>
#include <QWidget>
#include <functional>

class QTabWidget;

namespace az {

class ShelfBar : public QWidget {
    Q_OBJECT
public:
    // run(command, language): language is "mel" or "python".
    explicit ShelfBar(std::function<void(const QString&, const QString&)> run, QWidget* parent = nullptr);
    void add_to_current(const QString& label, const QString& icon_name, const QString& command, const QString& language);
    // The shelf tab of this name in front, if there is one (a workspace's shelf).
    void select_shelf(const QString& name);
    void open_editor();
    // For scripts ($gShelfTopLevel): the shelves' names, the one in front, and buttons scripts put on a shelf, kept by
    // the script's name for the button. A button whose command is a script function runs through fire_script; one
    // with a command in text stays on the shelf between sessions.
    QStringList shelf_names() const;
    int current() const;
    void add_script_button(const QString& shelf, const QString& path, const QJsonObject& props);
    void remove_script_button(const QString& path);
    std::function<void(const QString& path)> fire_script;
signals:
    void shelves_changed();
public:
    // A widget at the far right of the tab row. Tabs that don't fit scroll; it stays.
    void set_corner(QWidget* w);
protected:
    void resizeEvent(QResizeEvent* e) override;
    bool eventFilter(QObject* o, QEvent* e) override;

public:
    // Reads the reference's shelf_Name.mel into a new shelf: its shelfButton blocks.
    void import_mel(const QString& path);
    static bool read_mel(const QString& path, QJsonObject* shelf);
    // Writes a shelf as the reference's shelf_Name.mel: a global proc of shelfButton calls.
    static bool write_mel(const QString& path, const QJsonObject& shelf);
    void rebuild();
    void save() const;
    QJsonArray& data() { return shelves_; }
    static QJsonArray defaults();
private:
    void tab_menu(QPoint global, int index);
    void button_menu(QPoint global, int shelf, int item);
    void edit_item(int shelf, int item);
    std::function<void(const QString&, const QString&)> run_;
    QWidget* corner_ = nullptr;
    QWidget* corner_gap_ = nullptr;
    void place_corner();
    QTabWidget* tabs_;
    QJsonArray shelves_;
    friend class ShelfPage;
};

} // namespace az

// The docked editors: outliner, channel box with the layer editor, attribute editor, script editor.
#pragma once
#include <QJsonObject>
#include <QHash>
#include <QPointer>
#include <QSet>
#include <QWidget>
#include <functional>

class QDockWidget;
class QTreeWidget;
class QTreeWidgetItem;
class QPlainTextEdit;
class QTabWidget;
class QTabBar;
class QStackedWidget;
class QVBoxLayout;
class QScrollArea;
class QLineEdit;
class QLabel;
class QToolButton;
class QMenu;
class QCheckBox;
class QSlider;
class QDoubleSpinBox;
class QAction;
class QDialog;
class QSplitter;

namespace az {

// Runs refresh() once after the scene settles rather than on every change.
class Panel : public QWidget {
    Q_OBJECT
public:
    explicit Panel(QWidget* parent = nullptr);
protected:
    virtual void refresh() = 0;
    // False for a panel slow to rebuild: while a button is held over a view (a drag), it waits for the release.
    virtual bool live_during_drag() const { return true; }
    void schedule();
    void showEvent(QShowEvent*) override;
private:
    bool pending_ = false;
    bool waiting_for_release_ = false;
    // Looks taken since it showed with nothing of it on screen yet.
    int retries_ = 0;
    // The dock whose tab coming to the front refreshes it.
    QPointer<QDockWidget> watched_dock_;
};

class Outliner : public Panel {
    Q_OBJECT
public:
    explicit Outliner(QWidget* parent = nullptr);
    // What outlinerEditor's flags say: -showShapes, -showDagOnly, -showAttributes, -mainListConnection, -setFilter.
    void set_editor_options(const QJsonObject& props);
protected:
    void refresh() override;
    // Nothing it lists changes during a drag, and rebuilding a big scene's tree at every step costs the view frames.
    bool live_during_drag() const override { return false; }
private:
    void add(QTreeWidgetItem* parent, const QJsonObject& node);
    void add_prims(QTreeWidgetItem* parent, const QString& stage, const QString& prim, const QString& key);
    void selection_changed();
    QTreeWidget* tree_;
    bool shapes_ = false;
    bool intermediates_ = false;
    bool updating_ = false;
    QSet<QString> expanded_;
    QLineEdit* filter_;
    // On, as the reference starts: off lists every node, not only the DAG and the sets.
    bool dag_only_ = true;
    bool attrs_ = false;
    bool hide_cameras_ = false;
    bool show_icons_ = true;
    // A selectionConnection or itemFilter limiting what shows; empty for everything.
    QString list_connection_, item_filter_;
    void apply_list_limit();
    QSet<QString> hidden_types_;
    // The reference's Display menu: what the list holds and how it reads.
    bool materials_ = false, ref_members_ = true, set_members_ = true, connected_ = false, namespaces_ = true;
    bool ignore_hidden_ = false, ignore_color_ = false, sort_alpha_ = false;
    QString channel_names_ = "nice";
    int attr_order_ = 0;
    // The reference's Show menu: kinds of object and of attribute to show (none: all), and the other way round.
    QSet<QString> show_kinds_, show_attrs_;
    bool invert_ = false;
    void fill_display_menu(QMenu* m);
    void fill_show_menu(QMenu* m);
    void fill_help_menu(QMenu* m);
    void apply_show_filter();
    QStringList attribute_rows(const QString& path);
    // What the tree was last built from, to tell a change of selection from one of the tree.
    QByteArray last_shape_;
    QString last_view_;
};

// The channels picked in the channel box, by attribute name (translateX...).
QStringList selected_channels();
// The picked channels that are a history node's, as whole plugs: they don't follow the selected object.
QStringList selected_input_plugs();
void clear_channel_selection();

class ChannelBox : public Panel {
    Q_OBJECT
public:
    // name: the channelBox control it is (mainChannelBox for the application's own).
    explicit ChannelBox(QWidget* parent = nullptr, const QString& name = "mainChannelBox");
    // What channelBox -e changed: -takeFocus, and anything else draws it again.
    void apply_flags(const QJsonObject& props);
    const QString& box_name() const { return name_; }
protected:
    void refresh() override;
private:
    void add_channels(QVBoxLayout* lay, const QJsonArray& channels, int label_width = 0);
    void add_nodes(QVBoxLayout* lay, const QString& heading, const QJsonArray& nodes, int label_width = 0);
    QScrollArea* scroll_;
    QWidget* body_ = nullptr;
    QSet<QString> open_inputs_;
    QString name_;
    // The reference's three buttons above the channels: manipulators, slider speed, hyperbolic.
    QToolButton *manip_button_ = nullptr, *speed_button_ = nullptr, *hyperbolic_button_ = nullptr;
    // The settings channelBox gave this box (labelWidth, fieldWidth...).
    QJsonObject options_;
};

class LayerEditor : public Panel {
    Q_OBJECT
public:
    explicit LayerEditor(QWidget* parent = nullptr);
protected:
    void refresh() override;
private:
    QTreeWidget* list_;
};

// The Anim tab: the animation layer stack.
class AnimLayerEditor : public Panel {
    Q_OBJECT
public:
    explicit AnimLayerEditor(QWidget* parent = nullptr);
protected:
    void refresh() override;
private:
    QWidget* make_row(const QJsonObject& layer);
    QString current() const;
    void update_weight();
    void create(bool from_selected, bool override_mode);
    void zero_key();
    void key_weight(const QString& layer);
    void for_selected(const std::function<void(const QString&)>& f);
    void merge_options();
    void build_layers_menu(QMenu* m);
    void build_options_menu(QMenu* m);
    void build_show_menu(QMenu* m);
    QTreeWidget* list_ = nullptr;
    // The reference's weight controls under the tree, for the current layer.
    QSlider* weight_slider_ = nullptr;
    QDoubleSpinBox* weight_spin_ = nullptr;
    // A weight dragged or typed, held in the controls until the layer or time changes or K records it.
    QString weight_pending_layer_;
    double weight_pending_value_ = -1.0;
    double weight_pending_time_ = 0.0;
    // The top of the stack: its ghost button is hidden, as the reference's.
    QString top_layer_;
    bool updating_ = false;
    bool only_affected_ = false;
};

class AttributeEditor : public Panel {
    Q_OBJECT
protected:
    // Rebuilding every field costs a tenth of a second: not at every step of a drag.
    bool live_during_drag() const override { return false; }
public:
    // pinned: stays on this node (Copy Tab); empty follows the selection.
    explicit AttributeEditor(QWidget* parent = nullptr, const QString& pinned = QString());
protected:
    void refresh() override;
    bool eventFilter(QObject* o, QEvent* e) override;
private:
    QTabWidget* tabs_;
    QString current_tab_;
    QString pinned_;
    // What the tabs were last built from: the same again (a change somewhere else) leaves them be.
    QByteArray last_;
};

class ScriptEditor : public QWidget {
    Q_OBJECT
public:
    explicit ScriptEditor(QWidget* parent = nullptr);
    ~ScriptEditor() override;
    void append(const QJsonArray& lines);
    // scriptEditorInfo -clearHistory and -input.
    void clear_history();
    void set_input(const QString& text);
    void run(bool clear_after);
    // Opens a .mel or .py file in a tab of its own.
    void load_file(const QString& path);
protected:
    bool eventFilter(QObject* o, QEvent* e) override;
    void showEvent(QShowEvent* e) override;
private:
    void add_tab(const QString& language, const QString& name, const QString& text, const QString& path = QString());
    void rename_tab(int i);
    void close_tab(int i);
    void save_current(bool as);
    void store_tabs();
    QPlainTextEdit* current() const;
    // Ctrl+F focuses the search field; Edit > Search and Replace... opens the dialog.
    void open_search_replace();
    void find(bool back);
    // Replaces the match under the cursor (and finds the next), or every match; returns how many.
    int replace(bool all);
    // The history's lines coloured again for the theme in use (#88).
    void recolour_history();
    QPlainTextEdit* history_;
    QWidget* input_pane_ = nullptr;
    QTabBar* tab_bar_ = nullptr;
    QStackedWidget* stack_ = nullptr;
    QList<QWidget*> pages_;
    QLineEdit* find_ = nullptr;
    QLineEdit* replace_ = nullptr;
    QCheckBox* find_case_ = nullptr;
    QCheckBox* find_down_ = nullptr;
    QCheckBox* find_wrap_ = nullptr;
    QDialog* find_dialog_ = nullptr;
    QPlainTextEdit* find_target_ = nullptr;
    bool show_numbers_ = true;
    bool stack_trace_ = true;
    bool echo_all_ = false;
    // The history, the input, or both: the Show History, Show Input and Show Both buttons.
    QSplitter* split_ = nullptr;
    QWidget* history_pane_ = nullptr;
    QLineEdit* search_field_ = nullptr;
    QLineEdit* goto_field_ = nullptr;
    QHash<QString, QAction*> option_actions_;
    int pane_mode_ = 2;
    QList<int> both_sizes_ = {315, 313};
    bool split_sized_ = false;
    void set_pane_mode(int mode);
    QPlainTextEdit* search_pane() const;
    void run_all();
    // The reference's menus and marking menus (#133).
    void new_tab_prompt();
    void step_tab(int by);
    void delete_other_tabs();
    void save_all_tabs();
    void save_to_shelf();
    void goto_line();
    void goto_line_number(int line);
    void indent(bool in);
    void quick_help(bool documentation);
    void marking_menu(QPlainTextEdit* pane, QPoint global);
    void apply_editor_options();
    bool option(const QString& key, bool fallback) const;
    QString word_at_cursor() const;
};

} // namespace az

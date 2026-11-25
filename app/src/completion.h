// The Script Editor's completion popup and its index: commands and their flags, MEL procedures, and the Python
// modules and classes Aztek ships. One popup per Script Editor, shown at the caret of whichever input tab has focus.
#pragma once

#include <QHash>
#include <QJsonObject>
#include <QPointer>
#include <QString>
#include <QStringList>
#include <QVector>
#include <QWidget>

class QKeyEvent;
class QMouseEvent;
class QPlainTextEdit;
class QWheelEvent;

namespace az {

// One entry in the list. `insert` replaces the typed prefix; `detail` is the signature line and `doc` the description
// under the list.
struct CompletionEntry {
    enum Kind { Module, Class, Function, Method, Flag, Property, Constant, Command, Procedure };
    QString label;
    QString insert;
    QString detail;
    QString doc;
    QString category;
    Kind kind = Function;
    bool enabled = true;
};

class ScriptCompletion : public QWidget {
    Q_OBJECT
public:
    explicit ScriptCompletion(QWidget* host);
    ~ScriptCompletion() override;

    void set_enabled(bool on);
    bool enabled() const { return enabled_; }
    bool popup_visible() const { return isVisible(); }
    // True when the popup took the key. The editor calls this before its own key handling.
    bool key_press(QKeyEvent* e, QPlainTextEdit* editor);
    // Recomputes the context after the editor has handled an edit or a caret move.
    void refresh(QPlainTextEdit* editor);
    void dismiss();

protected:
    void paintEvent(QPaintEvent* e) override;
    void mouseMoveEvent(QMouseEvent* e) override;
    void mousePressEvent(QMouseEvent* e) override;
    void wheelEvent(QWheelEvent* e) override;
    bool eventFilter(QObject* o, QEvent* e) override;

private:
    struct Flag {
        QString name, shortName, type, doc;
    };
    struct Command {
        QString name, doc, positional;
        QVector<Flag> flags;
    };
    struct Proc {
        QString name, signature, doc;
    };
    struct PyEntry {
        QString name, kind, signature, doc;
    };
    struct PyModule {
        QString name, doc;
        QVector<PyEntry> entries;
        QStringList children;
    };
    struct PyMember {
        QString name, kind, signature, doc;
        bool inherited = false;
    };
    struct PyClass {
        QString name, module, doc, init;
        QVector<PyMember> members;
        bool loaded = false;
    };
    struct Analysis {
        enum Mode { None, Words, Members, Flags, Hint } mode = None;
        QString prefix;
        QString base;   // the module or class key for Members
        int replace_start = -1;
        int replace_end = -1;
        const Command* command = nullptr;
        bool python = false;
        QString hint_name;
        QStringList hint_params;
        int hint_current = -1;
        bool offer_entries = true;
    };

    void ensure_index();
    void ensure_python();
    bool python_loaded_ = false;
    void load_python(const QJsonObject& py);
    const PyClass* py_class(const QString& module, const QString& name);
    Analysis analyse(QPlainTextEdit* editor);
    void build_candidates(const Analysis& a);
    void add_module_members(const QString& module);
    void add_class_members(const QString& module, const QString& name);
    void add_command_candidates(bool python);
    void add_flag_candidates(const Command& c, bool python);
    void apply_filter();
    void layout_popup(int max_height);
    void place();
    void move_selection(int by);
    void accept();
    void paint_entry(QPainter& p, const QRect& r, const CompletionEntry& e, bool selected) const;
    void paint_icon(QPainter& p, const QRect& r, CompletionEntry::Kind kind, bool enabled, bool selected) const;
    void paint_hint(QPainter& p, const QRect& r) const;
    QString aliased(const QString& expression, QPlainTextEdit* editor) const;
    QString python_signature(const Command& c) const;
    int detail_height() const;
    int hint_height() const { return hint_ ? 22 : 0; }
    int row_at(const QPoint& pos) const;

    QWidget* host_;
    QPointer<QPlainTextEdit> editor_;
    bool enabled_ = false;
    bool loaded_ = false;
    bool hint_ = false;
    QString hint_name_;
    QStringList hint_params_;
    int hint_current_ = -1;
    int replace_start_ = -1;
    int replace_end_ = -1;
    int selected_ = 0;
    int scroll_ = 0;
    int visible_rows_ = 1;
    bool place_above_ = false;
    bool preserve_order_ = false;
    QString applied_key_;
    QString applied_prefix_;

    QHash<QString, Command> commands_;
    QHash<QString, Proc> procs_;
    QStringList builtins_;
    QHash<QString, PyModule> py_modules_;
    QHash<QString, PyClass> py_classes_;
    QString py_error_;

    QVector<CompletionEntry> base_;
    QVector<CompletionEntry> entries_;
};

} // namespace az

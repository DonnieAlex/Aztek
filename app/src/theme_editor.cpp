// The Theme Editor: every part of a theme, seen on the whole interface as it is changed. Built-in themes stay as they
// are; saving one makes a copy of your own.
#include "themes.h"

#include <QApplication>
#include <QCloseEvent>
#include <QFileInfo>
#include <QCheckBox>
#include <QColorDialog>
#include <QComboBox>
#include <QDialog>
#include <QDialogButtonBox>
#include <QFileDialog>
#include <QFontComboBox>
#include <QFormLayout>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonDocument>
#include <QIcon>
#include <QLabel>
#include <QPainter>
#include <QPixmap>
#include <QLineEdit>
#include <QListWidget>
#include <QMessageBox>
#include <QPointer>
#include <QPushButton>
#include <QScrollArea>
#include <QSpinBox>
#include <QTabWidget>
#include <QVBoxLayout>

namespace az {

namespace {

QString slug(const QString& name) {
    QString s;
    for (QChar c : name.toLower()) s += c.isLetterOrNumber() ? c : QChar('-');
    while (s.contains("--")) s.replace("--", "-");
    return s.isEmpty() ? QString("theme") : s;
}

// A theme at a glance: its window, buttons, fields and highlight side by side.
QIcon chip(const Theme& t) {
    QPixmap pm(32, 16);
    QPainter p(&pm);
    const char* keys[] = {"window", "button", "field", "hi"};
    for (int i = 0; i < 4; ++i) p.fillRect(i * 8, 0, 8, 16, t.c.value(keys[i]));
    p.setPen(QColor(0, 0, 0, 120));
    p.drawRect(0, 0, 31, 15);
    p.end();
    return QIcon(pm);
}

class Editor : public QDialog {
public:
    explicit Editor(QWidget* parent) : QDialog(parent) {
        setWindowTitle("Theme Editor");
        setAttribute(Qt::WA_DeleteOnClose);
        resize(760, 600);
        auto* root = new QHBoxLayout(this);

        // The themes, and what can be done with them.
        auto* left = new QVBoxLayout;
        list_ = new QListWidget(this);
        list_->setMinimumWidth(250);
        list_->setIconSize(QSize(32, 16));
        left->addWidget(list_, 1);
        auto* row1 = new QHBoxLayout;
        auto* dup = new QPushButton("Duplicate", this);
        del_ = new QPushButton("Delete", this);
        row1->addWidget(dup);
        row1->addWidget(del_);
        left->addLayout(row1);
        auto* row2 = new QHBoxLayout;
        auto* imp = new QPushButton("Import...", this);
        auto* exp = new QPushButton("Export...", this);
        row2->addWidget(imp);
        row2->addWidget(exp);
        left->addLayout(row2);
        root->addLayout(left);

        // The selected theme's parts.
        auto* right = new QVBoxLayout;
        auto* name_row = new QFormLayout;
        name_ = new QLineEdit(this);
        name_row->addRow("Name:", name_);
        right->addLayout(name_row);
        auto* tabs = new QTabWidget(this);
        tabs->addTab(colours_page(), "Colours");
        tabs->addTab(text_page(), "Text");
        tabs->addTab(shapes_page(), "Shapes");
        right->addWidget(tabs, 1);
        note_ = new QLabel(this);
        note_->setWordWrap(true);
        note_->setObjectName("optionsDoc");
        right->addWidget(note_);
        auto* buttons = new QHBoxLayout;
        auto* save = new QPushButton("Save", this);
        auto* revert = new QPushButton("Revert", this);
        auto* close = new QPushButton("Close", this);
        save->setDefault(true);
        buttons->addStretch(1);
        buttons->addWidget(save);
        buttons->addWidget(revert);
        buttons->addWidget(close);
        right->addLayout(buttons);
        root->addLayout(right, 1);

        connect(list_, &QListWidget::currentRowChanged, this, [this](int r) {
            if (r < 0 || loading_) return;
            if (!confirm_leave()) {
                select_id(work_.id);
                return;
            }
            load(theme(list_->item(r)->data(Qt::UserRole).toString()));
        });
        connect(name_, &QLineEdit::textEdited, this, [this](const QString& s) {
            work_.name = s;
            dirty_ = true;
            update_note();
        });
        connect(dup, &QPushButton::clicked, this, [this] {
            Theme t = work_;
            t.builtin = false;
            t.name = work_.name + " copy";
            t.id = unique_id(slug(t.name));
            QString err;
            if (!save_user_theme(t, &err)) return fail(err);
            dirty_ = false;
            refresh(t.id);
            load(t);
        });
        connect(del_, &QPushButton::clicked, this, [this] {
            if (work_.builtin) return;
            if (QMessageBox::question(this, "Delete Theme", "Delete " + work_.name + "?") != QMessageBox::Yes) return;
            const QString gone = work_.id;
            delete_user_theme(gone);
            if (current_theme() == gone) apply_theme(*qApp, "classic");
            dirty_ = false;
            refresh(current_theme());
            load(theme(current_theme()));
        });
        connect(imp, &QPushButton::clicked, this, [this] {
            const QString f = QFileDialog::getOpenFileName(this, "Import Theme", QString(), "Aztek themes (*.json)");
            if (f.isEmpty()) return;
            QFile file(f);
            if (!file.open(QIODevice::ReadOnly)) return fail(file.errorString());
            const QJsonDocument doc = QJsonDocument::fromJson(file.readAll());
            if (!doc.isObject()) return fail("Not a theme file.");
            Theme t = Theme::from_json(doc.object());
            if (t.name.isEmpty()) t.name = QFileInfo(f).completeBaseName();
            t.id = unique_id(slug(t.name));
            QString err;
            if (!save_user_theme(t, &err)) return fail(err);
            refresh(t.id);
            load(t);
        });
        connect(exp, &QPushButton::clicked, this, [this] {
            const QString f = QFileDialog::getSaveFileName(this, "Export Theme", slug(work_.name) + ".json", "Aztek themes (*.json)");
            if (f.isEmpty()) return;
            QFile file(f);
            if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) return fail(file.errorString());
            file.write(QJsonDocument(work_.to_json()).toJson());
        });
        connect(save, &QPushButton::clicked, this, [this] { save_work(); });
        connect(revert, &QPushButton::clicked, this, [this] {
            dirty_ = false;
            load(theme(work_.id));
        });
        connect(close, &QPushButton::clicked, this, &QDialog::close);

        refresh(current_theme());
        load(theme(current_theme()));
    }

protected:
    void closeEvent(QCloseEvent* e) override {
        if (!confirm_leave()) {
            e->ignore();
            return;
        }
        // What's shown goes back to the theme in use.
        apply_theme(*qApp, current_theme());
        QDialog::closeEvent(e);
    }

private:
    QWidget* colours_page() {
        auto* scroll = new QScrollArea(this);
        scroll->setWidgetResizable(true);
        auto* w = new QWidget;
        auto* g = new QGridLayout(w);
        int i = 0;
        for (const auto& [key, label] : colour_keys()) {
            auto* b = new QPushButton(w);
            b->setFixedSize(54, 20);
            b->setObjectName("themeSwatch");
            swatches_[key] = b;
            const QString k = key;
            connect(b, &QPushButton::clicked, this, [this, k] {
                const QColor c = QColorDialog::getColor(work_.c.value(k), this, "Theme Colour: " + k);
                if (!c.isValid()) return;
                work_.c[k] = c;
                changed();
            });
            const int col = (i % 2) * 2, row = i / 2;
            g->addWidget(new QLabel(label + ":", w), row, col, Qt::AlignRight);
            g->addWidget(b, row, col + 1);
            ++i;
        }
        g->setColumnStretch(1, 1);
        g->setColumnStretch(3, 1);
        g->setRowStretch(i / 2 + 1, 1);
        scroll->setWidget(w);
        return scroll;
    }

    QWidget* text_page() {
        auto* w = new QWidget(this);
        auto* f = new QFormLayout(w);
        system_font_ = new QCheckBox("The system's font", w);
        font_ = new QFontComboBox(w);
        size_ = new QSpinBox(w);
        size_->setRange(6, 20);
        size_->setSuffix(" pt");
        italic_ = new QCheckBox("Italic", w);
        bold_ = new QCheckBox("Bold", w);
        f->addRow("", system_font_);
        f->addRow("Font:", font_);
        f->addRow("Size:", size_);
        auto* menus = new QHBoxLayout;
        menus->addWidget(italic_);
        menus->addWidget(bold_);
        menus->addStretch(1);
        f->addRow("Menus and tabs:", menus);
        connect(system_font_, &QCheckBox::toggled, this, [this](bool on) {
            if (loading_) return;
            font_->setEnabled(!on);
            size_->setEnabled(!on);
            if (on) {
                work_.font.clear();
                work_.point_size = 0;
            } else {
                work_.font = QStringList{font_->currentFont().family()};
                work_.point_size = size_->value();
            }
            changed();
        });
        connect(font_, &QFontComboBox::currentFontChanged, this, [this](const QFont& ft) {
            if (loading_) return;
            work_.font = QStringList{ft.family()};
            changed();
        });
        connect(size_, &QSpinBox::valueChanged, this, [this](int v) {
            if (loading_) return;
            work_.point_size = v;
            changed();
        });
        connect(italic_, &QCheckBox::toggled, this, [this](bool on) {
            if (loading_) return;
            work_.menu_italic = on;
            changed();
        });
        connect(bold_, &QCheckBox::toggled, this, [this](bool on) {
            if (loading_) return;
            work_.menu_bold = on;
            changed();
        });
        return w;
    }

    QComboBox* choice(QFormLayout* f, const QString& label, const QList<QPair<QString, QString>>& items, QString Theme::*field) {
        auto* c = new QComboBox;
        for (const auto& [id, text] : items) c->addItem(text, id);
        f->addRow(label, c);
        connect(c, &QComboBox::activated, this, [this, c, field](int i) {
            work_.*field = c->itemData(i).toString();
            changed();
        });
        choices_.append({c, field});
        return c;
    }

    QSpinBox* number(QFormLayout* f, const QString& label, int lo, int hi, int Theme::*field) {
        auto* s = new QSpinBox;
        s->setRange(lo, hi);
        s->setSuffix(" px");
        f->addRow(label, s);
        connect(s, &QSpinBox::valueChanged, this, [this, field](int v) {
            if (loading_) return;
            work_.*field = v;
            changed();
        });
        numbers_.append({s, field});
        return s;
    }

    QWidget* shapes_page() {
        auto* w = new QWidget(this);
        auto* f = new QFormLayout(w);
        choice(f, "Fill:", {{"classic", "Classic (the reference's own look)"}, {"flat", "Flat"}, {"gradient", "Gradient"}}, &Theme::fill);
        choice(f, "Bevel:", {{"none", "None"}, {"single", "Single (Motif)"}, {"double", "Double (1998)"}}, &Theme::bevel);
        number(f, "Corner radius:", 0, 14, &Theme::radius);
        number(f, "Border width:", 0, 3, &Theme::border);
        number(f, "Padding:", 0, 10, &Theme::padding);
        choice(f, "Tabs:", {{"boxed", "Boxed"}, {"underline", "Underlined"}, {"pill", "Pills"}, {"rounded", "Rounded tops"}}, &Theme::tabs);
        choice(f, "Arrows:", {{"chevron", "Chevron"}, {"triangle", "Triangle"}, {"boxed", "Triangle in a button"}}, &Theme::arrows);
        choice(f, "Scroll bars:", {{"thin", "Thin, no arrows"}, {"standard", "With arrows"}, {"bevel", "Bevelled"}}, &Theme::scrollbar);
        auto* note = new QLabel("Classic keeps the reference's own drawing and uses only the colours and text settings. Bevels "
                                "draw buttons, fields and menus with raised and sunken edges from the bevel light and shadow "
                                "colours.",
                                w);
        note->setWordWrap(true);
        note->setObjectName("optionsDoc");
        f->addRow(note);
        return w;
    }

    void load(const Theme& t) {
        loading_ = true;
        work_ = t;
        dirty_ = false;
        name_->setText(t.name);
        for (auto it = swatches_.begin(); it != swatches_.end(); ++it) paint_swatch(it.key());
        system_font_->setChecked(t.font.isEmpty());
        font_->setEnabled(!t.font.isEmpty());
        size_->setEnabled(!t.font.isEmpty());
        if (!t.font.isEmpty()) font_->setCurrentFont(QFont(t.font));
        size_->setValue(t.point_size > 0 ? t.point_size : QApplication::font().pointSize());
        italic_->setChecked(t.menu_italic);
        bold_->setChecked(t.menu_bold);
        for (const auto& [c, field] : choices_) c->setCurrentIndex(qMax(0, c->findData(t.*field)));
        for (const auto& [s, field] : numbers_) s->setValue(t.*field);
        del_->setEnabled(!t.builtin);
        select_id(t.id);
        loading_ = false;
        preview_theme(*qApp, work_);
        update_note();
    }

    void changed() {
        if (loading_) return;
        dirty_ = true;
        for (auto it = swatches_.begin(); it != swatches_.end(); ++it) paint_swatch(it.key());
        preview_theme(*qApp, work_);
        update_note();
    }

    void paint_swatch(const QString& k) {
        // A swatch keeps its own look whatever the theme draws buttons with.
        swatches_[k]->setStyleSheet(QString("QPushButton#themeSwatch { background: %1; border: 1px solid #000000; border-image: none; border-radius: 2px; padding: 0; }").arg(work_.c.value(k).name()));
        swatches_[k]->setToolTip(work_.c.value(k).name());
    }

    void update_note() {
        if (work_.builtin)
            note_->setText(dirty_ ? "Built-in themes stay as they are: Save makes a copy of your own with these changes." : "A built-in theme.");
        else
            note_->setText(dirty_ ? "Changed. Save keeps it, Revert goes back." : "Yours, in " + themes_folder() + ".");
    }

    void refresh(const QString& select) {
        loading_ = true;
        list_->clear();
        for (const Theme& t : themes()) {
            auto* item = new QListWidgetItem(t.builtin ? t.name : t.name + "  (yours)", list_);
            item->setData(Qt::UserRole, t.id);
            item->setToolTip(t.dark() ? "Dark" : "Light");
            item->setIcon(chip(t));
        }
        loading_ = false;
        select_id(select);
    }

    void select_id(const QString& id) {
        const bool was = loading_;
        loading_ = true;
        for (int i = 0; i < list_->count(); ++i)
            if (list_->item(i)->data(Qt::UserRole).toString() == id) list_->setCurrentRow(i);
        loading_ = was;
    }

    QString unique_id(const QString& base) {
        QString id = "user-" + base;
        int n = 2;
        auto taken = [](const QString& x) {
            for (const Theme& t : themes())
                if (t.id == x) return true;
            return false;
        };
        while (taken(id)) id = "user-" + base + "-" + QString::number(n++);
        return id;
    }

    bool save_work() {
        Theme t = work_;
        if (t.builtin) {
            const Theme original = theme(t.id);
            if (t.name == original.name) t.name += " (yours)";
            t.id = unique_id(slug(t.name));
            t.builtin = false;
        }
        QString err;
        if (!save_user_theme(t, &err)) {
            fail(err);
            return false;
        }
        dirty_ = false;
        apply_theme(*qApp, t.id);
        refresh(t.id);
        load(theme(t.id));
        return true;
    }

    bool confirm_leave() {
        if (!dirty_) return true;
        const auto r = QMessageBox::question(this, "Theme Editor", "Save the changes to " + work_.name + "?", QMessageBox::Save | QMessageBox::Discard | QMessageBox::Cancel, QMessageBox::Save);
        if (r == QMessageBox::Cancel) return false;
        if (r == QMessageBox::Save) return save_work();
        dirty_ = false;
        return true;
    }

    void fail(const QString& why) { QMessageBox::warning(this, "Theme Editor", why); }

    Theme work_;
    bool dirty_ = false;
    bool loading_ = false;
    QListWidget* list_ = nullptr;
    QLineEdit* name_ = nullptr;
    QLabel* note_ = nullptr;
    QPushButton* del_ = nullptr;
    QHash<QString, QPushButton*> swatches_;
    QCheckBox* system_font_ = nullptr;
    QFontComboBox* font_ = nullptr;
    QSpinBox* size_ = nullptr;
    QCheckBox* italic_ = nullptr;
    QCheckBox* bold_ = nullptr;
    QList<QPair<QComboBox*, QString Theme::*>> choices_;
    QList<QPair<QSpinBox*, int Theme::*>> numbers_;
};

} // namespace

void show_theme_editor(QWidget* parent) {
    static QPointer<Editor> open;
    if (!open) open = new Editor(parent);
    open->show();
    open->raise();
    open->activateWindow();
}

} // namespace az

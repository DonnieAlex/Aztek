// The file options windows. The File Type Specific Options come from the formats' own panels (fbxui, usdui).

#include "fileoptions.h"

#include <QCheckBox>
#include <QComboBox>
#include <QDialog>
#include <QFormLayout>
#include <QGroupBox>
#include <QHBoxLayout>
#include <QLabel>
#include <QMenuBar>
#include <QPushButton>
#include <QScrollArea>
#include <QFileDialog>
#include <QFileInfo>
#include <QFrame>
#include <QRegularExpression>
#include <memory>
#include <QSettings>
#include <QVBoxLayout>

#include "fbxui.h"
#include "host.h"
#include "usdui.h"

namespace az {

namespace {

QString key(FileAction a) {
    switch (a) {
    case FileAction::Import: return "fileOptions/import/type";
    case FileAction::ExportAll: return "fileOptions/exportAll/type";
    case FileAction::ExportSelection: return "fileOptions/exportSelection/type";
    }
    return QString();
}

bool loaded(const QString& plugin) {
    return ask("pluginInfo", QJsonObject{{"args", QJsonArray{plugin}}, {"query", true}, {"loaded", true}}).toBool();
}

// The reference's File type names, for the formats Aztek reads or writes; plug-in formats while their plug-in is loaded.
QStringList types(FileAction a) {
    QStringList t;
    t << "mayaAscii" << "mayaBinary";
    if (a == FileAction::Import) {
        if (loaded("fbx")) t << "FBX";
        if (loaded("objExport")) t << "OBJ";
        if (loaded("mayaUsdPlugin")) t << "USD Import";
        if (loaded("gltf")) t << "glTF";
        if (loaded("AbcImport")) t << "Alembic";
        t << "PLY" << "STL";
    } else {
        if (loaded("fbx")) t << "FBX export";
        if (loaded("objExport")) t << "OBJexport";
        if (loaded("mayaUsdPlugin")) t << "USD Export";
        if (loaded("gltf")) t << "glTF export";
        if (loaded("AbcExport")) t << "Alembic";
        t << "PLY" << "STL";
    }
    return t;
}

// glTF export: what goes in the file and how, kept under gltf/ (#122). The file kind follows the name's extension;
// "Format" picks it when the name has none.
OptionsPanel gltf_export_panel(QWidget* parent) {
    auto* w = new QWidget(parent);
    auto* form = new QFormLayout(w);
    QSettings s;
    auto* format = new QComboBox(w);
    format->addItems({"glTF Binary (.glb)", "glTF Separate (.gltf)"});
    format->setCurrentIndex(s.value("gltf/format", 0).toInt());
    format->setToolTip("The kind of file when the name has no extension: one binary file with textures inside, or JSON with textures beside it");
    form->addRow("Format:", format);
    auto* up = new QComboBox(w);
    up->addItems({"Y Up", "Z Up"});
    up->setCurrentIndex(s.value("gltf/exportYUp", true).toBool() ? 0 : 1);
    up->setToolTip("The file's up axis. Y is the format's; a scene that is the other way up is converted");
    form->addRow("Up axis:", up);
    struct Box { const char* label; const char* key; bool on; const char* tip; };
    const Box boxes[] = {
        {"Materials", "materials", true, "Each shading group's material and its colour texture"},
        {"Skins", "skins", true, "Joints, weights and bind matrices of bound meshes"},
        {"Blend Shapes", "blendShapes", true, "Each blend shape target as a morph target, with the weights as they are"},
        {"Animation", "animation", false, "Keyed transforms and blend shape weights, sampled over the frame range"},
    };
    QList<QPair<QString, QCheckBox*>> checks;
    for (const Box& b : boxes) {
        auto* c = new QCheckBox(b.label, w);
        c->setChecked(s.value(QString("gltf/") + b.key, b.on).toBool());
        c->setToolTip(b.tip);
        form->addRow(c);
        checks.append({b.key, c});
    }
    auto* range = new QComboBox(w);
    range->addItems({"Time Slider", "Animation Range"});
    range->setCurrentIndex(s.value("gltf/range", 0).toInt());
    range->setToolTip("The frames animation is sampled over: the playback range or the whole animation range");
    form->addRow("Frame range:", range);
    auto save = [format, up, checks, range] {
        QSettings s;
        s.setValue("gltf/format", format->currentIndex());
        s.setValue("gltf/exportYUp", up->currentIndex() == 0);
        for (const auto& c : checks) s.setValue("gltf/" + c.first, c.second->isChecked());
        s.setValue("gltf/range", range->currentIndex());
    };
    auto reset = [] {
        QSettings s;
        for (const char* k : {"format", "exportYUp", "materials", "skins", "blendShapes", "animation", "range"}) s.remove(QString("gltf/") + k);
    };
    return {w, save, reset};
}

class FileOptions : public QDialog {
public:
    FileOptions(QWidget* parent, FileAction action, std::function<void(const QString&)> run) : QDialog(parent), action_(action), run_(std::move(run)) {
        const char* title = action == FileAction::Import ? "Import Options" : action == FileAction::ExportAll ? "Export All Options" : "Export Selection Options";
        const char* verb = action == FileAction::Import ? "Import" : action == FileAction::ExportAll ? "Export All" : "Export Selection";
        setWindowTitle(title);
        resize(520, 720);
        auto* outer = new QVBoxLayout(this);
        auto* bar = new QMenuBar(this);
        QMenu* edit = bar->addMenu("Edit");
        edit->addAction("Save Settings", [this] { save(); });
        edit->addAction("Reset Settings", [this] { reset(); });
        outer->setMenuBar(bar);

        auto* scroll = new QScrollArea(this);
        scroll->setWidgetResizable(true);
        scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        auto* body = new QWidget(scroll);
        auto* v = new QVBoxLayout(body);
        auto* general = new QGroupBox("General Options", body);
        auto* form = new QFormLayout(general);
        type_ = new QComboBox(general);
        type_->addItems(types(action));
        const int at = type_->findText(chosen_file_type(action));
        if (at >= 0) type_->setCurrentIndex(at);
        form->addRow("File type:", type_);
        if (action != FileAction::Import) {
            ext_ = new QCheckBox("Default file extensions", general);
            ext_->setChecked(default_file_extensions());
            form->addRow(ext_);
        }
        v->addWidget(general);
        specific_ = new QGroupBox("File Type Specific Options", body);
        specific_layout_ = new QVBoxLayout(specific_);
        v->addWidget(specific_);
        v->addStretch(1);
        scroll->setWidget(body);
        outer->addWidget(scroll, 1);
        connect(type_, &QComboBox::currentTextChanged, this, [this] { build(); });
        build();

        // The reference's three buttons, of one width.
        auto* buttons = new QHBoxLayout();
        auto* act = new QPushButton(verb, this);
        auto* apply = new QPushButton("Apply", this);
        auto* close = new QPushButton("Close", this);
        for (QPushButton* b : {act, apply, close}) {
            b->setAutoDefault(false);
            buttons->addWidget(b, 1);
        }
        act->setDefault(true);
        connect(act, &QPushButton::clicked, this, [this] {
            save();
            const QString t = type_->currentText();
            auto run = run_;
            this->close();
            run(t);
        });
        connect(apply, &QPushButton::clicked, this, [this] {
            save();
            run_(type_->currentText());
        });
        connect(close, &QPushButton::clicked, this, &QDialog::reject);
        outer->addLayout(buttons);
    }

private:
    void build() {
        if (panel_) {
            specific_layout_->removeWidget(panel_);
            delete panel_;
            panel_ = nullptr;
        }
        save_panel_ = nullptr;
        reset_panel_ = nullptr;
        const QString t = type_->currentText();
        if (t == "FBX" || t == "FBX export") {
            FbxPanel p = action_ == FileAction::Import ? fbx_import_panel(specific_) : fbx_export_panel(specific_);
            panel_ = p.widget;
            reset_panel_ = p.reset;
        } else if (t == "USD Import" || t == "USD Export") {
            OptionsPanel p = action_ == FileAction::Import ? usd_import_panel(specific_) : usd_export_panel(specific_);
            panel_ = p.widget;
            save_panel_ = p.save;
            reset_panel_ = p.reset;
        } else if (t == "glTF export") {
            OptionsPanel p = gltf_export_panel(specific_);
            panel_ = p.widget;
            save_panel_ = p.save;
            reset_panel_ = p.reset;
        } else {
            panel_ = new QLabel("This file type has no further options.", specific_);
        }
        specific_layout_->addWidget(panel_);
    }

    void save() {
        QSettings s;
        s.setValue(key(action_), type_->currentText());
        if (ext_) s.setValue("fileOptions/defaultExtensions", ext_->isChecked());
        if (save_panel_) save_panel_();
    }

    void reset() {
        QSettings s;
        s.remove(key(action_));
        s.remove("fileOptions/defaultExtensions");
        if (reset_panel_) reset_panel_();
        const int at = type_->findText(chosen_file_type(action_));
        if (at >= 0) type_->setCurrentIndex(at);
        if (ext_) ext_->setChecked(true);
        build();
    }

    FileAction action_;
    std::function<void(const QString&)> run_;
    QComboBox* type_ = nullptr;
    QCheckBox* ext_ = nullptr;
    QGroupBox* specific_ = nullptr;
    QVBoxLayout* specific_layout_ = nullptr;
    QWidget* panel_ = nullptr;
    std::function<void()> save_panel_, reset_panel_;
};

} // namespace

void file_options_window(QWidget* parent, FileAction action, std::function<void(const QString&)> run) {
    // Not modal (#109): Aztek stays usable beside it, and it stays over the main window.
    auto* d = new FileOptions(parent ? parent->window() : main_window_widget(), action, std::move(run));
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setWindowModality(Qt::NonModal);
    d->show();
    d->raise();
}

void browse_file(bool save, const QString& title, const QString& start, const QString& filters, const QString& selected,
                 std::function<void(const QString&)> done) {
    // Qt's own dialog: a system one blocks the window it belongs to while it is open.
    auto* d = new QFileDialog(main_window_widget(), title, start, filters);
    d->setOption(QFileDialog::DontUseNativeDialog, true);
    d->setAcceptMode(save ? QFileDialog::AcceptSave : QFileDialog::AcceptOpen);
    d->setFileMode(save ? QFileDialog::AnyFile : QFileDialog::ExistingFile);
    if (!selected.isEmpty()) d->selectNameFilter(selected);
    if (save && !start.isEmpty() && !QFileInfo(start).isDir()) d->selectFile(start);
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setWindowModality(Qt::NonModal);
    QObject::connect(d, &QFileDialog::fileSelected, d, [done](const QString& f) {
        if (!f.isEmpty()) done(f);
    });
    d->show();
    d->raise();
}

QJsonObject gltf_export_flags() {
    QSettings s;
    QJsonObject f{{"upAxis", s.value("gltf/exportYUp", true).toBool() ? "y" : "z"},
                  {"materials", s.value("gltf/materials", true).toBool()},
                  {"skins", s.value("gltf/skins", true).toBool()},
                  {"blendShapes", s.value("gltf/blendShapes", true).toBool()},
                  {"animation", s.value("gltf/animation", false).toBool()}};
    if (f["animation"].toBool()) {
        const bool whole = s.value("gltf/range", 0).toInt() == 1;
        f["startTime"] = ask("playbackOptions", QJsonObject{{"query", true}, {whole ? "animationStartTime" : "minTime", true}}).toDouble();
        f["endTime"] = ask("playbackOptions", QJsonObject{{"query", true}, {whole ? "animationEndTime" : "maxTime", true}}).toDouble();
    }
    return f;
}

namespace {

// The options for a file type filter's formats, for the browser's side panel.
OptionsPanel panel_for_filter(FileAction action, const QString& filter, QWidget* parent) {
    const bool import = action == FileAction::Import;
    if (filter.startsWith("FBX")) {
        FbxPanel p = import ? fbx_import_panel(parent) : fbx_export_panel(parent);
        return {p.widget, [] {}, p.reset};
    }
    if (filter.startsWith("USD")) return import ? usd_import_panel(parent) : usd_export_panel(parent);
    if (filter.startsWith("glTF") && !import) return gltf_export_panel(parent);
    auto* l = new QLabel("This file type has no further options.", parent);
    l->setWordWrap(true);
    return {l, [] {}, [] {}};
}

} // namespace

void browse_file_with_options(bool save, FileAction action, const QString& title, const QString& start, const QString& filters,
                              const QString& selected, std::function<void(const QString& file)> done) {
    auto* d = new QDialog(main_window_widget());
    d->setWindowTitle(title);
    d->resize(1100, 620);
    auto* row = new QHBoxLayout(d);
    row->setContentsMargins(4, 4, 4, 4);
    auto* fd = new QFileDialog(d, title, start, filters);
    fd->setWindowFlags(Qt::Widget);
    fd->setOption(QFileDialog::DontUseNativeDialog, true);
    fd->setAcceptMode(save ? QFileDialog::AcceptSave : QFileDialog::AcceptOpen);
    fd->setFileMode(save ? QFileDialog::AnyFile : QFileDialog::ExistingFile);
    if (!selected.isEmpty()) fd->selectNameFilter(selected);
    if (save && !start.isEmpty() && !QFileInfo(start).isDir()) fd->selectFile(start);
    row->addWidget(fd, 3);
    auto* side = new QGroupBox("Options", d);
    auto* side_lay = new QVBoxLayout(side);
    auto* scroll = new QScrollArea(side);
    scroll->setWidgetResizable(true);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    scroll->setFrameShape(QFrame::NoFrame);
    side_lay->addWidget(scroll);
    side->setMinimumWidth(300);
    row->addWidget(side, 2);
    auto saver = std::make_shared<std::function<void()>>([] {});
    auto show = [action, scroll, side, saver](const QString& filter) {
        OptionsPanel p = panel_for_filter(action, filter, scroll);
        if (QWidget* old = scroll->takeWidget()) old->deleteLater();
        scroll->setWidget(p.widget);
        side->setTitle(filter.section(" (", 0, 0) + " Options");
        *saver = p.save;
    };
    show(fd->selectedNameFilter());
    QObject::connect(fd, &QFileDialog::filterSelected, d, show);
    // A name typed with an extension shows that type's options.
    QObject::connect(fd, &QFileDialog::currentChanged, d, [fd, show](const QString& f) {
        const QString ext = QFileInfo(f).suffix().toLower();
        if (ext.isEmpty()) return;
        for (const QString& filter : fd->nameFilters())
            if (filter.contains("*." + ext) && filter != fd->selectedNameFilter() && !filter.contains(' ' + QString("*.ma *.mb"))) {
                fd->selectNameFilter(filter);
                show(filter);
                return;
            }
    });
    QObject::connect(fd, &QFileDialog::fileSelected, d, [d, fd, save, saver, done](QString f) {
        (*saver)();
        // A name without an extension takes the chosen type's (glTF's as its Format says).
        if (save && !f.isEmpty() && QFileInfo(f).suffix().isEmpty() && default_file_extensions()) {
            const QString filter = fd->selectedNameFilter();
            const int at = filter.indexOf("*.");
            if (filter.startsWith("glTF")) f += QSettings().value("gltf/format", 0).toInt() == 0 ? ".glb" : ".gltf";
            else if (at >= 0) f += "." + filter.mid(at + 2).section(QRegularExpression("[ )]"), 0, 0);
        }
        d->accept();
        if (!f.isEmpty()) done(f);
    });
    QObject::connect(fd, &QFileDialog::rejected, d, &QDialog::reject);
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setWindowModality(Qt::NonModal);
    d->show();
    d->raise();
}

QString chosen_file_type(FileAction action) {
    return QSettings().value(key(action), "mayaAscii").toString();
}

QString file_type_filter(const QString& t) {
    if (t == "mayaAscii") return "mayaAscii (*.ma)";
    if (t == "mayaBinary") return "mayaBinary (*.mb)";
    if (t == "FBX" || t == "FBX export") return "FBX (*.fbx)";
    if (t == "OBJ" || t == "OBJexport") return "OBJ (*.obj)";
    if (t == "USD Import" || t == "USD Export") return kUsdFilter;
    if (t == "glTF" || t == "glTF export") return "glTF 2.0 (*.glb *.gltf)";
    if (t == "PLY") return "PLY (*.ply)";
    if (t == "STL") return "STL (*.stl)";
    if (t == "Alembic") return "Alembic (*.abc)";
    return QString();
}

QString file_type_extension(const QString& t) {
    if (t == "mayaAscii") return "ma";
    if (t == "mayaBinary") return "mb";
    if (t == "FBX" || t == "FBX export") return "fbx";
    if (t == "OBJ" || t == "OBJexport") return "obj";
    if (t == "USD Import" || t == "USD Export") return "usd";
    if (t == "glTF" || t == "glTF export") return "gltf";
    if (t == "PLY") return "ply";
    if (t == "STL") return "stl";
    if (t == "Alembic") return "abc";
    return QString();
}

bool default_file_extensions() {
    return QSettings().value("fileOptions/defaultExtensions", true).toBool();
}

} // namespace az

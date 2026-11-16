// The welcome screen (#85), as the reference has one at start-up that can be turned off: ways in (a new scene, a scene
// to open, the recent ones), the example scenes and scripts that come with Aztek, useful links, and from GitHub the
// latest release's notes and the announcements. GitHub is asked only when update checks are on (Preferences >
// Interface), as nothing leaves the machine otherwise; a button asks once on demand.

#include "mainwindow.h"
#include "aztek.h"
#include "host.h"
#include "icons.h"
#include "panels.h"

#include <QCheckBox>
#include <QCoreApplication>
#include <QDateTime>
#include <QDesktopServices>
#include <QDialog>
#include <QDir>
#include <QFileInfo>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QListWidget>
#include <QPointer>
#include <QPushButton>
#include <QSettings>
#include <QTextBrowser>
#include <QThread>
#include <QTreeWidget>
#include <QUrl>
#include <QVBoxLayout>

namespace az {

namespace {

const char* kSite = "https://srlegrand.github.io/Aztek/";
const char* kAnnouncements = "https://srlegrand.github.io/Aztek/announcements.json";
const char* kAnnouncementsPage = "https://github.com/srlegrand/Aztek/discussions/categories/announcements";
const char* kReleases = "https://github.com/srlegrand/Aztek/releases";

// "rigging_ik_arm.ma" reads "Rigging IK arm".
QString label_of(const QString& name) {
    static const QHash<QString, QString> kept = {{"ik", "IK"}, {"fk", "FK"}, {"uv", "UV"}, {"uvs", "UVs"}, {"usd", "USD"}, {"nurbs", "NURBS"}};
    QStringList words = QFileInfo(name).completeBaseName().split('_', Qt::SkipEmptyParts);
    for (QString& w : words) w = kept.value(w.toLower(), w);
    QString label = words.join(' ');
    if (!label.isEmpty()) label[0] = label[0].toUpper();
    return label;
}

// The examples folder: beside the program, in the source tree when running from a build, where the Linux packages
// put it. Empty when there is none.
QString examples_folder() {
    const QString base = QCoreApplication::applicationDirPath();
    for (const QString& where : {base + "/examples", base + "/../examples", base + "/../share/aztek/examples"})
        if (QDir(where).exists() && !QDir(where).entryList({"*.ma"}, QDir::Files).isEmpty()) return QDir(where).absolutePath();
    return QString();
}

// A GET in a thread of its own, the answer back on the interface thread. Through the update check's HTTPS client and
// proxy settings.
void fetch(QObject* owner, const QString& url, std::function<void(const QJsonDocument&)> done) {
    QPointer<QObject> guard(owner);
    QThread* t = QThread::create([guard, url, done] {
        char* r = url.isEmpty() ? aztek_update_latest() : aztek_fetch_json(url.toUtf8().constData());
        const QByteArray body = QByteArray(r);
        aztek_free(r);
        QMetaObject::invokeMethod(qApp, [guard, done, body] {
            if (guard) done(QJsonDocument::fromJson(body));
        }, Qt::QueuedConnection);
    });
    QObject::connect(t, &QThread::finished, t, &QObject::deleteLater);
    t->start();
}

QLabel* heading(const QString& text, QWidget* parent) {
    auto* l = new QLabel(text, parent);
    QFont f = l->font();
    f.setBold(true);
    f.setPointSizeF(f.pointSizeF() * 1.15);
    l->setFont(f);
    return l;
}

QPushButton* link_button(const QString& text, const QString& url, QWidget* parent) {
    auto* b = new QPushButton(text, parent);
    b->setFlat(true);
    b->setCursor(Qt::PointingHandCursor);
    b->setToolTip(url);
    b->setStyleSheet("QPushButton { text-align: left; padding: 2px 4px; }");
    QObject::connect(b, &QPushButton::clicked, [url] { QDesktopServices::openUrl(QUrl(url)); });
    return b;
}

} // namespace

void MainWindow::show_welcome(bool at_startup) {
    if (at_startup && !QSettings().value("welcome/atStartup", true).toBool()) return;
    static QPointer<QDialog> open;
    if (open) {
        open->show();
        open->raise();
        open->activateWindow();
        return;
    }
    auto* d = new QDialog(this, editor_window_flags());
    open = d;
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setObjectName("welcome");
    d->setWindowTitle("Welcome to Aztek");
    d->resize(980, 640);
    auto* outer = new QVBoxLayout(d);

    // The mark and the version.
    {
        auto* top = new QHBoxLayout();
        auto* logo = new QLabel(d);
        QPixmap pm(":/aztek-icon.png");
        if (!pm.isNull()) logo->setPixmap(pm.scaled(48, 48, Qt::KeepAspectRatio, Qt::SmoothTransformation));
        top->addWidget(logo);
        auto* title = new QLabel(QString("<span style='font-size:20pt; font-weight:600'>Aztek</span>&nbsp;&nbsp;<span style='opacity:0.7'>%1</span>")
                                     .arg(QString(AZTEK_VERSION).toHtmlEscaped()), d);
        top->addWidget(title, 1);
        outer->addLayout(top);
    }

    auto* columns = new QHBoxLayout();
    outer->addLayout(columns, 1);

    // Left: start, recent, examples.
    auto* left = new QVBoxLayout();
    columns->addLayout(left, 2);
    left->addWidget(heading("Start", d));
    {
        auto* row = new QHBoxLayout();
        auto* fresh = new QPushButton(icon("new"), "New Scene", d);
        auto* open_btn = new QPushButton(icon("open"), "Open Scene...", d);
        connect(fresh, &QPushButton::clicked, this, [this, d] {
            new_scene();
            d->close();
        });
        connect(open_btn, &QPushButton::clicked, this, [this, d] {
            d->close();
            open_scene();
        });
        row->addWidget(fresh);
        row->addWidget(open_btn);
        row->addStretch(1);
        left->addLayout(row);
    }
    const QStringList recent = QSettings().value("recentFiles").toStringList();
    if (!recent.isEmpty()) {
        left->addWidget(heading("Recent", d));
        auto* list = new QListWidget(d);
        list->setMaximumHeight(120);
        for (const QString& f : recent.mid(0, 6)) {
            auto* it = new QListWidgetItem(QFileInfo(f).fileName(), list);
            it->setToolTip(f);
            it->setData(Qt::UserRole, f);
        }
        connect(list, &QListWidget::itemActivated, this, [this, d](QListWidgetItem* it) {
            if (!confirm_unsaved()) return;
            d->close();
            open_file(it->data(Qt::UserRole).toString());
        });
        left->addWidget(list);
    }
    left->addWidget(heading("Examples", d));
    auto* examples = new QTreeWidget(d);
    examples->setHeaderHidden(true);
    examples->setRootIsDecorated(true);
    const QString root = examples_folder();
    if (root.isEmpty()) {
        auto* it = new QTreeWidgetItem(examples, {"No examples folder beside this copy of Aztek"});
        it->setDisabled(true);
    } else {
        auto add_scenes = [](QTreeWidgetItem* under, QTreeWidget* tree, const QDir& dir) {
            for (const QString& name : dir.entryList({"*.ma"}, QDir::Files, QDir::Name)) {
                // Those that start ref_ are what the references example points at.
                if (name.startsWith("ref_")) continue;
                auto* it = under ? new QTreeWidgetItem(under, {label_of(name)}) : new QTreeWidgetItem(tree, {label_of(name)});
                it->setData(0, Qt::UserRole, dir.absoluteFilePath(name));
                it->setToolTip(0, dir.absoluteFilePath(name));
            }
        };
        const QDir dir(root);
        for (const QString& sub : dir.entryList(QDir::Dirs | QDir::NoDotAndDotDot, QDir::Name)) {
            const QDir in(dir.absoluteFilePath(sub));
            if (sub == "scripts" || in.entryList({"*.ma"}, QDir::Files).isEmpty()) continue;
            auto* group = new QTreeWidgetItem(examples, {label_of(sub)});
            add_scenes(group, examples, in);
        }
        add_scenes(nullptr, examples, dir);
        const QDir scripts(dir.absoluteFilePath("scripts"));
        const QStringList files = scripts.entryList({"*.py", "*.mel"}, QDir::Files, QDir::Name);
        if (!files.isEmpty()) {
            auto* group = new QTreeWidgetItem(examples, {"Scripts"});
            for (const QString& name : files) {
                auto* it = new QTreeWidgetItem(group, {name});
                it->setData(0, Qt::UserRole, scripts.absoluteFilePath(name));
                it->setData(0, Qt::UserRole + 1, true);
            }
        }
    }
    examples->setToolTip("Double-click to open: scenes open, scripts go to the Script Editor");
    connect(examples, &QTreeWidget::itemActivated, this, [this, d](QTreeWidgetItem* it) {
        const QString path = it->data(0, Qt::UserRole).toString();
        if (path.isEmpty()) return;
        if (it->data(0, Qt::UserRole + 1).toBool()) {
            show_script_editor();
            if (script_editor_) script_editor_->load_file(path);
            return;
        }
        if (!confirm_unsaved()) return;
        d->close();
        open_file(path);
    });
    left->addWidget(examples, 1);

    // Middle: what's new and the announcements.
    auto* mid = new QVBoxLayout();
    columns->addLayout(mid, 3);
    auto* news_row = new QHBoxLayout();
    news_row->addWidget(heading("What's new", d), 1);
    news_row->addWidget(link_button("All releases", kReleases, d));
    mid->addLayout(news_row);
    auto* notes = new QTextBrowser(d);
    notes->setOpenExternalLinks(true);
    mid->addWidget(notes, 3);
    auto* ann_row = new QHBoxLayout();
    ann_row->addWidget(heading("Announcements", d), 1);
    ann_row->addWidget(link_button("All announcements", kAnnouncementsPage, d));
    mid->addLayout(ann_row);
    auto* ann = new QListWidget(d);
    connect(ann, &QListWidget::itemActivated, [](QListWidgetItem* it) {
        const QString url = it->data(Qt::UserRole).toString();
        if (!url.isEmpty()) QDesktopServices::openUrl(QUrl(url));
    });
    mid->addWidget(ann, 2);

    auto load_news = [d, notes, ann] {
        notes->setPlainText("Asking GitHub...");
        ann->clear();
        fetch(d, QString(), [notes](const QJsonDocument& doc) {
            const QJsonObject r = doc.object();
            if (r.contains("error") || r.isEmpty()) {
                notes->setPlainText("The release notes couldn't be fetched: " + r["error"].toString("no answer"));
                return;
            }
            QString md = QString("## %1\n\n").arg(r["name"].toString(r["tag"].toString())) + r["body"].toString();
            notes->setMarkdown(md);
        });
        fetch(d, kAnnouncements, [ann](const QJsonDocument& doc) {
            const QJsonArray list = doc.isArray() ? doc.array() : doc.object()["announcements"].toArray();
            if (list.isEmpty()) {
                auto* it = new QListWidgetItem("No announcements could be fetched", ann);
                it->setFlags(Qt::NoItemFlags);
                return;
            }
            for (const QJsonValue& v : list) {
                const QJsonObject a = v.toObject();
                const QString date = QDateTime::fromString(a["date"].toString(), Qt::ISODate).toString("dd/MM/yyyy");
                auto* it = new QListWidgetItem(date.isEmpty() ? a["title"].toString() : date + "   " + a["title"].toString(), ann);
                it->setData(Qt::UserRole, a["url"].toString());
                it->setToolTip(a["url"].toString());
            }
        });
    };
    if (QSettings().value("prefs/updateCheck", false).toBool()) {
        load_news();
    } else {
        notes->setPlainText("Release notes and announcements come from GitHub, which Aztek only asks when update checks are on "
                            "(Preferences > Interface).");
        auto* once = new QPushButton("Fetch them now", d);
        mid->addWidget(once);
        connect(once, &QPushButton::clicked, d, [once, load_news] {
            once->hide();
            load_news();
        });
    }

    // Right: links.
    auto* right = new QVBoxLayout();
    columns->addLayout(right, 1);
    right->addWidget(heading("Links", d));
    right->addWidget(link_button("Website", kSite, d));
    right->addWidget(link_button("Read me and documentation", "https://github.com/srlegrand/Aztek#readme", d));
    right->addWidget(link_button("Release notes", kReleases, d));
    right->addWidget(link_button("Discussions", "https://github.com/srlegrand/Aztek/discussions", d));
    right->addWidget(link_button("Discord", "https://discord.gg/XDEgpWDVKa", d));
    right->addWidget(link_button("Report an issue", "https://github.com/srlegrand/Aztek/issues/new/choose", d));
    right->addWidget(link_button("Support on Ko-fi", "https://ko-fi.com/turtleface", d));
    right->addStretch(1);

    // Bottom: shown at start-up or not, and close.
    auto* bottom = new QHBoxLayout();
    auto* again = new QCheckBox("Show this at start-up", d);
    again->setChecked(QSettings().value("welcome/atStartup", true).toBool());
    connect(again, &QCheckBox::toggled, [](bool on) { QSettings().setValue("welcome/atStartup", on); });
    bottom->addWidget(again);
    bottom->addStretch(1);
    auto* close = new QPushButton("Close", d);
    connect(close, &QPushButton::clicked, d, &QDialog::close);
    bottom->addWidget(close);
    outer->addLayout(bottom);

    d->show();
    d->raise();
}

} // namespace az

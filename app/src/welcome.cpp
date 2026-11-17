// The welcome screen (#85), as the reference has one at start-up that can be turned off: ways in (a new scene, a scene
// to open, the recent ones), the example scenes and scripts that come with Aztek, useful links, and from GitHub the
// notes of the day's releases and the announcements. GitHub is asked only when update checks are on (Preferences >
// Interface), as nothing leaves the machine otherwise; a button asks once on demand. A greyhound keeps it company,
// in a new pose with each version, and a bar along the foot shows how far Aztek is towards parity.

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
#include <QPainter>
#include <QPainterPath>
#include <QPointer>
#include <QProgressBar>
#include <QPushButton>
#include <QRegularExpression>
#include <QSettings>
#include <QTextBrowser>
#include <QThread>
#include <QTime>
#include <QTreeWidget>
#include <QUrl>
#include <QVBoxLayout>

#include <algorithm>
#include <cmath>

namespace az {

namespace {

const char* kSite = "https://srlegrand.github.io/Aztek/";
const char* kAnnouncements = "https://srlegrand.github.io/Aztek/announcements.json";
const char* kAnnouncementsPage = "https://github.com/srlegrand/Aztek/discussions/categories/announcements";
const char* kReleases = "https://github.com/srlegrand/Aztek/releases";
const char* kReleasesApi = "https://api.github.com/repos/srlegrand/Aztek/releases?per_page=40";

// The greyhound's poses, one a version: 2.N shows pose N mod their number.
const char* kPoses[] = {"1-stand", "2-sit", "3-bow", "4-run", "5-lie", "6-sleep"};

// N of "2.N (commit)" or "2.N.0".
int build_number() {
    const QRegularExpressionMatch m = QRegularExpression("^\\d+\\.(\\d+)").match(QString(AZTEK_VERSION));
    return m.hasMatch() ? m.captured(1).toInt() : 0;
}

// The welcome screen's own small icons, drawn on a 32 unit square in the colour given.
QIcon welcome_icon(const QString& name, const QColor& c) {
    QPixmap pm(64, 64);
    pm.fill(Qt::transparent);
    QPainter p(&pm);
    p.setRenderHint(QPainter::Antialiasing);
    p.scale(2, 2);
    QPen pen(c, 2.2, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin);
    p.setPen(pen);
    p.setBrush(Qt::NoBrush);
    if (name == "globe") {
        p.drawEllipse(QRectF(5, 5, 22, 22));
        p.drawEllipse(QRectF(11, 5, 10, 22));
        p.drawLine(QPointF(5, 16), QPointF(27, 16));
    } else if (name == "book") {
        p.drawRoundedRect(QRectF(7, 5, 18, 23), 2, 2);
        p.drawLine(QPointF(11, 5), QPointF(11, 28));
        p.drawLine(QPointF(14, 11), QPointF(21, 11));
    } else if (name == "chat") {
        QPainterPath path;
        path.addRoundedRect(QRectF(4, 6, 24, 16), 5, 5);
        p.drawPath(path);
        p.drawPolyline(QPolygonF({QPointF(10, 22), QPointF(8, 28), QPointF(15, 22)}));
    } else if (name == "bug") {
        p.drawEllipse(QRectF(10, 10, 12, 16));
        p.drawEllipse(QRectF(12, 5, 8, 7));
        for (double y : {14.0, 18.0, 22.0}) {
            p.drawLine(QPointF(10, y), QPointF(5, y - 2));
            p.drawLine(QPointF(22, y), QPointF(27, y - 2));
        }
    } else if (name == "heart") {
        QPainterPath path(QPointF(16, 27));
        path.cubicTo(2, 18, 4, 5, 16, 11);
        path.cubicTo(28, 5, 30, 18, 16, 27);
        p.setBrush(c);
        p.drawPath(path);
    } else if (name == "megaphone") {
        p.drawPolygon(QPolygonF({QPointF(5, 13), QPointF(11, 13), QPointF(24, 6), QPointF(24, 26), QPointF(11, 19), QPointF(5, 19)}));
        p.drawLine(QPointF(9, 19), QPointF(11, 26));
    } else if (name == "star") {
        QPolygonF star;
        for (int i = 0; i < 10; ++i) {
            const double a = M_PI / 2 + i * M_PI / 5, r = i % 2 ? 5.5 : 12;
            star << QPointF(16 + r * std::cos(a), 17 - r * std::sin(a));
        }
        p.setBrush(c);
        p.drawPolygon(star);
    } else if (name == "rocket") {
        QPainterPath path(QPointF(16, 3));
        path.cubicTo(24, 9, 23, 19, 20, 24);
        path.lineTo(12, 24);
        path.cubicTo(9, 19, 8, 9, 16, 3);
        p.drawPath(path);
        p.drawEllipse(QRectF(13.5, 10, 5, 5));
        p.drawLine(QPointF(14, 27), QPointF(14, 29));
        p.drawLine(QPointF(18, 27), QPointF(18, 29));
    } else if (name == "folder") {
        p.drawPolygon(QPolygonF({QPointF(4, 9), QPointF(12, 9), QPointF(15, 12), QPointF(28, 12), QPointF(28, 26), QPointF(4, 26)}));
    } else if (name == "clock") {
        p.drawEllipse(QRectF(5, 5, 22, 22));
        p.drawPolyline(QPolygonF({QPointF(16, 9), QPointF(16, 16), QPointF(21, 19)}));
    }
    p.end();
    return QIcon(pm);
}

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

// A section's title, with a small coloured icon before it.
QWidget* heading(const QString& text, QWidget* parent, const QString& glyph = QString(), const QColor& tint = QColor(217, 176, 124)) {
    auto* w = new QWidget(parent);
    auto* row = new QHBoxLayout(w);
    row->setContentsMargins(0, 6, 0, 2);
    row->setSpacing(6);
    if (!glyph.isEmpty()) {
        auto* i = new QLabel(w);
        i->setPixmap(welcome_icon(glyph, tint).pixmap(20, 20));
        row->addWidget(i);
    }
    auto* l = new QLabel(text, w);
    l->setObjectName("title");
    QFont f = l->font();
    f.setBold(true);
    f.setPointSizeF(f.pointSizeF() * 1.15);
    l->setFont(f);
    row->addWidget(l, 1);
    return w;
}

QPushButton* link_button(const QString& text, const QString& url, QWidget* parent, const QString& glyph = QString(), const QColor& tint = QColor(217, 176, 124)) {
    auto* b = new QPushButton(text, parent);
    if (!glyph.isEmpty()) b->setIcon(welcome_icon(glyph, tint));
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
        const int hour = QTime::currentTime().hour();
        const QString hello = hour < 5 ? "Up late?" : hour < 12 ? "Good morning!" : hour < 18 ? "Good afternoon!" : "Good evening!";
        auto* title = new QLabel(QString("<span style='font-size:20pt; font-weight:600'>Aztek</span>&nbsp;&nbsp;<span style='opacity:0.7'>%1</span>"
                                         "<br><span style='font-size:12pt'>%2 What shall we make today?</span>")
                                     .arg(QString(AZTEK_VERSION).toHtmlEscaped(), hello),
                                 d);
        top->addWidget(title, 1);
        outer->addLayout(top);
    }

    auto* columns = new QHBoxLayout();
    outer->addLayout(columns, 1);

    // Left: start, recent, examples.
    auto* left = new QVBoxLayout();
    columns->addLayout(left, 2);
    left->addWidget(heading("Start", d, "rocket", QColor(240, 160, 50)));
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
        left->addWidget(heading("Recent", d, "clock", QColor(95, 160, 220)));
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
    left->addWidget(heading("Examples", d, "folder", QColor(110, 200, 120)));
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
    auto* news_title = heading("What's new today", d, "star", QColor(240, 200, 60));
    news_row->addWidget(news_title, 1);
    news_row->addWidget(link_button("All releases", kReleases, d));
    mid->addLayout(news_row);
    auto* notes = new QTextBrowser(d);
    notes->setOpenExternalLinks(true);
    mid->addWidget(notes, 3);
    auto* ann_row = new QHBoxLayout();
    ann_row->addWidget(heading("Announcements", d, "megaphone", QColor(220, 90, 80)), 1);
    ann_row->addWidget(link_button("All announcements", kAnnouncementsPage, d));
    mid->addLayout(ann_row);
    auto* ann = new QListWidget(d);
    connect(ann, &QListWidget::itemActivated, [](QListWidgetItem* it) {
        const QString url = it->data(Qt::UserRole).toString();
        if (!url.isEmpty()) QDesktopServices::openUrl(QUrl(url));
    });
    mid->addWidget(ann, 2);

    auto load_news = [d, notes, ann, news_title] {
        notes->setPlainText("Asking GitHub...");
        ann->clear();
        // Every release of the newest one's day, newest first: there are often several a day.
        // AZTEK_RELEASES_URL stands in for GitHub's list, as AZTEK_UPDATE_URL does for the update check.
        fetch(d, qEnvironmentVariable("AZTEK_RELEASES_URL", kReleasesApi), [notes, news_title](const QJsonDocument& doc) {
            if (!doc.isArray()) {
                notes->setPlainText("The release notes couldn't be fetched: " + doc.object()["error"].toString("no answer"));
                return;
            }
            QList<QJsonObject> list;
            for (const QJsonValue& v : doc.array())
                if (!v.toObject()["draft"].toBool()) list << v.toObject();
            std::sort(list.begin(), list.end(), [](const QJsonObject& a, const QJsonObject& b) { return a["published_at"].toString() > b["published_at"].toString(); });
            if (list.isEmpty()) {
                notes->setPlainText("No releases yet.");
                return;
            }
            auto when = [](const QJsonObject& r) { return QDateTime::fromString(r["published_at"].toString(), Qt::ISODate).toLocalTime(); };
            const QDate day = when(list.first()).date();
            if (auto* l = news_title->findChild<QLabel*>("title"); l && day != QDate::currentDate()) l->setText("What's new on " + day.toString("dd/MM/yyyy"));
            const QString mine = QString("v2.%1.0").arg(build_number());
            QString md;
            int count = 0;
            for (const QJsonObject& r : list) {
                if (when(r).date() != day) break;
                ++count;
                md += QString("### %1, %2%3\n\n").arg(r["name"].toString(r["tag_name"].toString()), when(r).toString("HH:mm"),
                                                      r["tag_name"].toString() == mine ? "  (this version)" : "");
                // The notes' own headings sit below each release's.
                QString body = r["body"].toString().trimmed();
                body.replace(QRegularExpression("(^|\\n)#{1,3} "), "\\1#### ");
                md += body + "\n\n---\n\n";
            }
            notes->setMarkdown(QString("%1 release%2 that day.\n\n").arg(count).arg(count == 1 ? "" : "s") + md);
        });
        fetch(d, kAnnouncements, [ann](const QJsonDocument& doc) {
            const QJsonArray list = doc.isArray() ? doc.array() : doc.object()["announcements"].toArray();
            if (list.isEmpty()) {
                // Unreachable, or reached with nothing in it: which, and the way to them on GitHub either way.
                const QString why = doc.object().contains("error") ? "The announcements couldn't be fetched: " + doc.object()["error"].toString()
                                    : doc.isNull()                 ? QString("The announcements couldn't be fetched")
                                                                   : QString("The website lists no announcements yet");
                auto* it = new QListWidgetItem(why, ann);
                it->setFlags(Qt::NoItemFlags);
                auto* open = new QListWidgetItem("Open the announcements on GitHub", ann);
                open->setData(Qt::UserRole, QString(kAnnouncementsPage));
                open->setToolTip(kAnnouncementsPage);
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
    right->addWidget(heading("Links", d, "globe", QColor(95, 160, 220)));
    right->addWidget(link_button("Website", kSite, d, "globe", QColor(95, 160, 220)));
    right->addWidget(link_button("Read me and documentation", "https://github.com/srlegrand/Aztek#readme", d, "book", QColor(110, 200, 120)));
    right->addWidget(link_button("Release notes", kReleases, d, "star", QColor(240, 200, 60)));
    right->addWidget(link_button("Discussions", "https://github.com/srlegrand/Aztek/discussions", d, "chat", QColor(150, 170, 190)));
    right->addWidget(link_button("Discord", "https://discord.gg/XDEgpWDVKa", d, "chat", QColor(120, 130, 230)));
    right->addWidget(link_button("Report an issue", "https://github.com/srlegrand/Aztek/issues/new/choose", d, "bug", QColor(220, 90, 80)));
    right->addWidget(link_button("Support on Ko-fi", "https://ko-fi.com/turtleface", d, "heart", QColor(240, 110, 140)));
    // The greyhound, under the links: a new pose with each version.
    {
        const int n = int(sizeof(kPoses) / sizeof(kPoses[0]));
        const int pose = build_number() % n;
        auto* dog = new QLabel(d);
        QPixmap dpm(QString(":/greyhound/%1.png").arg(kPoses[pose]));
        if (!dpm.isNull()) {
            dpm.setDevicePixelRatio(2.0);
            dog->setPixmap(dpm);
        }
        dog->setToolTip(QString("Pose %1 of %2: it changes with every version.").arg(pose + 1).arg(n));
        right->addSpacing(8);
        right->addWidget(dog, 0, Qt::AlignHCenter | Qt::AlignTop);
    }
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

    // Right at the foot: how far towards parity, from the reference's commands and flags that Aztek takes.
    {
        const QJsonObject p = ask("aztekParity", QJsonObject{{"query", true}}).toObject();
        const double percent = p["percent"].toDouble();
        auto* row = new QHBoxLayout();
        auto* label = new QLabel(QString("<b>Parity %1%</b>").arg(percent, 0, 'f', 1), d);
        auto* bar = new QProgressBar(d);
        bar->setObjectName("parityBar");
        bar->setRange(0, 1000);
        bar->setValue(int(percent * 10));
        bar->setTextVisible(false);
        bar->setFixedHeight(14);
        label->setToolTip(QString("Of the reference's %1 commands, Aztek takes %2, and %3 of their %4 flags. Each command and each flag "
                                "counts once; aztekParity -q lists what is missing.")
                            .arg(p["commands"].toInt()).arg(p["commandsHave"].toInt()).arg(p["flagsHave"].toInt()).arg(p["flags"].toInt()));
        bar->setToolTip(label->toolTip());
        bar->setStyleSheet("QProgressBar#parityBar { border: 1px solid rgba(128,128,128,0.6); border-radius: 6px; font-weight: 600; }"
                           "QProgressBar#parityBar::chunk { border-radius: 5px; background: qlineargradient(x1:0, y1:0, x2:1, y2:0,"
                           " stop:0 #b98b55, stop:1 #e9c48c); }");
        row->addWidget(new QLabel("0%", d));
        row->addWidget(bar, 1);
        row->addWidget(new QLabel("100%", d));
        row->addSpacing(8);
        row->addWidget(label);
        outer->addLayout(row);
    }

    d->show();
    d->raise();
}

} // namespace az

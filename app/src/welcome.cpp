// The welcome screen (#85), as the reference has one at start-up that can be turned off: ways in (a new scene, a scene
// to open, the recent ones), the example scenes and scripts that come with Aztek, useful links, and from GitHub the
// notes of the day's releases and the announcements. GitHub is asked only when update checks are on (Preferences >
// Interface), as nothing leaves the machine otherwise; a button asks once on demand. A greyhound keeps it company
// under the links, and a bar along the foot shows how far Aztek is towards parity.

#include "mainwindow.h"
#include "examples.h"
#include "aztek.h"
#include "host.h"
#include "icons.h"
#include "panels.h"
#include "widgets.h"

#include <QCheckBox>
#include <QDialogButtonBox>
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
#include <QSet>
#include <QSettings>
#include <QTextBrowser>
#include <QThread>
#include <QTimer>
#include <QTime>
#include <QTreeWidget>
#include <QUrl>
#include <QVBoxLayout>

#include <algorithm>
#include <cmath>
#include <map>
#include <memory>

namespace az {

namespace {

const char* kSite = "https://srlegrand.github.io/Aztek/";
const char* kAnnouncements = "https://srlegrand.github.io/Aztek/announcements.json";
const char* kAnnouncementsPage = "https://github.com/srlegrand/Aztek/discussions/categories/announcements";
const char* kReleases = "https://github.com/srlegrand/Aztek/releases";
const char* kRepo = "https://github.com/srlegrand/Aztek";
const char* kCommitsApi = "https://api.github.com/repos/srlegrand/Aztek/commits?sha=main&per_page=100";
const char* kCommitHistoryApi = "https://api.github.com/repos/srlegrand/Aztek/commits?sha=%1&per_page=100";
const char* kCompareApi = "https://api.github.com/repos/srlegrand/Aztek/compare/%1...%2";
const char* kReleasesApi = "https://api.github.com/repos/srlegrand/Aztek/releases?per_page=100";
const char* kIssuesApi = "https://api.github.com/repos/srlegrand/Aztek/issues?state=all&per_page=100&page=";
const char* kOwner = "srlegrand";

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

// A line of text going by, right to left, round and round; the mouse over it holds it still (#148).
class Ticker : public QWidget {
public:
    explicit Ticker(QWidget* parent) : QWidget(parent) {
        setMinimumWidth(240);
        setFixedHeight(fontMetrics().height() + 6);
        connect(&timer_, &QTimer::timeout, this, [this] {
            if (!underMouse()) {
                offset_ += 1;
                update();
            }
        });
        timer_.start(33);
    }
    void set_text(const QString& t) {
        text_ = t;
        setToolTip(t);
        update();
    }

protected:
    void paintEvent(QPaintEvent*) override {
        if (text_.isEmpty()) return;
        QPainter p(this);
        p.setRenderHint(QPainter::TextAntialiasing);
        const int gap = 48;
        const int span = fontMetrics().horizontalAdvance(text_) + gap;
        const int y = (height() + fontMetrics().ascent() - fontMetrics().descent()) / 2;
        for (int x = -(offset_ % span); x < width(); x += span) p.drawText(x, y, text_);
        // Soft ends, so names come and go rather than being cut.
        const QColor bg = palette().color(QPalette::Window);
        QColor clear = bg;
        clear.setAlpha(0);
        for (bool right : {false, true}) {
            QLinearGradient g(right ? width() - 24 : 24, 0, right ? width() : 0, 0);
            g.setColorAt(0, clear);
            g.setColorAt(1, bg);
            p.fillRect(right ? QRect(width() - 24, 0, 24, height()) : QRect(0, 0, 24, height()), g);
        }
    }

private:
    QString text_;
    int offset_ = 0;
    QTimer timer_;
};

// Who has opened issues on GitHub, most first, leaving out the owner and bots: every page of the repository's issues,
// then done with the names. Kept, so the last list shows while offline.
void fetch_reporters(QObject* owner, std::function<void(const QStringList&)> done, int page = 1,
                     std::shared_ptr<std::map<QString, int>> counts = std::make_shared<std::map<QString, int>>()) {
    const QString base = qEnvironmentVariable("AZTEK_ISSUES_URL", kIssuesApi);
    fetch(owner, base + QString::number(page), [owner, done, page, counts](const QJsonDocument& doc) {
        const QJsonArray a = doc.array();
        for (const QJsonValue& v : a) {
            const QString login = v.toObject()["user"].toObject()["login"].toString();
            if (login.isEmpty() || login == kOwner || login.endsWith("[bot]")) continue;
            ++(*counts)[login];
        }
        if (a.size() >= 100 && page < 10) {
            fetch_reporters(owner, done, page + 1, counts);
            return;
        }
        std::vector<std::pair<QString, int>> v(counts->begin(), counts->end());
        std::stable_sort(v.begin(), v.end(), [](const auto& x, const auto& y) { return x.second > y.second; });
        QStringList names;
        for (const auto& [n, c] : v) names << n;
        done(names);
    });
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

// The welcome screen open now, if one is.
static QPointer<QDialog> open;

void MainWindow::app_home(const QJsonObject& o) {
    if (o.contains("iconVisible") && home_button_) home_button_->setVisible(o["iconVisible"].toBool());
    if (o.contains("visible")) {
        const bool want = o["visible"].toBool();
        if (want && !open) show_welcome(false);
        if (!want && open) open->close();
    }
    // Made again, its recent files are read afresh.
    if (o["updateRecentFiles"].toBool() && open) show_welcome(false);
    // A tab: the section of that name takes the focus.
    if (o.contains("tab") && open) {
        if (QWidget* w = open->findChild<QWidget*>("home" + o["tab"].toString())) {
            open->raise();
            open->activateWindow();
            w->setFocus(Qt::OtherFocusReason);
        }
    }
}

void MainWindow::show_welcome(bool at_startup) {
    if (at_startup && !QSettings().value("welcome/atStartup", true).toBool()) return;
    // Opened again, it's made again where it was, so the announcements and the names going by are fetched afresh (#157).
    QRect was;
    if (open) {
        was = open->geometry();
        // Made again: the old one going doesn't hide the home.
        QObject::disconnect(open, &QDialog::finished, this, nullptr);
        open->close();
    }
    auto* d = new QDialog(this, editor_window_flags());
    open = d;
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setObjectName("welcome");
    d->setWindowTitle("Welcome to Aztek");
    // appHome -q -v reads whether it is up.
    exec_quiet("appHome", QJsonObject{{"edit", true}, {"visible", true}});
    connect(d, &QDialog::finished, this, [this] { exec_quiet("appHome", QJsonObject{{"edit", true}, {"visible", false}}); });
    d->resize(980, 640);
    if (!was.isNull()) d->setGeometry(was);
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
        top->addWidget(title);
        top->addSpacing(24);
        // Top right: thanks to the people reporting issues, their GitHub names going by (#148). It takes all the width
        // the title leaves, so more names show at once.
        auto* thanks = new QLabel("<b>Thanks to:</b>", d);
        auto* names = new Ticker(d);
        names->setMinimumWidth(300);
        auto show = [thanks, names](const QStringList& list) {
            thanks->setVisible(!list.isEmpty());
            names->setVisible(!list.isEmpty());
            names->set_text(list.join("   \u00b7   "));
        };
        show(QSettings().value("welcome/thanks").toStringList());
        fetch_reporters(d, [show](const QStringList& list) {
            if (list.isEmpty()) return;
            QSettings().setValue("welcome/thanks", list);
            show(list);
        });
        auto* thanks_col = new QVBoxLayout();
        auto* thanks_row = new QHBoxLayout();
        thanks_row->addWidget(thanks);
        thanks_row->addWidget(names, 1);
        thanks_col->addLayout(thanks_row);
        thanks_col->addStretch(1);
        top->addLayout(thanks_col, 1);
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
        fresh->setObjectName("homeGettingStarted");
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
        list->setObjectName("homeRecent");
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
    examples->setObjectName("homeLearning");
    examples->setHeaderHidden(true);
    examples->setRootIsDecorated(true);
    const QString root = examples_folder();
    if (root.isEmpty()) {
        auto* it = new QTreeWidgetItem(examples, {"The examples couldn't be written to your data folder"});
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
    auto* news_title = heading("What's new", d, "star", QColor(240, 200, 60));
    news_row->addWidget(news_title, 1);
    news_row->addWidget(link_button("All releases", kReleases, d));
    mid->addLayout(news_row);
    auto* notes = new QTextBrowser(d);
    notes->setObjectName("homeWhatsNew");
    notes->setOpenExternalLinks(true);
    mid->addWidget(notes, 3);
    auto* ann_row = new QHBoxLayout();
    ann_row->addWidget(heading("Announcements", d, "megaphone", QColor(220, 90, 80)), 1);
    ann_row->addWidget(link_button("All announcements", kAnnouncementsPage, d));
    mid->addLayout(ann_row);
    auto* ann = new QListWidget(d);
    ann->setObjectName("homeCommunity");
    connect(ann, &QListWidget::itemActivated, [](QListWidgetItem* it) {
        const QString url = it->data(Qt::UserRole).toString();
        if (!url.isEmpty()) QDesktopServices::openUrl(QUrl(url));
    });
    mid->addWidget(ann, 2);

    auto load_news = [d, notes, ann] {
        notes->setPlainText("Asking GitHub...");
        ann->clear();
        // The changes themselves, newest first, under the release each went out in (#135). A release's own notes are
        // what it shipped, so a commit merged after it (which GitHub's date-ordered commit list puts before it) never
        // lands in the older release. The commits after the newest release this build includes are asked of the
        // compare API. AZTEK_COMMITS_URL and AZTEK_RELEASES_URL stand in for GitHub's lists, as AZTEK_UPDATE_URL does
        // for the update check.
        const QRegularExpressionMatch own = QRegularExpression("\\(([0-9a-f]{7,40})\\)").match(QString(AZTEK_VERSION));
        const QString mine = own.hasMatch() ? own.captured(1) : QString();
        const QString commits_url = mine.isEmpty() ? QString(kCommitsApi) : QString(kCommitHistoryApi).arg(mine);
        auto with_commits = [d, notes, mine](const QJsonDocument& commits) {
            if (!commits.isArray()) {
                notes->setPlainText("The changes couldn't be fetched: " + commits.object()["error"].toString("no answer"));
                return;
            }
            fetch(d, qEnvironmentVariable("AZTEK_RELEASES_URL", kReleasesApi), [d, notes, commits, mine](const QJsonDocument& releases) {
                if (!releases.isArray()) {
                    notes->setPlainText("The releases couldn't be fetched: " + releases.object()["error"].toString("no answer"));
                    return;
                }
                // The commits this build has: a release whose commit is among them is one it includes.
                QSet<QString> listed;
                for (const QJsonValue& v : commits.array()) listed.insert(v.toObject()["sha"].toString());
                const QJsonArray rels = releases.array();
                // The newest release this build includes: the commits after it are the unreleased ones.
                QString base;
                for (const QJsonValue& v : rels) {
                    const QJsonObject r = v.toObject();
                    if (r["draft"].toBool()) continue;
                    const QString target = r["target_commitish"].toString();
                    if (listed.contains(target) || (!mine.isEmpty() && target.startsWith(mine))) {
                        base = target;
                        break;
                    }
                }
                auto when = [](const QString& iso) { return QDateTime::fromString(iso, Qt::ISODate).toLocalTime(); };
                auto render = [notes, mine, rels, listed, when](const QJsonArray& unreleased) {
                    auto linked = [](QString text) {
                        static const QRegularExpression issue("#(\\d+)");
                        text.replace(issue, QString("[#\\1](%1/issues/\\1)").arg(kRepo));
                        return text;
                    };
                    QString md;
                    int shown = 0;
                    if (!unreleased.isEmpty()) {
                        const QString running = mine.isEmpty() ? QString("Not released yet")
                            : QString("%1  (this version) -- not released yet").arg(QString(AZTEK_VERSION).toHtmlEscaped());
                        md += "\n### " + running + "\n\n";
                        for (const QJsonValue& v : unreleased) {
                            if (shown >= 100) break;
                            const QJsonObject c = v.toObject();
                            QString message = c["commit"].toObject()["message"].toString().trimmed().split("\n\n").first();
                            message.replace('\n', ' ');
                            md += "- " + linked(message.toHtmlEscaped()) + "\n";
                            ++shown;
                        }
                    }
                    // The releases this build includes, newest first, in their own notes.
                    for (const QJsonValue& v : rels) {
                        const QJsonObject r = v.toObject();
                        if (r["draft"].toBool()) continue;
                        const QString target = r["target_commitish"].toString();
                        const bool this_version = !mine.isEmpty() && target.startsWith(mine);
                        if (!listed.contains(target) && !this_version) continue;
                        const QString name = r["name"].toString(r["tag_name"].toString());
                        md += QString("\n### %1%2, %3\n\n").arg(name, this_version ? "  (this version)" : QString(),
                                                                    when(r["published_at"].toString()).toString("dd/MM/yyyy HH:mm"));
                        const QString body = linked(r["body"].toString().trimmed());
                        QStringList kept;
                        bool full = false;
                        for (const QString& line : body.split('\n')) {
                            if (line.startsWith("- ")) {
                                if (shown >= 100) {
                                    full = true;
                                    break;
                                }
                                ++shown;
                            }
                            kept << line;
                        }
                        if (!kept.isEmpty()) md += kept.join('\n') + "\n";
                        if (full) break;
                    }
                    notes->setMarkdown(md.isEmpty() ? QString("No changes yet.") : md);
                };
                if (base.isEmpty()) {
                    // No release among the commits this build has (a very old or a fork build): the commits stand alone.
                    QJsonArray recent;
                    for (const QJsonValue& v : commits.array()) {
                        const QJsonObject c = v.toObject();
                        if (c["parents"].toArray().size() > 1) continue;
                        recent.append(c);
                    }
                    render(recent);
                } else if (!mine.isEmpty() && !base.startsWith(mine)) {
                    // The commits after the newest release this build includes; the compare lists them oldest first.
                    fetch(d, QString(kCompareApi).arg(base, mine), [render](const QJsonDocument& compared) {
                        const QJsonArray commits = compared.object()["commits"].toArray();
                        QJsonArray unreleased;
                        for (int i = commits.size() - 1; i >= 0; --i) {
                            const QJsonObject c = commits[i].toObject();
                            if (c["parents"].toArray().size() > 1) continue;
                            unreleased.append(c);
                        }
                        render(unreleased);
                    });
                } else {
                    render(QJsonArray());
                }
            });
        };
        fetch(d, qEnvironmentVariable("AZTEK_COMMITS_URL", commits_url), [d, with_commits, commits_url](const QJsonDocument& commits) {
            if (!commits.isArray() && qEnvironmentVariableIsEmpty("AZTEK_COMMITS_URL") && commits_url != QString(kCommitsApi)) {
                // A build from a fork: its commit isn't in the upstream's history, so show the upstream's main instead.
                fetch(d, kCommitsApi, with_commits);
                return;
            }
            with_commits(commits);
        });
        fetch(d, qEnvironmentVariable("AZTEK_ANNOUNCEMENTS_URL", kAnnouncements), [ann](const QJsonDocument& doc) {
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
    // The greyhound, under the links.
    {
        auto* dog = new QLabel(d);
        QPixmap dpm(":/greyhound.png");
        if (!dpm.isNull()) {
            dpm.setDevicePixelRatio(2.0);
            dog->setPixmap(dpm);
        }
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

    // Right at the foot: how far towards parity, from the parity ledger's summary.
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
        // The parity ledger's areas (parity/ in the source), each done over the rows that apply, partial counting half.
        QString tip = "<b>Parity ledger</b><table>";
        for (const QJsonValue& v : p["areas"].toArray()) {
            const QJsonObject a = v.toObject();
            if (a["rows"].toInt() == 0)
                continue;
            tip += QString("<tr><td>%1</td><td align=right>&nbsp;%2 of %3&nbsp;</td><td align=right>%4%</td></tr>")
                       .arg(a["area"].toString().replace('_', ' '))
                       .arg(a["done"].toInt())
                       .arg(a["rows"].toInt() - a["na"].toInt())
                       .arg(a["percent"].toDouble(), 0, 'f', 1);
        }
        tip += "</table>The figure is the areas' weighted mean; areas with no rows yet don't count.";
        label->setToolTip(tip);
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

// Help > Thanks: the names the welcome screen's ticker goes through, each a link to its GitHub page. The last list kept
// shows at once; the fresh one replaces it when it comes.
void show_thanks(QWidget* parent) {
    auto* d = new QDialog(parent);
    d->setAttribute(Qt::WA_DeleteOnClose);
    d->setWindowTitle("Thanks");
    auto* v = new QVBoxLayout(d);
    auto* names = new QTextBrowser(d);
    names->setOpenExternalLinks(true);
    v->addWidget(names, 1);
    auto fill = [names](const QStringList& list) {
        QStringList links;
        for (const QString& n : list)
            links << QString("<a href=\"https://github.com/%1\" style=\"color:#7fb4d6\">%1</a>").arg(n.toHtmlEscaped());
        names->setHtml(QString("<p>Thanks to everyone on GitHub who has helped make Aztek:</p><p style=\"line-height:150%\">%1</p>")
                           .arg(links.isEmpty() ? QString("<i>Fetching the names from GitHub...</i>") : links.join("&nbsp;&nbsp;\u00b7&nbsp;&nbsp; ")));
    };
    fill(QSettings().value("welcome/thanks").toStringList());
    fetch_reporters(d, [fill](const QStringList& list) {
        if (list.isEmpty()) return;
        QSettings().setValue("welcome/thanks", list);
        fill(list);
    });
    auto* ok = new QDialogButtonBox(QDialogButtonBox::Ok, d);
    QObject::connect(ok, &QDialogButtonBox::accepted, d, &QDialog::accept);
    v->addWidget(ok);
    d->resize(560, 420);
    d->show();
}

} // namespace az

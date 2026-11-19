// The report window: text, optional screenshot with drawing on top, optional zipped copy of the scene. Everything goes
// through the person's browser: see feedback.h. The screenshot stays in memory until they press Open on GitHub.
#include "feedback.h"

#include "aztek.h"
#include "host.h"
#include "viewport.h"

#include <QApplication>
#include <QCheckBox>
#include <QClipboard>
#include <QComboBox>
#include <QCursor>
#include <QDateTime>
#include <QDesktopServices>
#include <QDialog>
#include <QDir>
#include <QFile>
#include <QFileDialog>
#include <QFileInfo>
#include <QMimeData>
#include <QDragEnterEvent>
#include <QDropEvent>
#include <QElapsedTimer>
#include <QTabBar>
#include <QHBoxLayout>
#include <QImage>
#include <QInputDialog>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QLineEdit>
#include <QMessageBox>
#include <QMouseEvent>
#include <QPainter>
#include <QPainterPath>
#include <QPlainTextEdit>
#include <QPointer>
#include <QProcess>
#include <QPushButton>
#include <QScreen>
#include <QSettings>
#include <QSpinBox>
#include <QStandardPaths>
#include <QSysInfo>
#include <QTemporaryDir>
#include <QTimer>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>
#include <QtGlobal>

#include <algorithm>
#include <cmath>
#include <functional>
#include <vector>

#ifndef AZTEK_VERSION
#define AZTEK_VERSION "dev"
#endif

namespace az {

namespace {

const char* const kPublicWarning =
    "This will be posted to the public GitHub bug tracker, where anyone can see it. Never include work under NDA or anything "
    "confidential. If you do, it is at your own risk.";
const char* const kSceneWarning =
    "The scene file holds the names of everything in it and may hold file paths. Referenced files and textures are not included.";
const qint64 kFileLimit = 25 * 1024 * 1024; // GitHub's limit for attached files.
const qint64 kImageLimit = 10 * 1024 * 1024; // And for images, and videos.
// A report's video (#153): 720p at 24 frames a second, 3, 5 or 10 seconds, at a rate that keeps 10 seconds near 3 MB.
const int kVideoWidth = 1280, kVideoHeight = 720, kVideoFps = 24, kVideoBits = 2500000;
const char* const kOwnWindow = "feedbackWindow";

// Whichever windows of the app are showing, viewports included, on one picture at their places on the screen. Report
// windows are left out. In memory only: a viewport's pixels pass through a temporary file that goes straight away.
QImage capture_app(QWidget* main_window) {
    QList<QWidget*> windows{main_window};
    for (QWidget* w : QApplication::topLevelWidgets()) {
        if (w == main_window || !w->isVisible() || w->isMinimized() || w->property(kOwnWindow).toBool()) continue;
        const Qt::WindowType type = w->windowType();
        if (type == Qt::Popup || type == Qt::ToolTip || type == Qt::SplashScreen) continue;
        windows << w;
    }
    // Wayland doesn't say where windows are, so they go one under another.
    const bool stacked = QGuiApplication::platformName().startsWith("wayland");
    QList<QPoint> at;
    QRect all;
    int below = 0;
    for (QWidget* w : windows) {
        QPoint p = stacked ? QPoint(0, below) : w->mapToGlobal(QPoint(0, 0));
        below += w->height();
        at << p;
        all |= QRect(p, w->size());
    }
    const qreal dpr = main_window->devicePixelRatioF();
    QImage out(all.size() * dpr, QImage::Format_RGB32);
    out.setDevicePixelRatio(dpr);
    out.fill(QColor(43, 43, 43));
    QPainter p(&out);
    QTemporaryDir tmp;
    for (int i = 0; i < windows.size(); ++i) {
        QWidget* w = windows[i];
        const QPoint origin = at[i] - all.topLeft();
        p.drawPixmap(origin, w->grab());
        for (ViewportWidget* v : w->findChildren<ViewportWidget*>()) {
            if (!v->isVisible() || !tmp.isValid()) continue;
            const QString file = tmp.filePath("view.ppm");
            if (!v->capture(file)) continue;
            p.drawImage(QRect(origin + v->mapTo(w, QPoint(0, 0)), v->size()), QImage(file));
            QFile::remove(file);
        }
    }
    p.end();
    out.setDevicePixelRatio(1.0);
    return out;
}

enum class Tool { Pen, Arrow, Rectangle, Text, Crop };

struct Mark {
    Tool tool = Tool::Pen;
    QColor colour;
    int width = 4;
    QVector<QPointF> points;
    QString text;
};

void draw_mark(QPainter& p, const Mark& m) {
    QPen pen(m.colour, m.width, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin);
    p.setPen(pen);
    p.setBrush(Qt::NoBrush);
    if (m.points.isEmpty()) return;
    const QPointF a = m.points.first(), b = m.points.last();
    switch (m.tool) {
    case Tool::Pen:
        if (m.points.size() == 1) p.drawPoint(a);
        else p.drawPolyline(m.points.constData(), m.points.size());
        break;
    case Tool::Rectangle:
        p.drawRect(QRectF(a, b).normalized());
        break;
    case Tool::Arrow: {
        p.drawLine(a, b);
        const QPointF d = b - a;
        const double len = std::hypot(d.x(), d.y());
        if (len < 1) break;
        const QPointF unit = d / len, side(-unit.y(), unit.x());
        const double head = std::min(len, 10.0 + m.width * 3.0);
        QPainterPath tip;
        tip.moveTo(b);
        tip.lineTo(b - unit * head + side * head * 0.5);
        tip.lineTo(b - unit * head - side * head * 0.5);
        tip.closeSubpath();
        p.fillPath(tip, m.colour);
        break;
    }
    case Tool::Crop: {
        // Only while dragging: the part that will be kept.
        QPen dash(QColor(255, 255, 255), 1, Qt::DashLine);
        dash.setCosmetic(true);
        p.setPen(dash);
        p.drawRect(QRectF(a, b).normalized());
        break;
    }
    case Tool::Text: {
        QFont f = p.font();
        f.setPixelSize(12 + m.width * 3);
        f.setBold(true);
        p.setFont(f);
        p.drawText(a, m.text);
        break;
    }
    }
}

// The pictures of a report and what is drawn on each, in image coordinates, kept apart so undo is simple and lines are
// sharp at full size. base, marks and undo_steps are the picture being worked on; the others wait in `pictures`.
class Annotation : public QObject {
    Q_OBJECT
public:
    struct Step {
        QImage base;
        std::vector<Mark> marks;
    };
    struct Picture {
        QImage base;
        std::vector<Mark> marks;
        std::vector<Step> undo;
    };
    QImage base;
    std::vector<Mark> marks;
    std::vector<Step> undo_steps;
    std::vector<Picture> pictures{Picture{}};
    int current = 0;
    Tool tool = Tool::Pen;
    QColor colour = QColor(230, 30, 30);
    int width = 4;

    void remember() { undo_steps.push_back({base, marks}); }
    void undo() {
        if (undo_steps.empty()) return;
        base = undo_steps.back().base;
        marks = undo_steps.back().marks;
        undo_steps.pop_back();
        emit changed();
    }
    void clear() {
        if (marks.empty()) return;
        remember();
        marks.clear();
        emit changed();
    }
    // A fresh capture in place of the picture being worked on.
    void set_picture(const QImage& image) {
        base = image;
        marks.clear();
        undo_steps.clear();
        emit changed();
    }
    // Keeps only this part of the picture, and moves what is drawn with it.
    void crop(QRectF r) {
        const QRect keep = r.toAlignedRect().intersected(base.rect());
        if (keep.width() < 4 || keep.height() < 4) return;
        remember();
        base = base.copy(keep);
        for (Mark& m : marks)
            for (QPointF& pt : m.points) pt -= keep.topLeft();
        emit changed();
    }
    int count() const { return int(pictures.size()); }
    void select(int i) {
        if (i < 0 || i >= count() || i == current) return;
        stash();
        current = i;
        base = pictures[i].base;
        marks = pictures[i].marks;
        undo_steps = pictures[i].undo;
        emit changed();
        emit pictures_changed();
    }
    // Another picture after the others, which becomes the one worked on.
    void add(const QImage& image) {
        if (image.isNull()) return;
        stash();
        pictures.push_back(Picture{image.convertToFormat(QImage::Format_RGB32), {}, {}});
        current = -1;
        select(count() - 1);
    }
    void remove_current() {
        if (count() <= 1) {
            set_picture(QImage());
            emit pictures_changed();
            return;
        }
        pictures.erase(pictures.begin() + current);
        const int next = qMin(current, count() - 1);
        current = -1;
        base = pictures[next].base;
        marks = pictures[next].marks;
        undo_steps = pictures[next].undo;
        current = next;
        emit changed();
        emit pictures_changed();
    }
    // Every picture with its drawing on, the empty ones left out.
    QList<QImage> composites() {
        stash();
        QList<QImage> out;
        for (const Picture& pic : pictures) {
            if (pic.base.isNull()) continue;
            QImage img = pic.base.convertToFormat(QImage::Format_RGB32);
            QPainter p(&img);
            p.setRenderHint(QPainter::Antialiasing);
            for (const Mark& m : pic.marks) draw_mark(p, m);
            out << img;
        }
        return out;
    }
signals:
    void changed();
    void settings_changed();
    void pictures_changed();

private:
    void stash() {
        if (current < 0 || current >= count()) return;
        pictures[current] = Picture{base, marks, undo_steps};
    }
};

// An image from a drop or the clipboard: image data, or the first file that reads as an image.
QImage image_from(const QMimeData* d) {
    if (!d) return QImage();
    if (d->hasImage()) {
        QImage img = qvariant_cast<QImage>(d->imageData());
        if (!img.isNull()) return img;
    }
    for (const QUrl& u : d->urls()) {
        if (!u.isLocalFile()) continue;
        QImage img(u.toLocalFile());
        if (!img.isNull()) return img;
    }
    return QImage();
}

// The picture scaled to fit, to draw on.
class Canvas : public QWidget {
public:
    explicit Canvas(Annotation* doc, QWidget* parent = nullptr) : QWidget(parent), doc_(doc) {
        setObjectName("feedbackCanvas");
        setMinimumSize(240, 150);
        setCursor(Qt::CrossCursor);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
        setAcceptDrops(true);
        setToolTip("Drop a picture here to add it to the report.");
        connect(doc_, &Annotation::changed, this, qOverload<>(&QWidget::update));
    }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.fillRect(rect(), QColor(30, 30, 30));
        if (doc_->base.isNull()) {
            p.setPen(QColor(150, 150, 150));
            p.drawText(rect(), Qt::AlignCenter | Qt::TextWordWrap, "No picture.\nDrop one here, or use Add Picture or Paste.");
            return;
        }
        p.setRenderHint(QPainter::SmoothPixmapTransform);
        p.setRenderHint(QPainter::Antialiasing);
        const QRectF r = picture_rect();
        p.drawImage(r, doc_->base);
        p.setClipRect(r);
        p.translate(r.topLeft());
        const double s = r.width() / doc_->base.width();
        p.scale(s, s);
        for (const Mark& m : doc_->marks) draw_mark(p, m);
        if (drawing_) draw_mark(p, current_);
    }
    void mousePressEvent(QMouseEvent* e) override {
        if (e->button() != Qt::LeftButton || doc_->base.isNull()) return;
        const QPointF at = to_image(e->position());
        if (doc_->tool == Tool::Text) {
            bool ok = false;
            const QString text = QInputDialog::getText(this, "Add Text", "Text:", QLineEdit::Normal, QString(), &ok);
            if (!ok || text.trimmed().isEmpty()) return;
            Mark m = fresh();
            m.points = {at};
            m.text = text;
            doc_->remember();
            doc_->marks.push_back(m);
            emit doc_->changed();
            return;
        }
        current_ = fresh();
        current_.points = {at};
        drawing_ = true;
        update();
    }
    void mouseMoveEvent(QMouseEvent* e) override {
        if (!drawing_) return;
        const QPointF at = to_image(e->position());
        if (current_.tool == Tool::Pen) current_.points << at;
        else current_.points = {current_.points.first(), at};
        update();
    }
    void dragEnterEvent(QDragEnterEvent* e) override {
        if (!image_from(e->mimeData()).isNull()) e->acceptProposedAction();
    }
    void dropEvent(QDropEvent* e) override {
        const QImage img = image_from(e->mimeData());
        if (img.isNull()) return;
        doc_->add(img);
        e->acceptProposedAction();
    }
    void mouseReleaseEvent(QMouseEvent* e) override {
        if (e->button() != Qt::LeftButton || !drawing_) return;
        drawing_ = false;
        if (current_.tool == Tool::Crop) {
            doc_->crop(QRectF(current_.points.first(), current_.points.last()).normalized());
            update();
            return;
        }
        doc_->remember();
        doc_->marks.push_back(current_);
        emit doc_->changed();
    }

private:
    Mark fresh() const {
        Mark m;
        m.tool = doc_->tool;
        m.colour = doc_->colour;
        m.width = doc_->width;
        return m;
    }
    QRectF picture_rect() const {
        QSizeF size = QSizeF(doc_->base.size()).scaled(QSizeF(this->size()), Qt::KeepAspectRatio);
        return QRectF(QPointF((width() - size.width()) / 2, (height() - size.height()) / 2), size);
    }
    QPointF to_image(QPointF p) const {
        const QRectF r = picture_rect();
        const double s = doc_->base.width() / r.width();
        return QPointF(qBound(0.0, (p.x() - r.x()) * s, double(doc_->base.width())), qBound(0.0, (p.y() - r.y()) * s, double(doc_->base.height())));
    }
    Annotation* doc_;
    Mark current_;
    bool drawing_ = false;
};

// Tools over a canvas. Two can share one annotation: the small one in the report and the large window.
class AnnotationView : public QWidget {
public:
    explicit AnnotationView(Annotation* doc, QWidget* parent = nullptr) : QWidget(parent), doc_(doc) {
        auto* v = new QVBoxLayout(this);
        v->setContentsMargins(0, 0, 0, 0);
        auto* row = new QHBoxLayout;
        row->setSpacing(2);
        const std::pair<const char*, Tool> tools[] = {{"Pen", Tool::Pen}, {"Arrow", Tool::Arrow}, {"Rectangle", Tool::Rectangle}, {"Text", Tool::Text}, {"Crop", Tool::Crop}};
        for (const auto& [label, tool] : tools) {
            auto* b = new QToolButton(this);
            b->setText(label);
            b->setCheckable(true);
            b->setAutoExclusive(true);
            b->setChecked(doc_->tool == tool);
            if (tool == Tool::Crop) b->setToolTip("Drag around the part of the picture to keep. Undo brings the rest back.");
            Tool t = tool;
            connect(b, &QToolButton::clicked, this, [this, t] {
                doc_->tool = t;
                emit doc_->settings_changed();
            });
            tool_buttons_.push_back({b, t});
            row->addWidget(b);
        }
        colour_ = new QComboBox(this);
        const std::pair<const char*, QColor> colours[] = {{"Red", QColor(230, 30, 30)}, {"Yellow", QColor(255, 215, 0)}, {"Green", QColor(40, 200, 70)}, {"Blue", QColor(40, 130, 255)}, {"White", Qt::white}, {"Black", Qt::black}};
        for (const auto& [name, c] : colours) {
            QPixmap swatch(12, 12);
            swatch.fill(c);
            colour_->addItem(QIcon(swatch), name, c);
        }
        colour_->setCurrentIndex(colour_->findData(doc_->colour));
        connect(colour_, &QComboBox::activated, this, [this](int i) {
            doc_->colour = colour_->itemData(i).value<QColor>();
            emit doc_->settings_changed();
        });
        row->addWidget(colour_);
        width_ = new QSpinBox(this);
        width_->setRange(1, 30);
        width_->setValue(doc_->width);
        width_->setPrefix("Width ");
        connect(width_, qOverload<int>(&QSpinBox::valueChanged), this, [this](int w) {
            if (doc_->width == w) return;
            doc_->width = w;
            emit doc_->settings_changed();
        });
        row->addWidget(width_);
        auto* undo = new QToolButton(this);
        undo->setText("Undo");
        connect(undo, &QToolButton::clicked, doc_, &Annotation::undo);
        row->addWidget(undo);
        auto* clear = new QToolButton(this);
        clear->setText("Clear");
        connect(clear, &QToolButton::clicked, doc_, &Annotation::clear);
        row->addWidget(clear);
        row->addStretch(1);
        v->addLayout(row);
        v->addWidget(new Canvas(doc_, this), 1);
        // The other view's choices show here too.
        connect(doc_, &Annotation::settings_changed, this, [this] {
            for (const auto& [b, t] : tool_buttons_)
                if (t == doc_->tool) b->setChecked(true);
            colour_->setCurrentIndex(colour_->findData(doc_->colour));
            width_->setValue(doc_->width);
        });
    }

private:
    Annotation* doc_;
    std::vector<std::pair<QToolButton*, Tool>> tool_buttons_;
    QComboBox* colour_;
    QSpinBox* width_;
};

QString megabytes(qint64 bytes) {
    return QString::number(bytes / (1024.0 * 1024.0), 'f', 1) + " MB";
}

// Shows the folder, with the file selected where the system allows it.
void show_in_folder(const QString& folder, const QString& file) {
    const QString path = QDir(folder).filePath(file);
#if defined(Q_OS_WIN)
    if (!file.isEmpty() && QProcess::startDetached("explorer", {"/select,", QDir::toNativeSeparators(path)})) return;
#elif defined(Q_OS_LINUX)
    if (!file.isEmpty()) {
        QProcess p;
        p.start("dbus-send", {"--session", "--print-reply", "--dest=org.freedesktop.FileManager1", "/org/freedesktop/FileManager1",
                              "org.freedesktop.FileManager1.ShowItems", "array:string:" + QUrl::fromLocalFile(path).toString(), "string:"});
        if (p.waitForFinished(1500) && p.exitStatus() == QProcess::NormalExit && p.exitCode() == 0) return;
    }
#elif defined(Q_OS_MACOS)
    if (!file.isEmpty() && QProcess::startDetached("open", {"-R", path})) return;
#else
    Q_UNUSED(path);
#endif
    QDesktopServices::openUrl(QUrl::fromLocalFile(folder));
}

QString dry_run_file() {
    return qEnvironmentVariable("AZTEK_FEEDBACK_DRYRUN");
}

bool open_link(const QString& url) {
    if (!dry_run_file().isEmpty()) return true;
    return QDesktopServices::openUrl(QUrl::fromEncoded(url.toLatin1()));
}

// What is left to do on the page, after the browser has been asked to open it.
class NextSteps : public QDialog {
public:
    NextSteps(QWidget* parent, const QString& url, const QString& full_text, const QList<QImage>& pictures, const QString& folder, const QString& file_to_show,
              const QString& picture_note, bool shortened, bool opened, bool has_zip, bool has_video = false) : QDialog(parent), pictures_(pictures) {
        const QImage picture = pictures.isEmpty() ? QImage() : pictures.first();
        setAttribute(Qt::WA_DeleteOnClose);
        setProperty(kOwnWindow, true);
        setObjectName("feedbackNextSteps");
        setWindowTitle("Next Steps");
        auto* v = new QVBoxLayout(this);

        QStringList steps;
        steps << (opened ? "GitHub has opened in your browser with your report filled in." : "<b>The browser did not open.</b> Press Copy Link and paste it into your browser.");
        steps << "Sign in if GitHub asks you to.";
        if (shortened) steps << "Your text was too long for a link and has been shortened. Press Copy Full Text and paste it over the shortened text.";
        if (pictures.size() == 1) steps << "Click in the description box and press Ctrl+V to add the picture. It is on the clipboard. If that does nothing, drag the PNG in from the folder.";
        if (pictures.size() > 1)
            steps << QString("Click in the description box and press Ctrl+V to add the first picture; it is on the clipboard. Copy each of the others with its button below and paste it the same way, or drag the %1 PNGs in from the folder.").arg(pictures.size());
        if (!picture_note.isEmpty()) steps << picture_note;
        if (has_video) steps << "Drag <b>aztek-video.mp4</b> from the folder onto the description box.";
        if (has_zip) steps << "Drag <b>aztek-scene.zip</b> from the folder onto the description box.";
        steps << "Read it through, then press <b>Submit new issue</b>. Nothing is public until you do.";
        QString html = "<ol style=\"margin-left: 0px; -qt-list-indent: 1;\">";
        for (const QString& s : steps) html += "<li style=\"margin-bottom: 6px;\">" + s + "</li>";
        auto* list = new QLabel(html + "</ol>", this);
        list->setObjectName("feedbackSteps");
        list->setWordWrap(true);
        list->setTextFormat(Qt::RichText);
        v->addWidget(list);

        auto* row = new QHBoxLayout;
        auto add = [&](const QString& text, std::function<void()> fn) {
            auto* b = new QPushButton(text, this);
            b->setAutoDefault(false);
            connect(b, &QPushButton::clicked, this, fn);
            row->addWidget(b);
        };
        if (pictures_.size() == 1) add("Copy Screenshot Again", [this] { QApplication::clipboard()->setImage(pictures_.first()); });
        for (int i = 0; pictures_.size() > 1 && i < pictures_.size(); ++i)
            add(QString("Copy Picture %1").arg(i + 1), [this, i] { QApplication::clipboard()->setImage(pictures_[i]); });
        if (!folder.isEmpty()) add("Open Folder Again", [folder, file_to_show] {
            if (dry_run_file().isEmpty()) show_in_folder(folder, file_to_show);
        });
        add("Copy Full Text", [full_text] { QApplication::clipboard()->setText(full_text); });
        add("Copy Link", [url] { QApplication::clipboard()->setText(url); });
        row->addStretch(1);
        add("Close", [this] { close(); });
        v->addLayout(row);
        setMinimumWidth(560);
    }

private:
    QList<QImage> pictures_;
};

class LargeDrawing : public QDialog {
public:
    LargeDrawing(QWidget* parent, Annotation* doc) : QDialog(parent) {
        setAttribute(Qt::WA_DeleteOnClose);
        setProperty(kOwnWindow, true);
        setObjectName("feedbackLargeDrawing");
        setWindowTitle("Draw on the Screenshot");
        auto* v = new QVBoxLayout(this);
        v->addWidget(new AnnotationView(doc, this), 1);
        auto* row = new QHBoxLayout;
        row->addStretch(1);
        auto* done = new QPushButton("Done", this);
        done->setAutoDefault(false);
        connect(done, &QPushButton::clicked, this, &QDialog::close);
        row->addWidget(done);
        v->addLayout(row);
        resize(QSize(1100, 760).boundedTo(parent->screen()->availableSize() * 0.9));
    }
};

class FeedbackDialog : public QDialog {
public:
    FeedbackDialog(QWidget* main_window, const QImage& picture) : QDialog(main_window), main_(main_window) {
        setAttribute(Qt::WA_DeleteOnClose);
        setProperty(kOwnWindow, true);
        setObjectName("feedbackDialog");
        setWindowTitle("This doesn't feel right!");
        doc_ = new Annotation;
        doc_->setParent(this);
        doc_->base = picture;

        auto* outer = new QHBoxLayout(this);
        auto* left = new QVBoxLayout;
        outer->addLayout(left, 1);

        auto* intro = new QLabel("Say what behaves differently from what you expect. This opens a GitHub issue page in your browser, "
                                 "filled in with what you write here. Nothing goes public until you press Submit on GitHub, and you need a GitHub account to do that.", this);
        intro->setWordWrap(true);
        left->addWidget(intro);

        title_ = new QLineEdit(this);
        title_->setObjectName("feedbackTitle");
        title_->setPlaceholderText("Short title (optional: the first line of the text is used otherwise)");
        left->addWidget(title_);
        text_ = new QPlainTextEdit(this);
        text_->setObjectName("feedbackText");
        text_->setPlaceholderText("What did you do? What did you expect? What happened?");
        text_->setTabChangesFocus(true);
        left->addWidget(text_, 2);

        auto* ticks = new QHBoxLayout;
        shot_tick_ = new QCheckBox("Include a screenshot", this);
        shot_tick_->setObjectName("feedbackScreenshot");
        scene_tick_ = new QCheckBox("Include my current scene file", this);
        scene_tick_->setObjectName("feedbackScene");
        video_tick_ = new QCheckBox("Include a video", this);
        video_tick_->setObjectName("feedbackVideo");
        ticks->addWidget(shot_tick_);
        ticks->addWidget(video_tick_);
        ticks->addWidget(scene_tick_);
        ticks->addStretch(1);
        left->addLayout(ticks);

        // The video: a few seconds of the windows, recorded after the delay with this window out of the way (#153).
        video_box_ = new QWidget(this);
        auto* video_row = new QHBoxLayout(video_box_);
        video_row->setContentsMargins(0, 0, 0, 0);
        video_length_ = new QComboBox(video_box_);
        video_length_->setObjectName("feedbackVideoLength");
        for (int sec : {3, 5, 10}) video_length_->addItem(QString("%1 seconds").arg(sec), sec);
        video_length_->setCurrentIndex(qBound(0, QSettings().value("feedback/videoLength", 1).toInt(), 2));
        connect(video_length_, &QComboBox::currentIndexChanged, this, [](int i) { QSettings().setValue("feedback/videoLength", i); });
        video_delay_ = new QSpinBox(video_box_);
        video_delay_->setObjectName("feedbackVideoDelay");
        video_delay_->setRange(0, 30);
        video_delay_->setSuffix(" s");
        video_delay_->setPrefix("Delay ");
        video_delay_->setToolTip("How long to wait before recording starts, with a count in the corner of the screen.");
        video_delay_->setValue(QSettings().value("feedback/videoDelay", 3).toInt());
        connect(video_delay_, &QSpinBox::valueChanged, this, [](int v) { QSettings().setValue("feedback/videoDelay", v); });
        auto* record = new QPushButton("Record", video_box_);
        record->setObjectName("feedbackRecord");
        record->setAutoDefault(false);
        record->setToolTip("Hides this window, waits the delay and records the windows of Aztek: 720p, 24 frames a second.");
        connect(record, &QPushButton::clicked, this, &FeedbackDialog::record_later);
        video_play_ = new QPushButton("Play", video_box_);
        video_play_->setAutoDefault(false);
        video_play_->setEnabled(false);
        connect(video_play_, &QPushButton::clicked, this, [this] { QDesktopServices::openUrl(QUrl::fromLocalFile(video_path_)); });
        video_state_ = new QLabel("Not recorded yet.", video_box_);
        video_state_->setObjectName("feedbackVideoState");
        video_row->addWidget(video_length_);
        video_row->addWidget(video_delay_);
        video_row->addWidget(record);
        video_row->addWidget(video_play_);
        video_row->addWidget(video_state_, 1);
        video_box_->hide();
        left->addWidget(video_box_);

        warning_ = new QLabel(this);
        warning_->setObjectName("feedbackWarning");
        warning_->setWordWrap(true);
        warning_->hide();
        left->addWidget(warning_);

        left->addWidget(new QLabel("What the page will be filled in with:", this));
        preview_ = new QPlainTextEdit(this);
        preview_->setObjectName("feedbackPreview");
        preview_->setReadOnly(true);
        left->addWidget(preview_, 1);
        short_note_ = new QLabel("This is too long for a link, so the page opens with it shortened. Copy Full Text gets you the whole of it to paste in.", this);
        short_note_->setWordWrap(true);
        short_note_->hide();
        left->addWidget(short_note_);

        auto* buttons = new QHBoxLayout;
        copy_full_ = new QPushButton("Copy Full Text", this);
        copy_full_->setAutoDefault(false);
        copy_full_->hide();
        connect(copy_full_, &QPushButton::clicked, this, [this] { QApplication::clipboard()->setText(report_["fullBody"].toString()); });
        buttons->addWidget(copy_full_);
        buttons->addStretch(1);
        auto* cancel = new QPushButton("Cancel", this);
        cancel->setAutoDefault(false);
        connect(cancel, &QPushButton::clicked, this, &QDialog::reject);
        buttons->addWidget(cancel);
        submit_ = new QPushButton("Open on GitHub", this);
        submit_->setObjectName("feedbackSubmit");
        submit_->setAutoDefault(false);
        submit_->setToolTip("Opens the filled-in page in your browser. Nothing is posted until you press Submit there.");
        connect(submit_, &QPushButton::clicked, this, &FeedbackDialog::go);
        buttons->addWidget(submit_);
        left->addLayout(buttons);

        // The picture: only once the box is ticked and accepted.
        picture_box_ = new QWidget(this);
        auto* right = new QVBoxLayout(picture_box_);
        right->setContentsMargins(0, 0, 0, 0);
        auto* about_picture = new QLabel("Shows the main window and any editor windows that are open. Draw on it to point things out, crop it, "
                                         "or add pictures of your own: drop them on it, Add Picture or Paste.", this);
        about_picture->setWordWrap(true);
        right->addWidget(about_picture);
        // The report's pictures, one tab each.
        auto* pictures_row = new QHBoxLayout;
        tabs_ = new QTabBar(picture_box_);
        tabs_->setObjectName("feedbackPictures");
        tabs_->setExpanding(false);
        tabs_->setDrawBase(false);
        pictures_row->addWidget(tabs_, 1);
        auto* add_file = new QPushButton("Add Picture...", picture_box_);
        add_file->setAutoDefault(false);
        connect(add_file, &QPushButton::clicked, this, [this] {
            const QStringList files = QFileDialog::getOpenFileNames(this, "Add Picture", QString(), "Images (*.png *.jpg *.jpeg *.bmp *.gif *.webp)");
            for (const QString& f : files) {
                QImage img(f);
                if (img.isNull()) QMessageBox::warning(this, "Can't Read the Picture", f);
                else doc_->add(img);
            }
        });
        auto* paste = new QPushButton("Paste", picture_box_);
        paste->setAutoDefault(false);
        paste->setToolTip("Adds the picture on the clipboard.");
        connect(paste, &QPushButton::clicked, this, [this] {
            const QImage img = image_from(QApplication::clipboard()->mimeData());
            if (img.isNull()) QMessageBox::information(this, "Paste", "There is no picture on the clipboard.");
            else doc_->add(img);
        });
        auto* remove = new QPushButton("Remove", picture_box_);
        remove->setAutoDefault(false);
        remove->setToolTip("Takes this picture out of the report.");
        connect(remove, &QPushButton::clicked, doc_, &Annotation::remove_current);
        pictures_row->addWidget(add_file);
        pictures_row->addWidget(paste);
        pictures_row->addWidget(remove);
        right->addLayout(pictures_row);
        connect(tabs_, &QTabBar::currentChanged, this, [this](int i) {
            if (!syncing_) doc_->select(i);
        });
        connect(doc_, &Annotation::pictures_changed, this, &FeedbackDialog::sync_tabs);
        sync_tabs();
        right->addWidget(new AnnotationView(doc_, picture_box_), 1);
        auto* picture_row = new QHBoxLayout;
        auto* larger = new QPushButton("Open Larger...", picture_box_);
        larger->setAutoDefault(false);
        connect(larger, &QPushButton::clicked, this, [this] {
            if (!large_) large_ = new LargeDrawing(this, doc_);
            large_->show();
            large_->raise();
        });
        auto* retake = new QPushButton("Retake", picture_box_);
        retake->setAutoDefault(false);
        retake->setToolTip("Hides this window and takes the screenshot again, in place of the picture shown. What you have drawn on it is cleared.");
        connect(retake, &QPushButton::clicked, this, &FeedbackDialog::retake);
        // Another screenshot, kept beside the others: with a delay, to catch the views in the middle of something (#126).
        auto* another = new QPushButton("Take Another", picture_box_);
        another->setAutoDefault(false);
        another->setToolTip("Hides this window and takes one more screenshot, added after the others.");
        connect(another, &QPushButton::clicked, this, [this] { capture_later(true); });
        delay_ = new QSpinBox(picture_box_);
        delay_->setObjectName("feedbackDelay");
        delay_->setRange(0, 30);
        delay_->setSuffix(" s");
        delay_->setPrefix("Delay ");
        delay_->setToolTip("How long Retake and Take Another wait before the screenshot, with a count in the corner of the screen: time to start a drag or open a menu.");
        delay_->setValue(QSettings().value("feedback/delay", 0).toInt());
        connect(delay_, &QSpinBox::valueChanged, this, [](int v) { QSettings().setValue("feedback/delay", v); });
        picture_row->addWidget(larger);
        picture_row->addWidget(retake);
        picture_row->addWidget(another);
        picture_row->addWidget(delay_);
        picture_row->addStretch(1);
        right->addLayout(picture_row);
        picture_box_->hide();
        outer->addWidget(picture_box_, 2);

        connect(title_, &QLineEdit::textChanged, this, &FeedbackDialog::refresh);
        connect(text_, &QPlainTextEdit::textChanged, this, &FeedbackDialog::refresh);
        // clicked and not toggled: only the person's own click asks, and declining unticks without asking again.
        connect(shot_tick_, &QCheckBox::clicked, this, [this](bool on) { ticked(shot_tick_, on, false); });
        connect(scene_tick_, &QCheckBox::clicked, this, [this](bool on) { ticked(scene_tick_, on, true); });
        connect(video_tick_, &QCheckBox::clicked, this, [this](bool on) { ticked(video_tick_, on, false); });
        // The theme's placeholder colour is too dim to read here, and this text matters.
        QPalette hint = text_->palette();
        hint.setColor(QPalette::PlaceholderText, QColor(165, 165, 165));
        text_->setPalette(hint);
        title_->setPalette(hint);
        resize(620, 620);
        refresh();
    }

    void reject() override {
        if (!text_->toPlainText().trimmed().isEmpty()) {
            auto answer = QMessageBox::question(this, "Discard Report", "Discard what you have written?", QMessageBox::Discard | QMessageBox::Cancel, QMessageBox::Cancel);
            if (answer != QMessageBox::Discard) return;
        }
        QDialog::reject();
    }

private:
    void sync_tabs() {
        syncing_ = true;
        while (tabs_->count() > doc_->count()) tabs_->removeTab(tabs_->count() - 1);
        while (tabs_->count() < doc_->count()) tabs_->addTab(QString());
        for (int i = 0; i < tabs_->count(); ++i) tabs_->setTabText(i, QString("Picture %1").arg(i + 1));
        tabs_->setCurrentIndex(doc_->current);
        syncing_ = false;
    }

    void ticked(QCheckBox* box, bool on, bool scene) {
        // The red warning stays on the window either way; this asks once more unless told not to.
        const char* skip = "feedback/skipPublicWarning";
        if (on && !QSettings().value(skip, false).toBool()) {
            QMessageBox ask(this);
            ask.setIcon(QMessageBox::Warning);
            ask.setWindowTitle(scene ? "Include Your Scene File?" : box == video_tick_ ? "Include a Video?" : "Include a Screenshot?");
            ask.setText(QString("<b>%1</b>").arg(kPublicWarning));
            if (scene) ask.setInformativeText(kSceneWarning);
            QPushButton* yes = ask.addButton("I Understand, Include It", QMessageBox::AcceptRole);
            QPushButton* no = ask.addButton("Don't Include It", QMessageBox::RejectRole);
            ask.setDefaultButton(no);
            ask.setEscapeButton(no);
            auto* never = new QCheckBox("Don't ask again", &ask);
            ask.setCheckBox(never);
            ask.exec();
            if (ask.clickedButton() != yes) box->setChecked(false);
            else if (never->isChecked()) QSettings().setValue(skip, true);
        }
        video_box_->setVisible(video_tick_->isChecked());
        const bool shot = shot_tick_->isChecked();
        if (shot != picture_box_->isVisible()) {
            if (shot) narrow_width_ = width();
            picture_box_->setVisible(shot);
            layout()->activate();
            resize(shot ? narrow_width_ + 560 : narrow_width_, height());
            keep_on_screen();
        }
        refresh();
    }

    void keep_on_screen() {
        const QRect room = screen()->availableGeometry();
        resize(qMin(width(), room.width()), qMin(height(), room.height()));
        move(qBound(room.left(), x(), qMax(room.left(), room.right() + 1 - width())), qBound(room.top(), y(), qMax(room.top(), room.bottom() + 1 - height())));
    }

    QJsonObject build_report(const QStringList& attachments) const {
        QJsonObject call{{"title", title_->text()},
                         {"description", text_->toPlainText()},
                         {"version", AZTEK_VERSION},
                         {"os", QSysInfo::prettyProductName()},
                         {"qt", QString(qVersion())},
                         {"attachments", QJsonArray::fromStringList(attachments)}};
        const QByteArray json = QJsonDocument(call).toJson(QJsonDocument::Compact);
        return parse(take(aztek_feedback_issue(json.constData()))).toObject();
    }

    QStringList ticked_attachments() const {
        QStringList out;
        if (shot_tick_->isChecked()) out << (doc_->count() > 1 ? QString("%1 pictures").arg(doc_->count()) : QString("screenshot"));
        if (video_tick_->isChecked() && !video_path_.isEmpty()) out << "video";
        if (scene_tick_->isChecked()) out << "scene file (zip)";
        return out;
    }

    void refresh() {
        const bool has_text = !text_->toPlainText().trimmed().isEmpty();
        submit_->setEnabled(has_text);
        report_ = build_report(ticked_attachments());
        const bool shortened = report_["shortened"].toBool();
        preview_->setPlainText(has_text ? "Title: " + report_["title"].toString() + "\n\n" + report_["body"].toString() : QString());
        short_note_->setVisible(shortened);
        copy_full_->setVisible(shortened);
        const bool warn = shot_tick_->isChecked() || video_tick_->isChecked() || scene_tick_->isChecked();
        if (warn) {
            QString text = kPublicWarning;
            if (scene_tick_->isChecked()) text += QString(" ") + kSceneWarning;
            warning_->setText(text);
        }
        warning_->setVisible(warn);
    }

    void retake() {
        if (!doc_->marks.empty()) {
            auto answer = QMessageBox::question(this, "Retake Screenshot", "Taking it again clears what you have drawn. Go on?", QMessageBox::Yes | QMessageBox::Cancel, QMessageBox::Cancel);
            if (answer != QMessageBox::Yes) return;
        }
        capture_later(false);
    }

    // Hides this window, waits the delay with a count in the corner, and takes the screenshot: in place of the one shown,
    // or added after them.
    void capture_later(bool add) {
        if (large_) large_->close();
        hide();
        const int seconds = delay_ ? delay_->value() : 0;
        auto* count = new QLabel(nullptr, Qt::ToolTip | Qt::FramelessWindowHint | Qt::WindowStaysOnTopHint | Qt::WindowDoesNotAcceptFocus);
        count->setAttribute(Qt::WA_ShowWithoutActivating);
        count->setAttribute(Qt::WA_TransparentForMouseEvents);
        count->setStyleSheet("QLabel { background: #d85a1a; color: white; font-weight: bold; padding: 6px 12px; border-radius: 4px; }");
        auto place = [count, this] {
            const QRect r = main_ ? main_->frameGeometry() : QApplication::primaryScreen()->geometry();
            count->adjustSize();
            count->move(r.right() - count->width() - 16, r.bottom() - count->height() - 40);
        };
        auto left = std::make_shared<int>(seconds);
        auto* tick = new QTimer(count);
        auto shoot = [this, add, count] {
            count->hide();
            // Long enough for the window system to take this window and the count off the screen.
            QTimer::singleShot(250, this, [this, add, count] {
                QApplication::processEvents();
                const QImage shot = capture_app(main_);
                count->deleteLater();
                if (add) doc_->add(shot);
                else doc_->set_picture(shot);
                show();
                raise();
                activateWindow();
            });
        };
        if (seconds <= 0) {
            shoot();
            return;
        }
        count->setText(QString("Screenshot in %1").arg(*left));
        place();
        count->show();
        connect(tick, &QTimer::timeout, count, [count, left, tick, shoot, place] {
            if (--*left <= 0) {
                tick->stop();
                shoot();
                return;
            }
            count->setText(QString("Screenshot in %1").arg(*left));
            place();
        });
        tick->start(1000);
    }

    // A count in the corner of the main window, for the screenshot's and the video's delays.
    QLabel* corner_label(const QString& colour) {
        auto* l = new QLabel(nullptr, Qt::ToolTip | Qt::FramelessWindowHint | Qt::WindowStaysOnTopHint | Qt::WindowDoesNotAcceptFocus);
        l->setAttribute(Qt::WA_ShowWithoutActivating);
        l->setAttribute(Qt::WA_TransparentForMouseEvents);
        l->setProperty(kOwnWindow, true);
        l->setStyleSheet(QString("QLabel { background: %1; color: white; font-weight: bold; padding: 6px 12px; border-radius: 4px; }").arg(colour));
        return l;
    }
    void place_corner(QLabel* l) const {
        const QRect r = main_ ? main_->frameGeometry() : QApplication::primaryScreen()->geometry();
        l->adjustSize();
        l->move(r.right() - l->width() - 16, r.bottom() - l->height() - 40);
    }

    // Hides this window, counts the delay down and records: each frame the windows as the screenshot takes them, on a
    // 1280x720 frame, encoded as it goes. A frame that takes longer than its 24th of a second is held, so the video
    // runs as long as asked (#153).
    void record_later() {
        if (large_) large_->close();
        const int seconds = video_length_->currentData().toInt();
        const QString dir = QDir(QStandardPaths::writableLocation(QStandardPaths::AppDataLocation)).filePath("feedback");
        QDir().mkpath(dir);
        const QString path = QDir(dir).filePath("recording-" + QDateTime::currentDateTime().toString("yyyyMMdd-HHmmss") + ".mp4");
        hide();
        auto* count = corner_label("#d85a1a");
        auto left = std::make_shared<int>(video_delay_->value());
        auto start = [this, count, seconds, path] {
            void* video = aztek_video_begin(path.toUtf8().constData(), kVideoWidth, kVideoHeight, kVideoFps, kVideoBits);
            if (!video) {
                count->deleteLater();
                show();
                QMessageBox::warning(this, "Can't Record", "The video could not be started.");
                return;
            }
            count->setStyleSheet("QLabel { background: #c0202a; color: white; font-weight: bold; padding: 6px 12px; border-radius: 4px; }");
            const int frames = seconds * kVideoFps;
            auto done = std::make_shared<int>(0);
            auto clock = std::make_shared<QElapsedTimer>();
            clock->start();
            auto* timer = new QTimer(count);
            timer->setTimerType(Qt::PreciseTimer);
            auto frame = [this, count, video, frames, done, clock, timer, path, seconds] {
                // The frames this moment stands for: one, or more when the last took long.
                const int due = qMin(frames, int(clock->elapsed() * kVideoFps / 1000) + 1);
                if (*done < due) {
                    const QImage shot = capture_app(main_);
                    QImage canvas(kVideoWidth, kVideoHeight, QImage::Format_RGBA8888);
                    canvas.fill(QColor(43, 43, 43));
                    const QImage fit = shot.scaled(kVideoWidth, kVideoHeight, Qt::KeepAspectRatio, Qt::SmoothTransformation);
                    QPainter p(&canvas);
                    p.drawImage((kVideoWidth - fit.width()) / 2, (kVideoHeight - fit.height()) / 2, fit);
                    p.end();
                    while (*done < due) {
                        aztek_video_frame(video, canvas.constBits(), int(canvas.bytesPerLine()));
                        ++*done;
                    }
                }
                count->setText(QString("Recording %1 s").arg(qMax(1, seconds - int(clock->elapsed() / 1000))));
                place_corner(count);
                if (*done < frames) return;
                timer->stop();
                const qint64 bytes = aztek_video_end(video, 1);
                count->deleteLater();
                if (bytes > 0) {
                    if (!video_path_.isEmpty() && video_path_ != path) QFile::remove(video_path_);
                    video_path_ = path;
                    video_state_->setText(QString("%1 seconds, %2.").arg(seconds).arg(megabytes(bytes)));
                    video_play_->setEnabled(true);
                } else {
                    video_state_->setText("The recording failed.");
                }
                show();
                raise();
                activateWindow();
                refresh();
            };
            connect(timer, &QTimer::timeout, count, frame);
            timer->start(1000 / kVideoFps);
        };
        if (*left <= 0) {
            count->setText("Recording");
            place_corner(count);
            count->show();
            // Long enough for this window to leave the screen.
            QTimer::singleShot(250, this, start);
            return;
        }
        count->setText(QString("Recording in %1").arg(*left));
        place_corner(count);
        count->show();
        auto* tick = new QTimer(count);
        connect(tick, &QTimer::timeout, count, [this, count, left, tick, start] {
            if (--*left <= 0) {
                tick->stop();
                start();
                return;
            }
            count->setText(QString("Recording in %1").arg(*left));
            place_corner(count);
        });
        tick->start(1000);
    }

    // Writes what was ticked, opens the browser and shows what is left to do.
    void go() {
        const bool want_shot = shot_tick_->isChecked(), want_scene = scene_tick_->isChecked();
        const bool want_video = video_tick_->isChecked() && !video_path_.isEmpty() && QFileInfo::exists(video_path_);
        QString folder;
        if (want_shot || want_scene || want_video) {
            folder = QDir(QStandardPaths::writableLocation(QStandardPaths::AppDataLocation)).filePath("feedback/" + QDateTime::currentDateTime().toString("yyyyMMdd-HHmmss"));
            if (!QDir().mkpath(folder)) {
                QMessageBox::warning(this, "Can't Save", "Could not make the folder for the attachments:\n" + folder);
                return;
            }
        }
        QString zip_name;
        if (want_scene) {
            const QString zip_path = QDir(folder).filePath("aztek-scene.zip");
            QApplication::setOverrideCursor(Qt::WaitCursor);
            const QJsonObject r = parse(take(aztek_feedback_scene_zip(zip_path.toUtf8().constData()))).toObject();
            QApplication::restoreOverrideCursor();
            if (r.contains("error")) {
                QMessageBox::warning(this, "Can't Save the Scene", "The scene copy could not be written:\n" + r["error"].toString());
                return;
            }
            const qint64 bytes = qint64(r["ok"].toDouble());
            if (bytes > kFileLimit) {
                QMessageBox big(this);
                big.setIcon(QMessageBox::Warning);
                big.setWindowTitle("Scene File Too Large");
                big.setText(QString("The zipped scene is %1. GitHub takes attachments up to 25 MB, so it can't be attached.").arg(megabytes(bytes)));
                QPushButton* on = big.addButton("Go On Without the Scene File", QMessageBox::AcceptRole);
                QPushButton* back = big.addButton("Go Back", QMessageBox::RejectRole);
                big.setDefaultButton(back);
                big.setEscapeButton(back);
                big.exec();
                QFile::remove(zip_path);
                if (big.clickedButton() != on) {
                    if (!want_shot) QDir().rmdir(folder);
                    return;
                }
                if (!want_shot) {
                    QDir().rmdir(folder);
                    folder.clear();
                }
            } else {
                zip_name = "aztek-scene.zip";
            }
        }
        QString video_name;
        if (want_video) {
            if (folder.isEmpty()) {
                folder = QDir(QStandardPaths::writableLocation(QStandardPaths::AppDataLocation)).filePath("feedback/" + QDateTime::currentDateTime().toString("yyyyMMdd-HHmmss"));
                QDir().mkpath(folder);
            }
            const QString to = QDir(folder).filePath("aztek-video.mp4");
            QFile::remove(to);
            if (QFile::copy(video_path_, to)) video_name = "aztek-video.mp4";
            else QMessageBox::warning(this, "Can't Save the Video", "The video could not be copied to:\n" + to);
        }
        QList<QImage> pictures;
        QString picture_note, png_name;
        if (want_shot) {
            pictures = doc_->composites();
            for (int i = 0; i < pictures.size(); ++i) {
                const QString name = pictures.size() == 1 ? QString("aztek-screenshot.png") : QString("aztek-picture-%1.png").arg(i + 1);
                if (i == 0) png_name = name;
                const QString png_path = QDir(folder).filePath(name);
                if (!pictures[i].save(png_path, "PNG")) {
                    QMessageBox::warning(this, "Can't Save the Picture", "The picture could not be written:\n" + png_path);
                    return;
                }
                const qint64 bytes = QFileInfo(png_path).size();
                if (bytes > kImageLimit)
                    picture_note += QString("%1 is %2. GitHub takes images up to 10 MB, so it may be refused: crop it, or retake it with a smaller window. ").arg(name, megabytes(bytes));
            }
            if (!pictures.isEmpty()) QApplication::clipboard()->setImage(pictures.first());
        }

        QStringList attachments;
        if (pictures.size() == 1) attachments << "screenshot";
        if (pictures.size() > 1) attachments << QString("%1 pictures").arg(pictures.size());
        if (!video_name.isEmpty()) attachments << "video";
        if (!zip_name.isEmpty()) attachments << "scene file (zip)";
        const QJsonObject report = build_report(attachments);
        const QString url = report["url"].toString();
        const QString show_file = !zip_name.isEmpty() ? zip_name : !video_name.isEmpty() ? video_name : png_name;

        const QString dry = dry_run_file();
        if (!dry.isEmpty()) {
            QFile f(dry);
            if (f.open(QIODevice::WriteOnly | QIODevice::Text)) {
                f.write(url.toLatin1() + "\n");
                f.write("folder=" + folder.toUtf8() + "\n");
                f.write("show=" + show_file.toUtf8() + "\n");
            }
        } else if (!folder.isEmpty()) {
            show_in_folder(folder, show_file);
        }
        const bool opened = open_link(url);
        auto* steps = new NextSteps(main_, url, report["fullBody"].toString(), pictures, folder, show_file, picture_note, report["shortened"].toBool(), opened, !zip_name.isEmpty(), !video_name.isEmpty());
        steps->show();
        QDialog::accept();
    }

    QWidget* main_;
    Annotation* doc_;
    QLineEdit* title_;
    QPlainTextEdit* text_;
    QPlainTextEdit* preview_;
    QCheckBox* shot_tick_;
    QCheckBox* scene_tick_;
    QCheckBox* video_tick_;
    QWidget* video_box_;
    QComboBox* video_length_;
    QSpinBox* video_delay_;
    QPushButton* video_play_;
    QLabel* video_state_;
    // The last recording, in the app's folder until the report is opened.
    QString video_path_;
    QLabel* warning_;
    QLabel* short_note_;
    QPushButton* copy_full_;
    QPushButton* submit_;
    QWidget* picture_box_;
    QTabBar* tabs_;
    bool syncing_ = false;
    QPointer<LargeDrawing> large_;
    QSpinBox* delay_ = nullptr;
    int narrow_width_ = 620;
    QJsonObject report_;
};

QPointer<FeedbackDialog>& open_dialog() {
    static QPointer<FeedbackDialog> d;
    return d;
}

} // namespace

// The button, with a small arrow on its left that folds it away and brings it back. Remembered between sessions.
QWidget* make_feedback_button(QWidget* parent) {
    auto* box = new QWidget(parent);
    auto* row = new QHBoxLayout(box);
    row->setContentsMargins(0, 0, 0, 0);
    row->setSpacing(2);
    auto* fold = new QToolButton(box);
    fold->setObjectName("feelFold");
    fold->setAutoRaise(true);
    fold->setFocusPolicy(Qt::NoFocus);
    fold->setCursor(Qt::PointingHandCursor);
    fold->setFixedWidth(14);
    fold->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Expanding);
    auto* b = new QPushButton("This doesn't feel right!", box);
    b->setObjectName("feelButton");
    b->setFocusPolicy(Qt::NoFocus);
    b->setCursor(Qt::PointingHandCursor);
    b->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Fixed);
    b->setToolTip("Something behaves differently from what you expect? Tell us.\nWrite a report and open it as a GitHub issue. Nothing is public until you press Submit on GitHub.");
    QObject::connect(b, &QPushButton::clicked, b, [b] { open_feedback(b->window()); });
    row->addWidget(fold);
    row->addWidget(b);
    auto show = [fold, b](bool folded) {
        b->setVisible(!folded);
        fold->setText(folded ? QString::fromUtf8("\u25C2") : QString::fromUtf8("\u25B8"));
        fold->setToolTip(folded ? "Show the \"This doesn't feel right!\" button. It's also in the Help menu."
                                : "Fold the button away. It stays in the Help menu.");
    };
    show(QSettings().value("feedback/folded", false).toBool());
    QObject::connect(fold, &QToolButton::clicked, fold, [b, show] {
        const bool folded = b->isVisible();
        QSettings().setValue("feedback/folded", folded);
        show(folded);
    });
    return box;
}

void open_feedback(QWidget* main_window) {
    if (FeedbackDialog* d = open_dialog()) {
        d->show();
        d->raise();
        d->activateWindow();
        return;
    }
    // Before the window shows, so it isn't in the picture. Thrown away unless the box gets ticked.
    auto* d = new FeedbackDialog(main_window, capture_app(main_window));
    open_dialog() = d;
    d->show();
    d->raise();
    d->activateWindow();
}

} // namespace az

#include "feedback.moc"

#include "external.h"

#include <QCoreApplication>
#include <QDesktopServices>
#include <QDir>
#include <QObject>
#include <QProcess>
#include <QUrl>

namespace az {

QProcessEnvironment desktop_environment() {
    QProcessEnvironment env = QProcessEnvironment::systemEnvironment();
#if defined(Q_OS_LINUX)
    // AppRun's folder: the executable is in usr/bin under it.
    const QString here = QDir::cleanPath(QCoreApplication::applicationDirPath() + "/../..");
    const auto inside = [&](const QString& path) { return !path.isEmpty() && QDir::cleanPath(path).startsWith(here + "/"); };
    QStringList kept;
    for (const QString& p : env.value("LD_LIBRARY_PATH").split(':', Qt::SkipEmptyParts))
        if (!inside(p)) kept << p;
    if (kept.isEmpty()) env.remove("LD_LIBRARY_PATH");
    else env.insert("LD_LIBRARY_PATH", kept.join(':'));
    if (inside(env.value("PYTHONHOME"))) {
        env.remove("PYTHONHOME");
        env.remove("PYTHONNOUSERSITE");
    }
    for (const char* name : {"QT_PLUGIN_PATH", "QML2_IMPORT_PATH", "PYTHONPATH"})
        if (inside(env.value(name))) env.remove(name);
    // Set by main for Aztek's own windows only.
    if (env.value("AZTEK_QPA_SET") == "1") {
        env.remove("QT_QPA_PLATFORM");
        env.remove("AZTEK_QPA_SET");
    }
#endif
    return env;
}

#if defined(Q_OS_LINUX)
namespace {

// The desktop portal's OpenURI, for when xdg-open could not open a web link.
void open_through_portal(const QString& link, const QProcessEnvironment& env) {
    QProcess p;
    p.setProcessEnvironment(env);
    p.setProgram("gdbus");
    p.setArguments({"call", "--session", "--dest", "org.freedesktop.portal.Desktop", "--object-path", "/org/freedesktop/portal/desktop", "--method",
                    "org.freedesktop.portal.OpenURI.OpenURI", "", link, "{}"});
    p.startDetached();
}

} // namespace
#endif

bool open_external(const QUrl& url) {
#if defined(Q_OS_LINUX)
    const QString link = url.toString(QUrl::FullyEncoded);
    const QProcessEnvironment env = desktop_environment();
    const bool web = !url.isLocalFile();
    auto* p = new QProcess;
    p->setProcessEnvironment(env);
    p->setProgram("xdg-open");
    p->setArguments({link});
    QObject::connect(p, &QProcess::finished, p, [p, link, env, web](int code, QProcess::ExitStatus status) {
        if (web && (status != QProcess::NormalExit || code != 0)) open_through_portal(link, env);
        p->deleteLater();
    });
    QObject::connect(p, &QProcess::errorOccurred, p, [p, link, env, web](QProcess::ProcessError e) {
        if (e != QProcess::FailedToStart) return;
        if (web) open_through_portal(link, env);
        p->deleteLater();
    });
    p->start();
    return true;
#else
    return QDesktopServices::openUrl(url);
#endif
}

namespace {

class LinkHandler : public QObject {
    Q_OBJECT
public:
    using QObject::QObject;
public slots:
    void open(const QUrl& url) { open_external(url); }
};

} // namespace

void install_link_handler() {
#if defined(Q_OS_LINUX)
    auto* h = new LinkHandler(QCoreApplication::instance());
    for (const char* scheme : {"http", "https", "mailto"}) QDesktopServices::setUrlHandler(scheme, h, "open");
#endif
}

} // namespace az

#include "external.moc"

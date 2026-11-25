#include "examples.h"

#include <QCoreApplication>
#include <QDir>
#include <QDirIterator>
#include <QFile>
#include <QFileInfo>
#include <QStandardPaths>

#ifndef AZTEK_VERSION
#define AZTEK_VERSION "dev"
#endif

namespace az {

namespace {

bool has_scenes(const QString& where) {
    const QDir dir(where);
    return dir.exists() && !dir.entryList({"*.ma"}, QDir::Files).isEmpty();
}

// The copy in the program's resources (:/examples), written to the user's data folder. Written again when the version
// changes, file by file, so anything the user saved there stays.
QString unpacked() {
    const QString dir = QStandardPaths::writableLocation(QStandardPaths::AppLocalDataLocation) + "/examples";
    const QByteArray version = AZTEK_VERSION;
    QFile stamp(dir + "/.version");
    if (has_scenes(dir) && stamp.open(QIODevice::ReadOnly) && stamp.readAll() == version) return dir;
    stamp.close();

    const QString root = QStringLiteral(":/examples");
    bool wrote = false;
    QDirIterator it(root, QDir::Files, QDirIterator::Subdirectories);
    while (it.hasNext()) {
        const QString from = it.next();
        const QString to = dir + from.mid(root.size());
        if (!QDir().mkpath(QFileInfo(to).absolutePath())) return QString();
        QFile::remove(to);
        if (!QFile::copy(from, to)) return QString();
        // Files copied out of the resources are read-only.
        QFile::setPermissions(to, QFile::ReadOwner | QFile::WriteOwner | QFile::ReadGroup | QFile::ReadOther);
        wrote = true;
    }
    if (!wrote) return QString();
    if (stamp.open(QIODevice::WriteOnly | QIODevice::Truncate)) stamp.write(version);
    return has_scenes(dir) ? dir : QString();
}

} // namespace

QString examples_folder() {
    const QString base = QCoreApplication::applicationDirPath();
    for (const QString& where : {base + "/examples", base + "/../examples", base + "/../share/aztek/examples",
                                 base + "/../Resources/examples"})
        if (has_scenes(where)) return QDir(where).absolutePath();
    return unpacked();
}

} // namespace az

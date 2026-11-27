#include "userfolder.h"
#include "host.h"

#include <QButtonGroup>
#include <QCheckBox>
#include <QComboBox>
#include <QDialog>
#include <QDialogButtonBox>
#include <QDir>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonObject>
#include <QLabel>
#include <QPushButton>
#include <QRadioButton>
#include <QVBoxLayout>

namespace az {

void ask_user_folder_at_first_start(QWidget* parent, QWidget* splash) {
    const QJsonObject uf = ask("aztekUserFolder", QJsonObject{{"query", true}}).toObject();
    const QString maya = uf["detected"].toString();
    // Asked on the first start, and again whenever the folder in use is gone: Aztek's own deleted, a linked one moved.
    if (!uf["needsChoice"].toBool()) return;
    if (splash) splash->hide();
    const QString own = uf["ownFolder"].toString();
    const bool own_exists = uf["ownExists"].toBool();
    auto shown = [](const QString& p) { return QDir::toNativeSeparators(p); };

    QDialog d(parent);
    d.setWindowTitle("User Folder");
    auto* v = new QVBoxLayout(&d);
    // No existing user folder from the reference to take from: only whether to make Aztek's own.
    if (maya.isEmpty()) {
        auto* intro = new QLabel(QString("Aztek keeps your scripts, plug-ins, modules and preferences in <b>%1</b>, which isn't there yet.<br>Create it now?").arg(shown(own).toHtmlEscaped()), &d);
        intro->setWordWrap(true);
        v->addWidget(intro);
        auto* box = new QDialogButtonBox(&d);
        box->addButton("Create", QDialogButtonBox::AcceptRole)->setDefault(true);
        box->addButton("Ask Me Next Time", QDialogButtonBox::RejectRole);
        QObject::connect(box, &QDialogButtonBox::accepted, &d, &QDialog::accept);
        QObject::connect(box, &QDialogButtonBox::rejected, &d, &QDialog::reject);
        v->addWidget(box);
        d.setMinimumWidth(460);
        if (d.exec() != QDialog::Accepted) return;
        exec_quiet("aztekUserFolder", QJsonObject{{"mode", "aztek"}});
        log(QString("// User Folder: %1 created //").arg(shown(own)), 0);
        return;
    }
    const QString missing = uf["mode"].toString() != "aztek" && uf["chosen"].toBool()
                                ? QString("The folder Aztek was set to use, <b>%1</b>, isn't there any more.<br><br>").arg(shown(uf["folder"].toString()).toHtmlEscaped())
                                : (!own_exists ? QString("Aztek's own user folder, <b>%1</b>, isn't there %2.<br><br>").arg(shown(own).toHtmlEscaped(), uf["chosen"].toBool() ? "any more" : "yet") : QString());
    auto* intro = new QLabel(missing + QString("An existing user folder (from the reference) was found at <b>%1</b>. It holds scripts, plug-ins, modules, shelves and preferences.<br>How should Aztek use it?").arg(shown(maya).toHtmlEscaped()), &d);
    intro->setWordWrap(true);
    v->addWidget(intro);

    auto* group = new QButtonGroup(&d);
    auto option = [&](const QString& title, const QString& detail, int id) {
        auto* r = new QRadioButton(title, &d);
        group->addButton(r, id);
        v->addWidget(r);
        auto* l = new QLabel(detail, &d);
        l->setWordWrap(true);
        l->setIndent(22);
        l->setEnabled(false);
        v->addWidget(l);
        return r;
    };
    QRadioButton* copy = option(
        QString("Copy it into Aztek's own folder (%1)").arg(shown(own)),
        own_exists ? "What is missing there is added. Nothing already there is replaced, and the existing user folder is only read." : "The folder is made. The existing user folder is only read, and what you change in Aztek afterwards stays out of it.", 0);
    QRadioButton* link = option("Use the existing user folder (from the reference) directly (risky)", "Aztek reads its scripts, plug-ins, modules and preferences from it and saves its own settings elsewhere, but a script you run in Aztek can write there, into the folder the reference uses. What you add there for the reference shows up here.", 1);
    QRadioButton* none = option(own_exists ? QString("Keep Aztek's own folder as it is (%1)").arg(shown(own)) : QString("Create an empty folder (%1)").arg(shown(own)), "Nothing is taken from the existing user folder.", 2);
    // With a folder of Aztek's already there, it is left alone unless asked; with none, the copy is the start most want.
    (own_exists ? none : copy)->setChecked(true);

    auto* row = new QHBoxLayout();
    auto* version_label = new QLabel("Version:", &d);
    auto* version = new QComboBox(&d);
    version->addItem("Newest", QString());
    for (const QJsonValue& x : uf["detectedVersions"].toArray()) version->addItem(x.toString(), x.toString());
    version->setToolTip("The version's folder whose scripts, plug-ins, modules and preferences are taken.");
    row->addWidget(version_label);
    row->addWidget(version);
    row->addStretch(1);
    v->addLayout(row);
    auto* setup = new QCheckBox("Run its userSetup scripts at start", &d);
    setup->setChecked(true);
    setup->setToolTip("userSetup.mel and userSetup.py were written for the reference: one that asks for something Aztek hasn't got stops with an error.");
    v->addWidget(setup);
    auto sync = [&] {
        const bool uses = !none->isChecked();
        version_label->setEnabled(uses);
        version->setEnabled(uses);
        setup->setEnabled(uses);
    };
    QObject::connect(group, &QButtonGroup::idClicked, &d, [&](int) { sync(); });
    sync();

    auto* later = new QLabel("This can be changed at any time in Windows > Settings/Preferences > Preferences > User Folder.", &d);
    later->setWordWrap(true);
    v->addWidget(later);
    auto* box = new QDialogButtonBox(&d);
    box->addButton("OK", QDialogButtonBox::AcceptRole)->setDefault(true);
    box->addButton("Ask Me Next Time", QDialogButtonBox::RejectRole);
    QObject::connect(box, &QDialogButtonBox::accepted, &d, &QDialog::accept);
    QObject::connect(box, &QDialogButtonBox::rejected, &d, &QDialog::reject);
    v->addWidget(box);
    d.setMinimumWidth(520);
    // Closed or put off: nothing is saved, so the question comes back at the next start.
    if (d.exec() != QDialog::Accepted) return;

    const QString ver = version->currentData().toString();
    if (copy->isChecked()) {
        const QJsonObject r = exec_quiet("aztekUserFolder", QJsonObject{{"copyFrom", maya}, {"version", ver}}).toObject();
        if (!r.isEmpty()) log(QString("// User Folder: %1 files copied to %2, %3 already there and kept //").arg(r["copied"].toInt()).arg(shown(r["to"].toString())).arg(r["kept"].toInt()), 0);
        const QJsonArray failed = r["failed"].toArray();
        if (!failed.isEmpty()) log(QString("// Warning: User Folder: %1 files could not be copied, the first %2 //").arg(failed.size()).arg(failed.at(0).toString()), 4);
        exec_quiet("aztekUserFolder", QJsonObject{{"mode", "aztek"}, {"userSetup", setup->isChecked()}});
    } else if (link->isChecked()) {
        exec_quiet("aztekUserFolder", QJsonObject{{"mode", "link"}, {"folder", maya}, {"version", ver}, {"userSetup", setup->isChecked()}});
        log(QString("// User Folder: using %1 where it is //").arg(shown(maya)), 0);
    } else {
        exec_quiet("aztekUserFolder", QJsonObject{{"mode", "aztek"}});
    }
}

} // namespace az

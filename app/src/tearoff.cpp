#include "tearoff.h"

#include <QAction>
#include <QActionEvent>
#include <QApplication>
#include <QMenu>
#include <QMenuBar>

namespace az {

namespace {

void enable(QMenu* m) {
    if (!m || m->isTearOffEnabled()) return;
    m->setTearOffEnabled(true);
    for (QAction* a : m->actions()) enable(a->menu());
}

// Watches every widget for menus being added: to a menu bar, or to a menu that already tears off.
class Watcher : public QObject {
public:
    using QObject::QObject;
    bool eventFilter(QObject* o, QEvent* e) override {
        if (e->type() == QEvent::ActionAdded) {
            QAction* a = static_cast<QActionEvent*>(e)->action();
            if (a && a->menu()) {
                if (qobject_cast<QMenuBar*>(o)) enable(a->menu());
                else if (auto* m = qobject_cast<QMenu*>(o); m && m->isTearOffEnabled()) enable(a->menu());
            }
        }
        return false;
    }
};

} // namespace

void install_tear_offs(QApplication& app) { app.installEventFilter(new Watcher(&app)); }

} // namespace az

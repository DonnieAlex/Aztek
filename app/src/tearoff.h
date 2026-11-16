// Menus that tear off into floating windows, as the reference's do: every menu reached from a menu bar (the main
// window's, a panel's or an editor's) and their submenus get the dashed tear-off line at the top.
#pragma once

class QApplication;

namespace az {
void install_tear_offs(QApplication& app);
}

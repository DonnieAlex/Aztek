// Opening links and files in the desktop's own applications: the browser, the file manager, a video player.
#pragma once

#include <QProcessEnvironment>

class QUrl;

namespace az {

// The environment Aztek was started with, for programs it starts that are not its own. AppRun points the library path
// and Python's home at the libraries inside Aztek; a browser or file manager loading those fails to start.
QProcessEnvironment desktop_environment();

// Opens a link or a file in the desktop's application for it. True when something was started.
bool open_external(const QUrl& url);

// Sends every web link Qt opens (QDesktopServices::openUrl) through open_external. Called once, after the application
// is made.
void install_link_handler();

} // namespace az

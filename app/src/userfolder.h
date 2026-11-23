// The question asked at the first start when the reference's user folder is found: copy it, use it where it is, or
// start without it.
#pragma once

class QWidget;

namespace az {

// Asks before the user folder's start-up scripts run. If a choice is needed, hides the splash before showing the dialog.
void ask_user_folder_at_first_start(QWidget* parent, QWidget* splash);

} // namespace az

#pragma once

#include <QString>

namespace az {

// The example scenes and scripts: the folder that came with this copy of Aztek (beside the program, in the source tree
// when running from a build, where the Linux packages put it, inside Aztek.app on macOS) or, when there is none, the
// copy built into the program, written out once per version to the user's data folder. Empty only when that can't be
// written.
QString examples_folder();

} // namespace az

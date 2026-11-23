#!/usr/bin/env bash
# Usage: xvfb-run -a tools/ui_checks/first_launch.sh build/aztek
set -euo pipefail

work=$(mktemp -d)
app_pid=
trap 'if [[ -n $app_pid ]]; then kill "$app_pid" 2>/dev/null || true; wait "$app_pid" 2>/dev/null || true; fi; rm -rf -- "$work"' EXIT

export QT_QPA_PLATFORM=xcb AZTEK_PROFILE=1 AZTEK_PREFS="$work/prefs"
export XDG_CONFIG_HOME="$work/config" AZTEK_APP_DIR="$work/aztek"
export MAYA_APP_DIR="$work/maya"
mkdir "$MAYA_APP_DIR"
"$1" > "$work/app.log" 2>&1 &
app_pid=$!

dialog=$(timeout 15 xdotool search --sync --onlyvisible --name '^User Folder$')
# Give an already visible viewport time to paint inside the dialog's event loop.
sleep 1
if grep -q '^render ' "$work/app.log"; then
    echo 'The viewport painted while User Folder was open.' >&2
    exit 1
fi

xdotool windowfocus "$dialog" key Escape
for attempt in {1..100}; do
    if grep -q '^render persp ' "$work/app.log"; then
        test ! -e "$AZTEK_PREFS/userFolder.json"
        exit 0
    fi
    sleep 0.1
done
cat "$work/app.log" >&2
echo 'The viewport did not paint after User Folder closed.' >&2
exit 1

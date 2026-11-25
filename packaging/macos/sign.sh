#!/usr/bin/env bash
# Signs Aztek.app: packaging/macos/sign.sh <path/to/Aztek.app>
# With AZTEK_SIGN_IDENTITY set (a "Developer ID Application: ..." certificate in the keychain) it signs every piece of
# code inside, deepest first, with the hardened runtime and a secure timestamp, as notarization needs. Without it the
# signature is ad hoc, which runs on the Mac that built it but makes Gatekeeper ask everywhere else.
set -euo pipefail
APP=$1
[ -d "$APP" ] || { echo "no app at $APP" >&2; exit 1; }
HERE="$(cd "$(dirname "$0")" && pwd)"

if [ -z "${AZTEK_SIGN_IDENTITY:-}" ]; then
  codesign --force --deep --sign - "$APP"
  codesign --verify --deep --strict "$APP"
  echo "Signed $APP ad hoc"
  exit 0
fi

SIGN=(codesign --force --timestamp --options runtime --sign "$AZTEK_SIGN_IDENTITY")
[ -n "${AZTEK_SIGN_KEYCHAIN:-}" ] && SIGN+=(--keychain "$AZTEK_SIGN_KEYCHAIN")
MAIN="$APP/Contents/MacOS/$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$APP/Contents/Info.plist")"

# Deepest path first, so nothing is signed before what it contains.
by_depth() { awk -F/ '{ print NF "\t" $0 }' | sort -rn | cut -f2-; }

# Every Mach-O file but the main program: dylibs, Python and PySide6 modules, Qt plug-ins, harold.
find "$APP/Contents" -type f ! -path "$MAIN" -print0 | while IFS= read -r -d '' f; do
  if file -b "$f" | grep -q '^Mach-O'; then printf '%s\n' "$f"; fi
done | by_depth | while IFS= read -r f; do
  "${SIGN[@]}" "$f"
done

# Frameworks as bundles, then the app with its entitlements.
find "$APP/Contents" -type d -name '*.framework' | by_depth | while IFS= read -r f; do
  "${SIGN[@]}" "$f"
done
"${SIGN[@]}" --entitlements "$HERE/aztek.entitlements" "$APP"

codesign --verify --deep --strict --verbose=2 "$APP"
echo "Signed $APP as $AZTEK_SIGN_IDENTITY"
